use crate::helpers::static_dir;
use crate::state::sast_state::SynAst;
use log::{error, info};
use serde::{Deserialize, Serialize};
use starlark::environment::{FrozenModule, Globals, GlobalsBuilder, LibraryExtension, Module};
use starlark::eval::{Evaluator, ReturnFileLoader};
use starlark::syntax::{AstModule, Dialect, DialectTypes};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StarlarkRuleType {
    Syn,
    Mir,
    LlvmIr,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarlarkRule {
    pub filename: String,
    pub content: String,
    pub rule_type: StarlarkRuleType,
}

pub type StarlarkRulesDir = Vec<StarlarkRule>;

pub trait StarlarkRuleDirExt
where
    Self: Sized,
{
    fn new_from_dir(rules_dir: &String) -> anyhow::Result<Self>;
}

impl StarlarkRuleDirExt for StarlarkRulesDir {
    fn new_from_dir(rules_dir: &String) -> anyhow::Result<Self> {
        let path = std::path::Path::new(rules_dir);

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

                info!(
                    "Loaded rule {} from directory {}",
                    filename,
                    rules_dir
                );

                Ok(StarlarkRule {
                    filename,
                    content,
                    rule_type,
                })
            })
            .collect::<Result<Vec<StarlarkRule>, anyhow::Error>>()
    }
}

#[derive(Debug, Clone)]
pub struct StarlarkEngine {
    pub dialect: Dialect,
    pub globals: Globals,
}

// TODO: Script header/footer
impl StarlarkEngine {
    pub fn new() -> Self {
        Self {
            dialect: Dialect {
                enable_types: DialectTypes::Enable,
                enable_f_strings: true,
                ..Dialect::Standard
            },
            // ? https://github.com/facebook/starlark-rust/blob/main/starlark/src/stdlib.rs#L131
            globals: GlobalsBuilder::extended_by(&[
                LibraryExtension::Json,         // ? To communicate with the Rust parts easily
                LibraryExtension::Map,          // ? For `map(lambda x: x * 2, [1, 2, 3, 4]) == [2, 4, 6, 8]`
                LibraryExtension::Filter,       // ? For `filter(lambda x: x > 2, [1, 2, 3, 4]) == [3, 4]`
                LibraryExtension::Typing,       // ? Type annotation and strict type checking
                LibraryExtension::StructType,   // ? For export in a pythonic way
                LibraryExtension::Print,        // ? Access to `print`
                LibraryExtension::SetType,      // ? Access to `set`
            ])
            .build(),
        }
    }

    fn wrap_syn_rule(code: String) -> String {
        format!(
            r#"# ! GENERATED
load("syn_ast.star", "syn_ast")
# ! GENERATED

{}

# ! GENERATED
def syn_rule_loader(ast: str) -> dict:
    return {{
        "matches": syn_ast_rule(syn_ast.prepare_ast(json.decode(ast)["items"])),
        "metadata": RULE_METADATA,
    }}


syn_rule_loader
# ! GENERATED
"#,
            code
        )
    }

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
            modules_owned.iter().map(|(k, v)| (k.clone(), v)).collect();

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
            &[heap.alloc(syn_serde::json::to_string(&syn_ast.ast))],
            &[],
        )
        .map(|v| v.to_json())
        .map_err(|e| e.into_anyhow())?
    }

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
            modules_owned.iter().map(|(k, v)| (k.clone(), v)).collect();

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

    fn load_modules<'a>(
        &self,
        starlark_ast: &'a AstModule,
    ) -> anyhow::Result<HashMap<&'a str, FrozenModule>> {
        let modules = starlark_ast
            .loads()
            .iter()
            .filter_map(|load| {
                let module_id = load.module_id.clone();
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
