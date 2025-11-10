use std::collections::HashMap;

/// Lightweight regex-based source parsing for constraints/seeds.
#[derive(Debug, Clone)]
pub(crate) struct FieldMeta {
    pub(crate) name: String,
    pub(crate) has_address: bool,
    pub(crate) has_owner: bool,
    pub(crate) has_has_one: bool,
    pub(crate) has_constraint: bool,
    pub(crate) has_seeds: bool,
    pub(crate) has_spl: bool,
    // memory-related
    pub(crate) has_space: bool,
    pub(crate) has_realloc: bool,
    pub(crate) has_realloc_zero: bool,
}

pub(crate) type AccountsStructMap = HashMap<String, HashMap<String, FieldMeta>>;
pub(crate) type InstrToStructMap = HashMap<String, String>;

/// Match a public instruction function and capture:
/// 1) the function name
/// 2) the full inside of Context< ... > allowing one level of nested generics (e.g. Foo<'info>)
///
/// Notes:
/// - Allow optional generics right after fn name: fn name<'info>( ...
/// - Allow optional `mut` context variable
/// - The inner (?:[^<>]|<[^<>]*>)+ accepts one nesting level like T<'info>.
pub(crate) fn map_instruction_to_struct(src: &str) -> InstrToStructMap {
    use regex::Regex;

    let mut out = HashMap::new();

    let fun_re = Regex::new(
        r"pub\s+fn\s+([A-Za-z0-9_]+)\s*(?:<[^>]*>)?\s*\(\s*(?:&\s*)?(?:mut\s+)?(?:[A-Za-z_][A-Za-z0-9_]*)\s*:\s*Context\s*<\s*((?:[^<>]|<[^<>]*>)+)\s*>\s*,?"
    ).unwrap();

    // Split by top-level commas (commas not inside <...>), to safely get the last Context generic.
    fn split_top_level_commas(s: &str) -> Vec<&str> {
        let mut parts = Vec::new();
        let mut depth = 0i32;
        let mut start = 0usize;
        for (i, ch) in s.char_indices() {
            match ch {
                '<' => depth += 1,
                '>' => {
                    if depth > 0 {
                        depth -= 1;
                    }
                }
                ',' if depth == 0 => {
                    parts.push(s[start..i].trim());
                    start = i + 1;
                }
                _ => {}
            }
        }
        parts.push(s[start..].trim());
        parts
    }

    for m in fun_re.captures_iter(src) {
        let ix = m.get(1).unwrap().as_str().to_string();
        let inside_ctx = m.get(2).unwrap().as_str();
        // keep last generic argument to Context<...>
        let parts = split_top_level_commas(inside_ctx);
        let mut accounts_ty = parts.last().unwrap_or(&inside_ctx).trim().to_string();
        // transform `Foo<'info>` -> `Foo`
        if let Some(pos) = accounts_ty.find('<') {
            accounts_ty.truncate(pos);
        }
        // transform `crate::x::Foo` -> `Foo`
        if let Some(pos) = accounts_ty.rfind("::") {
            accounts_ty = accounts_ty[(pos + 2)..].to_string();
        }

        out.insert(ix, accounts_ty);
    }

    out
}

/// Extract #[derive(Accounts)] blocks and aggregate all #[account(...)] per field.
pub(crate) fn extract_accounts_structs(src: &str) -> AccountsStructMap {
    let mut map: AccountsStructMap = HashMap::new();

    // allow extra attributes (e.g. #[instruction(...)]) and comments between derive and struct.
    let struct_pat = concat!(
        r"#\s*\[\s*derive\s*\(\s*Accounts\s*\)\s*\]",
        r"(?:\s*#\s*\[[^\]]+\]\s*)*",             // extra attributes
        r"(?:\s*//[^\n]*\n|\s*///[^\n]*\n|\s*)*", // comments/space
        r"\s*pub\s+struct\s+([A-Za-z0-9_]+)(?:\s*<[^>]*>)?\s*",
        r"\{([\s\S]*?)\n?\}"
    );
    let struct_re = regex::RegexBuilder::new(struct_pat)
        .dot_matches_new_line(true)
        .build()
        .unwrap();

    // capture stacked #[account(...)] blocks; tolerate comments around; allow pub or pub(...)
    let field_pat = concat!(
        r"(?:\s*//[^\n]*\n|\s*///[^\n]*\n|\s*)*", // comments/space before
        r"(?P<attrs>(?:#\s*\[\s*account\s*\((?:[\s\S]*?)\)\s*\]\s*)+)",
        r"(?:\s*//[^\n]*\n|\s*///[^\n]*\n|\s*)*", // comments/space after
        r"(?:pub(?:\([^)]+\))?\s+)?",
        r"(?P<field>[A-Za-z0-9_]+)\s*:\s*[^,]+,\s*",
    );
    let field_re = regex::RegexBuilder::new(field_pat)
        .dot_matches_new_line(true)
        .build()
        .unwrap();

    //markers
    let address_re = regex::Regex::new(r"\baddress\s*=").unwrap();
    let owner_re = regex::Regex::new(r"(^|[^.])owner\s*=").unwrap();
    let has_one_re = regex::Regex::new(r"\bhas_one\s*=").unwrap();
    let constraint_re = regex::Regex::new(r"\bconstraint\s*=").unwrap();
    let seeds_re = regex::Regex::new(r"\bseeds\s*=\s*\[").unwrap();

    // spl markers
    let token_mint_re = regex::Regex::new(concat!(r"\btoken::mint\s*=")).unwrap();
    let token_authority_re = regex::Regex::new(concat!(r"\btoken::authority\s*=")).unwrap();
    let mint_authority_re = regex::Regex::new(concat!(r"\bmint::authority\s*=")).unwrap();
    let mint_decimals_re = regex::Regex::new(concat!(r"\bmint::decimals\s*=")).unwrap();
    let mint_freeze_re = regex::Regex::new(concat!(r"\bmint::freeze_authority\s*=")).unwrap();
    let assoc_mint_re = regex::Regex::new(concat!(r"\bassociated_token::mint\s*=")).unwrap();
    let assoc_authority_re =
        regex::Regex::new(concat!(r"\bassociated_token::authority\s*=")).unwrap();

    // memory markers
    let space_re = regex::Regex::new(r"\bspace\s*=").unwrap();
    let realloc_re = regex::Regex::new(r"\brealloc\b").unwrap();
    let realloc_zero_re = regex::Regex::new(r"realloc::zero\s*=").unwrap();

    for cap in struct_re.captures_iter(src) {
        let struct_name = cap.get(1).unwrap().as_str().to_string();
        let body = cap.get(2).unwrap().as_str();
        let mut fields: HashMap<String, FieldMeta> = HashMap::new();

        for f in field_re.captures_iter(body) {
            let attrs_chunk = f.name("attrs").unwrap().as_str();
            let fname = f.name("field").unwrap().as_str().to_string();

            // check spl constraints
            let has_token_mint = token_mint_re.is_match(attrs_chunk);
            let has_token_authority = token_authority_re.is_match(attrs_chunk);
            let has_mint_authority = mint_authority_re.is_match(attrs_chunk);
            let has_mint_decimals = mint_decimals_re.is_match(attrs_chunk);
            let has_mint_freeze_authority = mint_freeze_re.is_match(attrs_chunk);
            let has_assoc_mint = assoc_mint_re.is_match(attrs_chunk);
            let has_assoc_authority = assoc_authority_re.is_match(attrs_chunk);

            let meta = FieldMeta {
                name: fname.clone(),
                has_address: address_re.is_match(attrs_chunk),
                has_owner: owner_re.is_match(attrs_chunk),
                has_has_one: has_one_re.is_match(attrs_chunk),
                has_constraint: constraint_re.is_match(attrs_chunk),
                has_seeds: seeds_re.is_match(attrs_chunk),
                has_spl: has_token_mint
                    || has_token_authority
                    || has_mint_authority
                    || has_mint_decimals
                    || has_mint_freeze_authority
                    || has_assoc_mint
                    || has_assoc_authority,
                has_space: space_re.is_match(attrs_chunk),
                has_realloc: realloc_re.is_match(attrs_chunk),
                has_realloc_zero: realloc_zero_re.is_match(attrs_chunk),
            };

            fields.insert(fname, meta);
        }

        if !fields.is_empty() {
            map.insert(struct_name, fields);
        }
    }

    map
}

#[cfg(test)]
mod tests {
    use super::map_instruction_to_struct;

    #[test]
    fn maps_context_with_lifetimes_and_nested_generics() {
        let src = r#"
            #[program]
            pub mod challenge {
                pub fn initialize<'info>(
                    ctx: Context<'_, '_, '_, 'info, Initialize<'info>>,
                ) -> Result<()> { Ok(()) }

                pub fn set_fee(ctx: Context<SetFee>) -> Result<()> { Ok(()) }
                pub fn set_pause_state(ctx: Context<SetPauseState>) -> Result<()> { Ok(()) }

                pub fn register_mint<'info>(
                    ctx: Context<'_, '_, '_, 'info, RegisterMint<'info>>,
                ) -> Result<()> { Ok(()) }

                pub fn update_mint_config<'info>(
                    ctx: Context<'_, '_, '_, 'info, crate::path::UpdateMintConfig<'info>>,
                ) -> Result<()> { Ok(()) }

                pub fn bridge<'info>(
                    ctx: Context<'_, '_, '_, 'info, Bridge<'info>>,
                ) -> Result<()> { Ok(()) }

                pub fn release<'info>(
                    ctx: Context<'_, '_, '_, 'info, Release<'info>>,
                ) -> Result<()> { Ok(()) }

                pub fn withdraw_fees<'info>(
                    ctx: Context<'_, '_, '_, 'info, WithdrawFees<'info>>,
                ) -> Result<()> { Ok(()) }
            }
        "#;

        let got = map_instruction_to_struct(src);
        let mut keys: Vec<_> = got.keys().cloned().collect();
        keys.sort();
        assert_eq!(
            keys,
            vec![
                "bridge",
                "initialize",
                "register_mint",
                "release",
                "set_fee",
                "set_pause_state",
                "update_mint_config",
                "withdraw_fees",
            ]
        );
        assert_eq!(got["initialize"], "Initialize");
        assert_eq!(got["update_mint_config"], "UpdateMintConfig");
    }
}
