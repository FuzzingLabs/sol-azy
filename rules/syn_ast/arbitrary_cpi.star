RULE_METADATA = {
    "version": "0.1.0",
    "author": "forefy",
    "name": "Arbitrary Cross-Program Invocation",
    "severity": "Medium",
    "certainty": "Medium",
    "description": "If not validated properly, when a program implements a Cross-Program Invocation, callers of the program may provide an arbitrary or untrusted program - manipulating the program to call instructions on an untrusted target program."
}
# ? Need to filter false positive
def syn_ast_rule(root: dict) -> list[dict]:
    matches = []
    for sink in syn_ast.find_chained_calls(root, "solana_program", "program", "invoke"):
        if sink.get("ident", "") in ["program", "invoke"]:
            continue
        if template_manager.is_matching_template(sink, "CHECK_SPLTOKEN_ID_CTX_ACCOUNT_AUTHORITY_KEY"):
            continue
        matches.append(syn_ast.to_result(sink))
    return matches