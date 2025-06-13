RULE_METADATA = {
    "version": "0.1.0",
    "author": "forefy",
    "name": "Duplicate Mutable Accounts",
    "severity": "Medium",
    "certainty": "Medium",
    "description": "When there are two or more accounts with mutable data, a check must be in place to ensure mutation of each account is differentiated properly, to avoid unintended data modification of other accounts."
}

def syn_ast_rule(root: dict) -> list[dict]:
    matches = []
    for sink in syn_ast.find_chained_calls(root, "derive", "Accounts"):
        if sink.get("ident", "") == "Accounts":
            continue
        if syn_ast.find_macro_attribute_by_names(root, "constraint"):
            continue
        mutables = syn_ast.find_mutables(root)
        account_typed_mutables = []
        account_typed_mutables_tracking = set()
        for mutable in mutables:
            if not syn_ast.find_account_typed_nodes(root, mutable.get("ident", "")):
                account_typed_mutables = []
                break
            if syn_ast.find_method_calls(root, mutable.get("ident", ""), "key"):
                account_typed_mutables = []
                break
            if mutable.get("ident", "") not in account_typed_mutables_tracking:
                account_typed_mutables.append(mutable)
                account_typed_mutables_tracking.add(mutable.get("ident", ""))
        if len(account_typed_mutables) < 2:
            continue
        for mut in account_typed_mutables:
            matches.append(syn_ast.to_result(mut))
    return matches