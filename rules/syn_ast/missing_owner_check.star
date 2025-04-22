RULE_METADATA = {
    "version": "0.1.0",
    "author": "forefy",
    "name": "Missing Owner Check",
    "severity": "Low",
    "certainty": "Low",
    "description": "The Account struct includes an owner field indicating the key associated with that account's owner. This field should be used to ensure a caller of an owner-only intended functionality, is in fact the owner."
}

def syn_ast_rule(root: dict) -> list[dict]:
    matches = []
    for sink in syn_ast.find_chained_calls(root, "derive", "Accounts"):
        if sink.get("identifier") == "Accounts":
            continue
        if syn_ast.find_by_names(root, "SplTokenAccount"):
            continue
        if syn_ast.find_comparison_to_any(root, "owner") or syn_ast.find_member_accesses(root, "owner"):
            continue
        matches.append(syn_ast.to_result(sink))
    return matches