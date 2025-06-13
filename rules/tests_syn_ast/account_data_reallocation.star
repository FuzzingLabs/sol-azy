RULE_METADATA = {
    "version": "0.1.0",
    "author": "MohaFuzzingLabs",
    "name": "Unsafe Account Data Reallocation",
    "severity": "Medium",
    "certainty": "Low",
    "description": "Improper memory management during reallocation can lead to memory corruption, uninitialized memory access, or exploitation of sensitive data left in uninitialized memory regions. This can result in security vulnerabilities including potential account takeovers or data leakage."
}

def syn_ast_rule(root: dict) -> list[dict]:
    matches = []
    for sink in syn_ast.find_chained_calls(root, "realloc"):
        if len(sink.get("args", [])) == 2 and sink.get("args", [])[1].get("lit", {}).get("bool", None) == False:
            matches.append(syn_ast.to_result(sink))
    return matches