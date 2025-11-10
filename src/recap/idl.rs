use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub(crate) struct Idl {
    pub(crate) name: Option<String>,
    pub(crate) version: Option<String>,
    #[serde(default)]
    pub(crate) instructions: Vec<IdlInstruction>,
    #[serde(default)]
    pub(crate) accounts: Vec<IdlStateAccount>,
    #[serde(default)]
    pub(crate) types: Vec<IdlTypeDef>,
    #[serde(default)]
    pub(crate) errors: Vec<IdlError>,
    pub(crate) metadata: Option<IdlMetadata>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct IdlInstruction {
    pub(crate) name: String,
    #[serde(default)]
    pub(crate) accounts: Vec<IdlAccountItem>,
    #[serde(default)]
    pub(crate) args: Vec<IdlArg>,
}

// Anchor 0.29 vs >0.30 flag names (see: https://solana.stackexchange.com/questions/13076/anchor-idl-different-incorrect-from-solana-playground-idl-generated)
#[derive(Debug, Deserialize)]
pub(crate) struct IdlAccountItem {
    pub(crate) name: String,
    #[serde(default)]
    pub(crate) isSigner: Option<bool>,
    #[serde(default)]
    pub(crate) signer: Option<bool>,
    #[serde(default)]
    pub(crate) isMut: Option<bool>,
    #[serde(default)]
    pub(crate) writable: Option<bool>,
    #[serde(default)]
    pub(crate) accounts: Vec<IdlAccountItem>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct IdlArg {
    pub(crate) name: String,
    #[serde(default)]
    pub(crate) r#type: serde_json::Value,
}
#[derive(Debug, Deserialize)]
pub(crate) struct IdlStateAccount {
    pub(crate) name: String,
    #[serde(default)]
    pub(crate) r#type: serde_json::Value,
}
#[derive(Debug, Deserialize)]
pub(crate) struct IdlTypeDef {
    pub(crate) name: String,
    #[serde(default)]
    pub(crate) r#type: serde_json::Value,
}
#[derive(Debug, Deserialize)]
pub(crate) struct IdlError {
    pub(crate) code: i64,
    pub(crate) name: String,
    #[serde(default)]
    pub(crate) msg: Option<String>,
}
#[derive(Debug, Deserialize)]
pub(crate) struct IdlMetadata {
    #[serde(default)]
    pub(crate) address: Option<String>,
}

pub(crate) fn load_idl(path: &Path) -> Result<Idl> {
    let raw = fs::read_to_string(path).with_context(|| format!("Reading IDL {}", path.display()))?;
    let idl: Idl = serde_json::from_str(&raw)
        .with_context(|| format!("Parsing IDL JSON {}", path.display()))?;
    Ok(idl)
}

pub(crate) fn flatten_accounts(items: &[IdlAccountItem], out: &mut Vec<(String, bool, bool)>) {
    for it in items {
        let is_signer = it.signer.unwrap_or_else(|| it.isSigner.unwrap_or(false));
        let is_writable = it.writable.unwrap_or_else(|| it.isMut.unwrap_or(false));
        if it.accounts.is_empty() {
            out.push((it.name.clone(), is_signer, is_writable));
        } else {
            flatten_accounts(&it.accounts, out);
        }
    }
}
