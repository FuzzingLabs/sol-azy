RULE_METADATA = {
    "version": "0.1.0",
    "author": "forefy",
    "name": "Account Reinitialization",
    "severity": "Medium",
    "certainty": "Low",
    "description": "When account initialization is not properly validated against reinitialization attempts, callers of the program may try to reinitialize an existing account to manipulate its data and state."
}

def syn_ast_rule(root: dict) -> list[dict]:
    matches = []
    for sink in syn_ast.find_functions_by_names(root, "initialize", "init"):
        if syn_ast.first(syn_ast.find_by_names(sink, "authority")) == syn_ast.EMPTY_NODE:
            continue
        discriminator_impls = syn_ast.find_by_names(sink, "discriminator")
        if len(discriminator_impls) >= 2:
            continue
        matches.append(syn_ast.to_result(sink))
    return matches