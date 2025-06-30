use crate::engines::starlark_engine::{StarlarkEngine, StarlarkRuleDirExt, StarlarkRulesDir};
use crate::parsers::syn_ast::{AstPositions, SourcePosition};
use crate::printers::sast_printer::SastPrinter;
use anyhow::{Context, Result};
use log::{debug, error};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

/// Represents the severity level of a rule match in static analysis.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Severity {
    Unknown,
    Low,
    Medium,
    High,
    Critical,
}

/// Indicates how confident the engine is about a rule match.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Certainty {
    Unknown,
    Low,
    Medium,
    High,
}

/// Metadata describing a syntactic rule, including severity, certainty, and author info.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SynRuleMetadata {
    pub version: String,
    pub author: String,
    pub name: String,
    pub severity: Severity,
    pub certainty: Certainty,
    pub description: String,
}

impl SynRuleMetadata {
    /// Returns a default metadata instance used when no metadata is provided or parsing fails.
    pub fn default() -> Self {
        Self {
            version: "DEFAULT_RULE_VERSION".to_string(),
            author: "DEFAULT_RULE_AUTHOR".to_string(),
            name: "DEFAULT_RULE_NAME".to_string(),
            severity: Severity::Unknown,
            certainty: Certainty::Unknown,
            description: "DEFAULT_RULE_DESC".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// Represents a single match result from a syntactic rule evaluation.
///
/// This includes contextual metadata such as the identifier, access path,
/// parent node, and any nested matches.
pub struct SynMatchResult {
    pub children: Vec<SynMatchResult>,
    pub access_path: String,
    pub metadata: HashMap<String, serde_json::Value>,
    pub ident: String,
    pub parent: String,
}

/// Stores the result of evaluating a single syntactic rule against a file's AST.
///
/// Contains the original rule filename, raw JSON result string, match results,
/// and associated rule metadata.
impl SynMatchResult {
    pub fn get_location_metadata(&self) -> Result<SourcePosition> {
        let value = self
            .metadata
            .get("position")
            .ok_or_else(|| anyhow::anyhow!("No 'position' metadata found in matches"))?;

        if let serde_json::Value::Object(_obj) = value {
            match serde_json::from_value::<SourcePosition>(value.clone()) {
                Ok(position) => Ok(position),
                Err(err) => Err(anyhow::anyhow!(
                    "Failed to parse 'position' metadata: {}",
                    err
                )),
            }
        } else {
            Err(anyhow::anyhow!(
                "Unsupported type for 'position' metadata. Expected SourcePosition type."
            ))
        }
    }
}

/// Stores the result of evaluating a single syntactic rule against a file's AST.
///
/// Contains the original rule filename, raw JSON result string, match results,
/// and associated rule metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynAstResult {
    pub rule_filename: String,
    pub result: String,
    pub matches: Vec<SynMatchResult>,
    pub rule_metadata: SynRuleMetadata,
}

impl SynAstResult {
    /// Constructs a `SynAstResult` from a raw JSON evaluation output string.
    ///
    /// This function attempts to deserialize both `matches` and `metadata` fields
    /// from a JSON string returned by a rule engine.
    ///
    /// # Arguments
    ///
    /// * `rule_filename` - Name of the rule file that produced this result.
    /// * `result` - The raw JSON result returned by the rule engine.
    ///
    /// # Returns
    ///
    /// A parsed `SynAstResult` or an error if JSON deserialization fails.
    pub fn new_from_json(rule_filename: String, result: String) -> Result<Self> {
        let parsed: serde_json::Value = serde_json::from_str(&result)
            .with_context(|| format!("Failed to parse JSON result for rule: {}", rule_filename))?;

        let matches = match parsed.get("matches") {
            Some(matches_value) => match serde_json::from_value(matches_value.clone()) {
                Ok(matches) => matches,
                Err(err) => {
                    error!(
                        "Failed to deserialize matches for rule {}: {}",
                        rule_filename, err
                    );
                    return Err(anyhow::anyhow!("Failed to deserialize matches: {}", err));
                }
            },
            None => {
                error!("No 'matches' field found in rule result: {}", rule_filename);
                Vec::new()
            }
        };

        let rule_metadata = match parsed.get("metadata") {
            Some(metadata_value) => match serde_json::from_value(metadata_value.clone()) {
                Ok(metadata) => metadata,
                Err(err) => {
                    error!(
                        "Failed to deserialize metadata for rule {}: {}",
                        rule_filename, err
                    );
                    return Err(anyhow::anyhow!(
                        "Failed to deserialize rule metadata: {}",
                        err
                    ));
                }
            },
            None => {
                error!(
                    "No 'metadata' field found in rule result: {}",
                    rule_filename
                );
                SynRuleMetadata::default()
            }
        };

        Ok(Self {
            rule_filename,
            result,
            matches,
            rule_metadata,
        })
    }
}

/// Represents an enriched syntax tree (`syn::File`) along with AST positions
/// and a collection of results from rule evaluations.
#[derive(Clone)]
pub struct SynAst {
    #[allow(dead_code)]
    pub ast: syn::File,
    pub ast_positions: AstPositions,
    pub ast_json: serde_json::Value,
    pub results: Vec<SynAstResult>,
}

impl fmt::Debug for SynAst {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("SynAst")
            .field("ast", &"<syn::File AST omitted>")
            .field("enriched_ast", &self.ast_positions)
            .field("results", &self.results)
            .finish()
    }
}

impl SynAst {
    /// Applies all rules in a directory to this syntax tree using the provided engine.
    ///
    /// # Arguments
    ///
    /// * `rules_dir` - A directory of Starlark-based rule files.
    /// * `starlark_engine` - The engine used to evaluate rules.
    ///
    /// # Returns
    ///
    /// `true` if at least one rule was applied successfully, otherwise `false`.
    pub fn scan_ast(
        &mut self,
        rules_dir: &StarlarkRulesDir,
        starlark_engine: &StarlarkEngine,
    ) -> bool {
        rules_dir
            .iter()
            .map(|rule| {
                debug!("Applying rule {}", rule.filename);
                let res = match starlark_engine.eval_syn_rule(
                    rule.filename.as_str(),
                    rule.content.clone(),
                    self,
                ) {
                    Ok(res) => res,
                    Err(e) => {
                        error!("Failed to evaluate rule: {}", e);
                        return false;
                    }
                };
                match SynAstResult::new_from_json(rule.filename.clone(), res.clone()) {
                    Ok(result) => {
                        debug!("Matches num: {}", result.matches.len());
                        self.results.push(result);
                        true
                    }
                    Err(e) => {
                        error!("Failed to parse result: {}", e);
                        false
                    }
                }
            })
            .all(|res| res)
    }
}

/// A mapping of file paths to their parsed and enriched syntax trees (`SynAst`).
pub type SynAstMap = HashMap<String, SynAst>;

/// Provides extension methods on a `SynAstMap` for applying rules and accessing metadata.
pub trait SynAstMapExt {
    /// Applies all rules in the directory to each file's AST in the map.
    ///
    /// # Returns
    ///
    /// `Ok(true)` if at least one rule matched across all files, otherwise `Ok(false)` or an error.
    fn apply_rules(
        &mut self,
        rules_dir: &StarlarkRulesDir,
        starlark_engine: &StarlarkEngine,
    ) -> Result<bool>;
    /// Returns all file paths present in the syntax map.
    #[allow(dead_code)]
    fn get_file_paths(&self) -> Vec<&String>;
    /// Returns the number of syntax trees (files) in the map.
    fn count_files(&self) -> usize;
}

impl SynAstMapExt for SynAstMap {
    fn apply_rules(
        &mut self,
        rules_dir: &StarlarkRulesDir,
        starlark_engine: &StarlarkEngine,
    ) -> Result<bool> {
        let results = self
            .values_mut()
            .map(|syn_ast| syn_ast.scan_ast(rules_dir, starlark_engine))
            .collect::<Vec<bool>>();
        Ok(results.into_iter().any(|applied| applied))
    }

    fn get_file_paths(&self) -> Vec<&String> {
        self.keys().collect()
    }

    fn count_files(&self) -> usize {
        self.len()
    }
}

/// Represents the global state of a SAST session, including parsed syntax trees,
/// rule directory, and rule engine.
#[derive(Debug, Clone)]
pub struct SastState {
    pub syn_ast_map: SynAstMap,
    pub starlark_rules_dir: StarlarkRulesDir,
    pub starlark_engine: StarlarkEngine,
}

impl SastState {
    /// Initializes a new `SastState` by loading rules and preparing the engine.
    ///
    /// # Arguments
    ///
    /// * `syn_ast_map` - Map of all parsed source files to their AST representations.
    /// * `starlark_rules_dir_path` - Path to the directory containing rule files.
    ///
    /// # Returns
    ///
    /// A new `SastState` instance, or an error if the rule directory couldn't be parsed.
    pub fn new(
        syn_ast_map: SynAstMap,
        starlark_rules_dir_path: Option<String>,
        use_internal_rules: bool,
    ) -> Result<Self> {
        Ok(Self {
            syn_ast_map,
            starlark_rules_dir: StarlarkRulesDir::new_from_dir(
                starlark_rules_dir_path,
                use_internal_rules,
            )?,
            starlark_engine: StarlarkEngine::new(),
        })
    }

    /// Applies all loaded rules to the parsed syntax trees.
    ///
    /// # Returns
    ///
    /// A boolean indicating whether any rules were successfully applied.
    pub fn apply_rules(&mut self) -> Result<bool> {
        self.syn_ast_map
            .apply_rules(&self.starlark_rules_dir, &self.starlark_engine)
    }

    /// Delegates printing of the rule evaluation results to a printer component.
    ///
    /// # Returns
    ///
    /// `Ok(())` on success, or an error if the print operation fails.
    pub fn print_results(&self, scanned_dir: &String) -> Result<()> {
        SastPrinter::print_sast_state(self, scanned_dir)
    }
}
