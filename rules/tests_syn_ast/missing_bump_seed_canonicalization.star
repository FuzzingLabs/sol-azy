RULE_METADATA = {
    "version": "0.1.0",
    "author": "forefy",
    "name": "Missing Bump Seed Canonicalization",
    "severity": "Medium",
    "certainty": "Low",
    "description": "If a program is deriving a bump seed for a Program Derived Address (PDA) without ensuring its uniqueness, it could lead to collisions or unexpected insecure logical occurrences. Check for usage of program_id which is not part of Pubkey::find_program_address and there is a usage of `bump`"
}

def syn_ast_rule(root: dict) -> list[dict]:
    matches = []
    for sink in syn_ast.find_member_accesses(root, "program_id"):
        if syn_ast.find_chained_calls(root, "Pubkey", "find_program_address"):
            continue
        if not syn_ast.find_member_accesses(root, "bump"):
            continue
        if not syn_ast.find_chained_calls(root, "Pubkey", "create_program_address"):
            continue
        matches.append(syn_ast.to_result(sink))
    return matches