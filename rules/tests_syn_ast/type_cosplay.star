RULE_METADATA = {
    "version": "0.1.0",
    "author": "forefy",
    "name": "Type Cosplay",
    "severity": "Low",
    "certainty": "Low",
    "description": "When two account types can be deserialized with the exact same values, a malicious user could substitute between the account types, leading to unexpected execution and possible authorization bypass depending on how the data is used. Using try_from_slice does not check for the necessary discriminator."
}

def syn_ast_rule(root: dict) -> list[dict]:
    matches = []
    if not syn_ast.find_by_child(root, "AccountInfo"):
        return matches
    try_from_slice_calls = syn_ast.find_functions_by_names(root, "try_from_slice")
    if try_from_slice_calls:
        matches.append(syn_ast.to_result(syn_ast.first(try_from_slice_calls)))
    return matches