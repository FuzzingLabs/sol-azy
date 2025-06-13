RULE_METADATA = {
    "version": "0.1.0",
    "author": "forefy",
    "name": "Closing Accounts Insecurely",
    "severity": "Medium",
    "certainty": "Low",
    "description": "Closing accounts in Solana requires transferring the lamports remaining in the account. When lamports are zeroed, the Solana runtime eventually closes the account. Improperly setting the account for closure could cause account reinitialization type attacks."
}

def syn_ast_rule(root: dict) -> list[dict]:
    matches = []
    for sink in syn_ast.find_method_calls(root, "lamports", "borrow_mut"):
        if sink.get("ident", "") in ["program", "invoke"]:
            continue
        if syn_ast.find_comparisons(sink, "spl_token", "token_program") or syn_ast.find_comparisons(sink, "spl_token_2022", "token_program"):
            continue
        matches.append(syn_ast.to_result(sink))
    return matches