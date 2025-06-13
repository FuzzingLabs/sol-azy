RULE_METADATA = {
    "version": "0.1.0",
    "author": "forefy",
    "name": "Missing Signer Check",
    "severity": "Low",
    "certainty": "Low",
    "description": "Signer checks verify whether an account owner has authorized the requested transaction. Failing to perform these checks might result in unintended operations executable by any account."
}

def syn_ast_rule(root: dict) -> list[dict]:
    matches = []
    for sink in syn_ast.find_chained_calls(root, "derive", "Accounts"):
        if sink.get("identifier") == "Accounts":
            continue
        if syn_ast.find_comparison_to_any(sink, "is_signer"):
            continue
        if syn_ast.find_member_accesses(sink, "is_signer"):
            continue
        matches.append(syn_ast.to_result(sink))
    return matches