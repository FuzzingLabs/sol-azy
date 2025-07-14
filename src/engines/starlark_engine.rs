use crate::helpers::static_dir;
use crate::state::sast_state::SynAst;
use log::{error, info};
use serde::{Deserialize, Serialize};
use starlark::environment::{FrozenModule, Globals, GlobalsBuilder, LibraryExtension, Module};
use starlark::eval::{Evaluator, ReturnFileLoader};
use starlark::syntax::{AstModule, Dialect, DialectTypes};
use std::collections::HashMap;

/// Represents the type of input a Starlark rule operates on.
///
/// Supported types include:
/// - `Syn`: Abstract Syntax Tree (AST) WIP
/// - `Mir`: Mid-level Intermediate Representation (MIR) Not yet implemented
/// - `LlvmIr`: LLVM Intermediate Representation (LLVM IR) Not yet implemented
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StarlarkRuleType {
    Syn,
    Mir,
    LlvmIr,
}

/// A representation of a single loaded Starlark rule file.
///
/// This struct holds the filename, file content, and the type of the rule.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarlarkRule {
    pub filename: String,
    pub content: String,
    pub rule_type: StarlarkRuleType,
}

/// A collection of Starlark rules loaded from a directory.
pub type StarlarkRulesDir = Vec<StarlarkRule>;

/// A trait for loading Starlark rule files from a directory.
pub trait StarlarkRuleDirExt
where
    Self: Sized,
{
    /// Creates a new collection of rules from a specified directory.
    ///
    /// # Arguments
    ///
    /// * `rules_dir` - The path to the directory containing the rule files.
    /// * `use_internal_rules` - A boolean indicating whether to include built-in rules.
    fn new_from_dir(rules_dir: Option<String>, use_internal_rules: bool) -> anyhow::Result<Self>;
}

impl StarlarkRuleDirExt for StarlarkRulesDir {
    /// Loads all `.star` files from the specified directory and, if requested,
    /// includes the internal (built-in) rules.
    ///
    /// # Arguments
    ///
    /// * `rules_dir` - Path to the directory containing external Starlark `.star` rule files.
    /// * `use_internal_rules` - If `true`, loads the bundled internal rules.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of `StarlarkRule` objects on success, or an error
    /// if the directory is invalid or contains faulty files.
    fn new_from_dir(rules_dir: Option<String>, use_internal_rules: bool) -> anyhow::Result<Self> {
        let mut rules = Vec::new();

        if use_internal_rules {
            let internal_rules = load_internal_rules()?;
            rules.extend(internal_rules);
        }

        if let Some(dir_path) = rules_dir {
            let path = std::path::Path::new(&dir_path);
            validate_rules_directory(path, &dir_path)?;
            let external_rules = load_external_rules(path, &dir_path)?;
            rules.extend(external_rules);
        }

        Ok(rules)
    }
}

/// Validates that the specified path exists and is a directory.
///
/// # Arguments
///
/// * `path` - The `Path` object to validate.
/// * `rules_dir` - The original path string, used for error messages.
///
/// # Returns
///
/// An empty `Result` on success, or an error if validation fails.
fn validate_rules_directory(path: &std::path::Path, rules_dir: &String) -> anyhow::Result<()> {
    if !path.exists() {
        error!("Rules directory does not exist: {}", rules_dir);
        return Err(anyhow::anyhow!(
            "Rules directory does not exist: {}",
            rules_dir
        ));
    }

    if !path.is_dir() {
        error!("Path is not a directory: {}", rules_dir);
        return Err(anyhow::anyhow!("Path is not a directory: {}", rules_dir));
    }

    Ok(())
}

/// Loads internal Starlark rules from the embedded `starlark_rules` directory.
///
/// # Returns
///
/// A `Result` containing a vector of `StarlarkRule` objects, or an I/O error.
fn load_internal_rules() -> anyhow::Result<Vec<StarlarkRule>> {
    static_dir::read_all_files_in_dir("starlark_rules/syn_ast")?
        .into_iter()
        .filter(|(name, _)| name.ends_with(".star"))
        .map(|(name, content)| {
            let filename = name
                .split('/')
                .last()
                .ok_or_else(|| anyhow::anyhow!("Invalid internal rule path"))?
                .to_string();

            info!("Loaded internal rule {}", filename);

            Ok(StarlarkRule {
                filename,
                content,
                rule_type: StarlarkRuleType::Syn,
            })
        })
        .collect()
}

/// Loads external Starlark rules from a specified filesystem directory.
///
/// # Arguments
///
/// * `path` - The `Path` of the directory to read from.
/// * `rules_dir` - The original path string, for logging purposes.
///
/// # Returns
///
/// A `Result` containing a vector of `StarlarkRule` objects, or an I/O error.
fn load_external_rules(
    path: &std::path::Path,
    rules_dir: &String,
) -> anyhow::Result<Vec<StarlarkRule>> {
    std::fs::read_dir(path)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| {
            path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("star")
        })
        .map(|path| {
            let filename = path
                .file_name()
                .and_then(|name| name.to_str())
                .ok_or_else(|| anyhow::anyhow!("Invalid filename"))?
                .to_string();

            let content = std::fs::read_to_string(&path)?;

            // TODO: get rule_type
            let rule_type = StarlarkRuleType::Syn;

            info!("Loaded rule {} from directory {}", filename, rules_dir);

            Ok(StarlarkRule {
                filename,
                content,
                rule_type,
            })
        })
        .collect()
}

/// Provides an environment to evaluate Starlark rule files against parsed Rust ASTs.
///
/// The engine is configured with a dialect that supports f-strings and type annotations.
/// It also extends the environment with useful libraries for JSON handling, data manipulation,
/// and other common utilities.
#[derive(Debug, Clone)]
pub struct StarlarkEngine {
    pub dialect: Dialect,
    pub globals: Globals,
}

// TODO: Script header/footer
impl StarlarkEngine {
    /// Initializes a new Starlark evaluation engine with standard extensions enabled.
    ///
    /// This includes libraries for:
    /// - `Json`: For data interchange with Rust components.
    /// - `Map`, `Filter`, `SetType`: For data manipulation.
    /// - `Typing`: For type annotation and checking.
    /// - `StructType`: For creating structured data.
    /// - `Print`: For debugging.
    pub fn new() -> Self {
        Self {
            dialect: Dialect {
                enable_types: DialectTypes::Enable,
                enable_f_strings: true,
                ..Dialect::Standard
            },
            // ? https://github.com/facebook/starlark-rust/blob/main/starlark/src/stdlib.rs#L131
            globals: GlobalsBuilder::extended_by(&[
                LibraryExtension::Json,       // ? To communicate with the Rust parts easily
                LibraryExtension::Map, // ? For `map(lambda x: x * 2, [1, 2, 3, 4]) == [2, 4, 6, 8]`
                LibraryExtension::Filter, // ? For `filter(lambda x: x > 2, [1, 2, 3, 4]) == [3, 4]`
                LibraryExtension::Typing, // ? Type annotation and strict type checking
                LibraryExtension::StructType, // ? For export in a pythonic way
                LibraryExtension::Print, // ? Access to `print`
                LibraryExtension::SetType, // ? Access to `set`
            ])
            .build(),
        }
    }

    /// Wraps Starlark rule source code with a standard entry point.
    ///
    /// This function adds boilerplate to import necessary modules (`syn_ast`, `template_manager`)
    /// and defines a `syn_rule_loader` function that the engine calls to execute the rule.
    ///
    /// # Arguments
    ///
    /// * `code` - The raw source code of the Starlark rule.
    ///
    /// # Returns
    ///
    /// The wrapped source code as a `String`.
    fn wrap_syn_rule(code: String) -> String {
        format!(
            r#"# ! GENERATED
load("syn_ast.star", "syn_ast")
load("template_manager.star", "template_manager")
# ! GENERATED

{}

# ! GENERATED
def syn_rule_loader(ast: str) -> dict:
    return {{
        "matches": syn_ast.filter_result(syn_ast_rule(
            syn_ast.prepare_ast(json.decode(ast)["items"]),
            # json.decode(ast),
        )),
        "metadata": RULE_METADATA,
    }}


syn_rule_loader
# ! GENERATED
"#,
            code
        )
    }
    
    fn wrap_get_prepared_ast(code: String) -> String {
        format!(
            r#"# ! GENERATED
load("syn_ast.star", "syn_ast")
load("template_manager.star", "template_manager")
# ! GENERATED

{}

# ! GENERATED
def get_prepared_ast(ast: str) -> dict:
    return syn_ast.prepare_ast(json.decode(ast)["items"])


get_prepared_ast
# ! GENERATED
"#,
            code
        )
    }

    /// Evaluates a Starlark rule script against a `SynAst` structure.
    ///
    /// This method parses the rule, loads its dependencies, sets up an evaluator, and
    /// invokes the rule with the provided syntax tree.
    ///
    /// # Arguments
    ///
    /// * `filename` - The path or name of the rule file, used for diagnostics.
    /// * `code` - The source code of the Starlark rule.
    /// * `syn_ast` - A reference to the syntax tree structure to be analyzed.
    ///
    /// # Returns
    ///
    /// A `Result` containing a JSON string with the analysis results, or an error if evaluation fails.
    pub fn eval_syn_rule(
        &self,
        filename: &str,
        code: String,
        syn_ast: &SynAst,
    ) -> anyhow::Result<String> {
        let starlark_ast = AstModule::parse(filename, Self::wrap_syn_rule(code), &self.dialect)
            .map_err(|e| e.into_anyhow())?;

        let binding = starlark_ast.clone();
        let modules_owned = self.load_modules(&binding)?;

        let modules_ref: HashMap<&str, &FrozenModule> =
            modules_owned.iter().map(|(k, v)| (*k, v)).collect();

        let loader = ReturnFileLoader {
            modules: &modules_ref,
        };

        let module = Module::new();
        let mut eval = Evaluator::new(&module);
        eval.set_loader(&loader);

        let syn_rule = eval
            .eval_module(starlark_ast, &self.globals)
            .map_err(|e| e.into_anyhow())?;

        let heap = eval.heap();
        eval.eval_function(
            syn_rule,
            &[heap.alloc(serde_json::to_string(&syn_ast.ast_json).unwrap_or(String::new()))],
            // &[heap.alloc(serde_json::to_string(
            //     &starlark_syn_ast::prepare_ast(&syn_ast.ast_json)
            // ).unwrap_or(String::new()))],
            &[],
        )
        .map(|v| v.to_json())
        .map_err(|e| e.into_anyhow())?
    }

    /// Evaluates a Starlark script to get the prepared AST structure.
    ///
    /// This method parses the code, loads its dependencies, sets up an evaluator, and
    /// invokes the `get_prepared_ast` function with the provided syntax tree.
    ///
    /// # Arguments
    ///
    /// * `filename` - The path or name of the script file, used for diagnostics.
    /// * `code` - The source code of the Starlark script.
    /// * `syn_ast` - A reference to the syntax tree structure to be prepared.
    ///
    /// # Returns
    ///
    /// A `Result` containing a JSON string with the prepared AST, or an error if evaluation fails.
    pub fn eval_get_prepared_ast(
        &self,
        filename: &str,
        code: String,
        syn_ast: &SynAst,
    ) -> anyhow::Result<String> {
        let starlark_ast = AstModule::parse(filename, Self::wrap_get_prepared_ast(code), &self.dialect)
            .map_err(|e| e.into_anyhow())?;

        let binding = starlark_ast.clone();
        let modules_owned = self.load_modules(&binding)?;

        let modules_ref: HashMap<&str, &FrozenModule> =
            modules_owned.iter().map(|(k, v)| (*k, v)).collect();

        let loader = ReturnFileLoader {
            modules: &modules_ref,
        };

        let module = Module::new();
        let mut eval = Evaluator::new(&module);
        eval.set_loader(&loader);

        let get_prepared_ast_fn = eval
            .eval_module(starlark_ast, &self.globals)
            .map_err(|e| e.into_anyhow())?;

        let heap = eval.heap();
        eval.eval_function(
            get_prepared_ast_fn,
            &[heap.alloc(serde_json::to_string(&syn_ast.ast_json).unwrap_or(String::new()))],
            &[],
        )
            .map(|v| v.to_json())
            .map_err(|e| e.into_anyhow())?
    }


    /// Loads a Starlark module and freezes it, making its values immutable.
    ///
    /// This is used to load dependencies required by a rule.
    ///
    /// # Arguments
    ///
    /// * `filename` - The path to the Starlark module file within the embedded library.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `FrozenModule`, or an error if loading or freezing fails.
    fn load_frozen_module(&self, filename: &str) -> anyhow::Result<FrozenModule> {
        let code = match static_dir::read_file(filename) {
            Ok(code) => code,
            Err(e) => {
                error!("Failed to read Starlark module {}: {}", filename, e);
                return Err(e.into());
            }
        };
        let starlark_ast =
            match AstModule::parse(filename, code, &self.dialect).map_err(|e| e.into_anyhow()) {
                Ok(ast) => ast,
                Err(e) => {
                    error!("Failed to parse Starlark module {}: {}", filename, e);
                    return Err(e.into());
                }
            };

        let binding = starlark_ast.clone();
        let modules_owned = self.load_modules(&binding)?;

        let modules_ref: HashMap<&str, &FrozenModule> =
            modules_owned.iter().map(|(k, v)| (*k, v)).collect();

        let loader = ReturnFileLoader {
            modules: &modules_ref,
        };

        let module = Module::new();
        {
            let mut eval = Evaluator::new(&module);
            eval.set_loader(&loader);
            match eval.eval_module(starlark_ast, &self.globals) {
                Ok(module) => module,
                Err(e) => {
                    println!("{:?}", e);
                    error!("Failed to load Starlark module {}: {}", filename, e);
                    return Err(e.into_anyhow());
                }
            };
        }

        module.freeze().map_err(|e| e.into())
    }

    /// Loads all module dependencies specified in `load()` statements within a Starlark file.
    ///
    /// # Arguments
    ///
    /// * `starlark_ast` - A reference to the parsed AST of a Starlark script.
    ///
    /// # Returns
    ///
    /// A `Result` containing a map of module names to their corresponding `FrozenModule`,
    /// or an error if any module fails to load.
    fn load_modules<'a>(
        &self,
        starlark_ast: &'a AstModule,
    ) -> anyhow::Result<HashMap<&'a str, FrozenModule>> {
        let modules = starlark_ast
            .loads()
            .iter()
            .filter_map(|load| {
                let module_id = load.module_id;
                match self.load_frozen_module(&format!("starlark_libs/{}", module_id)) {
                    Ok(module) => Some((module_id, module)),
                    Err(e) => {
                        error!("Failed to load module {}: {}", load.module_id, e);
                        None
                    }
                }
            })
            .collect();
        Ok(modules)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::syn_ast::parse_rust_file;
    use std::collections::HashMap;
    use std::path::Path;

    #[test]
    fn test_syn_account_data_matching_star() {
        tracing_subscriber::fmt()
            .with_env_filter(tracing_subscriber::EnvFilter::new("sol_azy=debug"))
            .pretty()
            .init();

        let script_path = "rules/syn_ast/account_data_matching.star";
        let script_content =
            std::fs::read_to_string(script_path).expect("Failed to read the Starlark script.");

        let mut ast_map = HashMap::new();
        let program_path = "test_cases/base_anchor/programs/base_anchor/src/lib.rs";
        parse_rust_file(&Path::new(program_path), &mut ast_map).unwrap();

        let engine = StarlarkEngine::new();

        for (_, syn_ast) in ast_map.iter() {
            match engine.eval_syn_rule(&script_path.to_string(), script_content.clone(), syn_ast) {
                Ok(result) => {
                    assert!(!result.is_empty(), "The result should not be empty.");
                    println!("Evaluation successful with result: {}", result);
                    println!("Enriched AST: {:?}", syn_ast.ast_positions);
                }
                Err(e) => panic!("Evaluation failed: {}", e),
            }
        }
    }
}
