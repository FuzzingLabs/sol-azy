RULE_METADATA = {
    "version": "0.1.0",
    "author": "forefy",
    "name": "Unvalidated Sysvar Account",
    "severity": "Low",
    "certainty": "Low",
    "description": "Sysvars are special Solana accounts that provide access to dynamically updated data. When using them, it's important to verify the interaction is made to the real intended system variable account, and not to fraudulent or incorrect ones."
}

sysvars = ["clock", "epoch_schedule", "instructions", "rent"]

def syn_ast_rule(root: dict) -> list[dict]:
    matches = []
    if syn_ast.find_by_names(root, "Sysvar"):
        return matches
    for sysvar in sysvars:
        if syn_ast.find_chained_calls(root, "sysvar", sysvar, "ID"):
            continue
        for sink in syn_ast.find_chained_calls(root, sysvar, "key"):
            matches.append(syn_ast.to_result(sink))
    return matches