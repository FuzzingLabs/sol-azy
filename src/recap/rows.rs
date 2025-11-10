use std::collections::BTreeSet;
use std::path::Path;

use super::fs_utils::{read, walk};
use super::idl::{flatten_accounts, Idl};
use super::parser::{extract_accounts_structs, map_instruction_to_struct, AccountsStructMap};

#[derive(Debug)]
pub(crate) struct Row {
    pub(crate) instruction: String,
    pub(crate) signers: Vec<String>,
    pub(crate) writables: Vec<String>,
    pub(crate) constrained: Vec<String>, // "field(marker,...)" where marker in {address,has_one,constraint,spl}
    pub(crate) seeded: Vec<String>,      // field names with seeds=[...]
    pub(crate) memory: Vec<String>,      // memory management (realloc, realloc::zero, space)
}

pub(crate) fn build_rows_for_program(idl: &Idl, crate_root: &Path) -> Vec<Row> {
    let src_dir = crate_root.join("src");
    let rs_files = walk(&src_dir)
        .into_iter()
        .filter(|p| p.extension().map(|e| e == "rs").unwrap_or(false))
        .collect::<Vec<_>>();
    let merged_src = rs_files
        .iter()
        .map(|p| read(p))
        .collect::<Vec<_>>()
        .join("\n/*--file--*/\n");

    let instr_to_struct = map_instruction_to_struct(&merged_src);
    let structs: AccountsStructMap = extract_accounts_structs(&merged_src);

    let mut rows = vec![];

    for ix in &idl.instructions {
        let mut flat = vec![];
        flatten_accounts(&ix.accounts, &mut flat);
        let mut signers = BTreeSet::new();
        let mut writables = BTreeSet::new();
        for (name, is_signer, is_writable) in flat {
            if is_signer {
                signers.insert(name.clone());
            }
            if is_writable {
                writables.insert(name.clone());
            }
        }

        let mut constrained = BTreeSet::new();
        let mut seeded = BTreeSet::new();
        let mut memory = BTreeSet::new();

        if let Some(struct_name) = instr_to_struct.get(&ix.name) {
            if let Some(fields) = structs.get(struct_name) {
                for (field_name, meta) in fields {
                    if !idl_account_present(idl, &ix.name, field_name) {
                        continue;
                    }

                    let mut tags = vec![];
                    if meta.has_address {
                        tags.push("address");
                    }
                    if meta.has_owner {
                        tags.push("owner");
                    }
                    if meta.has_has_one {
                        tags.push("has_one");
                    }
                    if meta.has_constraint {
                        tags.push("constraint");
                    }
                    if meta.has_spl {
                        tags.push("spl");
                    }

                    if !tags.is_empty() {
                        constrained.insert(format!("{}({})", field_name, tags.join(",")));
                    }

                    if meta.has_seeds {
                        seeded.insert(field_name.clone());
                    }

                    let mut mt = Vec::new();
                    if meta.has_space {
                        mt.push("space");
                    }
                    if meta.has_realloc_zero {
                        mt.push("realloc::zero");
                    } else if meta.has_realloc {
                        mt.push("realloc");
                    }

                    if !mt.is_empty() {
                        memory.insert(format!("{}({})", field_name, mt.join(",")));
                    }
                }
            }
        }

        rows.push(Row {
            instruction: ix.name.clone(),
            signers: signers.into_iter().collect(),
            writables: writables.into_iter().collect(),
            constrained: constrained.into_iter().collect(),
            seeded: seeded.into_iter().collect(),
            memory: memory.into_iter().collect(),
        });
    }

    rows
}

fn idl_account_present(idl: &Idl, ix_name: &str, field_name: &str) -> bool {
    idl.instructions
        .iter()
        .find(|ix| ix.name == ix_name)
        .map(|ix| {
            let mut flat = vec![];
            flatten_accounts(&ix.accounts, &mut flat);
            flat.into_iter().any(|(n, _, _)| n == field_name)
        })
        .unwrap_or(false)
}
