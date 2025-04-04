use crate::engines::starlark_engine::{StarlarkEngine, StarlarkRuleDirExt, StarlarkRulesDir};
use anyhow::{Context, Result};
use log::{debug, error};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Severity {
    Unknown,
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Certainty {
    Unknown,
    Low,
    Medium,
    High,
}

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
pub struct SynMatchResult {
    pub children: Vec<SynMatchResult>,
    pub access_path: String,
    pub metadata: HashMap<String, String>,
    pub ident: String,
    pub parent: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynAstResult {
    pub rule_filename: String,
    pub result: String,
    pub matches: Vec<SynMatchResult>,
    pub rule_metadata: SynRuleMetadata,
}

impl SynAstResult {
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

#[derive(Clone)]
pub struct SynAst {
    pub ast: syn::File,
    pub enriched_ast: HashMap<String, serde_json::Value>,
    pub results: Vec<SynAstResult>,
}

impl fmt::Debug for SynAst {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("SynAst")
            .field("ast", &"<syn::File AST omitted>")
            .field("enriched_ast", &self.enriched_ast)
            .field("results", &self.results)
            .finish()
    }
}

impl SynAst {
    pub fn scan_ast(&mut self, rules_dir: &StarlarkRulesDir, starlark_engine: &StarlarkEngine) -> bool {
        rules_dir
            .iter()
            .map(|rule| {
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
                        self.results.push(result);
                        true
                    }
                    Err(e) => {
                        error!("Failed to parse result: {}", e);
                        false
                    }
                }
            })
            .any(|applied| applied)
    }
}

pub type SynAstMap = HashMap<String, SynAst>;

pub trait SynAstMapExt {
    fn apply_rules(&mut self, rules_dir: &StarlarkRulesDir, starlark_engine: &StarlarkEngine) -> Result<bool>;
    fn get_file_paths(&self) -> Vec<&String>;
    fn count_files(&self) -> usize;
}

impl SynAstMapExt for SynAstMap {
    fn apply_rules(&mut self, rules_dir: &StarlarkRulesDir, starlark_engine: &StarlarkEngine) -> Result<bool> {
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

#[derive(Debug, Clone)]
pub struct SastState {
    pub syn_ast_map: SynAstMap,
    pub starlark_rules_dir: StarlarkRulesDir,
    pub starlark_engine: StarlarkEngine,
}

impl SastState {
    pub fn new(syn_ast_map: SynAstMap, starlark_rules_dir_path: &String) -> Result<Self> {
        Ok(Self {
            syn_ast_map,
            starlark_rules_dir: StarlarkRulesDir::new_from_dir(starlark_rules_dir_path)?,
            starlark_engine: StarlarkEngine::new(),
        })
    }

    pub fn apply_rules(&mut self) -> Result<bool> {
        self.syn_ast_map.apply_rules(&self.starlark_rules_dir, &self.starlark_engine)
    }
}
