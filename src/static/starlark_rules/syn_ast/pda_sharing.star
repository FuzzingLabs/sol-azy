RULE_METADATA = {
    "version": "0.1.0",
    "author": "forefy",
    "name": "PDA Sharing",
    "severity": "Low",
    "certainty": "Low",
    "description": "Reuse of a PDA across multiple authority domains can lead to unauthorized data or funds access."
}

def syn_ast_rule(root: dict) -> list[dict]:
    matches = []
    if syn_ast.find_macro_attribute_by_names(root, "seeds", "bump"):
        return matches
    for sink in syn_ast.find_chained_calls(root, "token", "transfer"):
        if sink.get("identifier") == "transfer":
            continue
        matches.append(syn_ast.to_result(sink))
    return matches