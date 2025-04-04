RULE_METADATA = {
    "version": "0.1.0",
    "author": "forefy",
    "name": "Account Data Matching",
    "severity": "Low",
    "certainty": "Low",
    "description": "Unpacking account structures without verifying authorization might allow an attacker to view or modify account data unintentionally. Therefore, account unpack operations should be accompanied by appropriate ownership verification."
}


def syn_ast_rule(root: dict) -> list[dict]:
    matches = []
    for sink in syn_ast.find_chained_calls(root, "SplTokenAccount", "unpack"):
        assigned_var = syn_ast.first(syn_ast.find_by_similar_access_path(
            root,
            sink.get("access_path"),
            "stmts"
        ))
        if assigned_var == syn_ast.EMPTY_NODE:
            continue
        if syn_ast.find_comparisons(sink, assigned_var.get("ident"), "authority"):
            continue
        matches.append(syn_ast.to_result(assigned_var))
    return matches