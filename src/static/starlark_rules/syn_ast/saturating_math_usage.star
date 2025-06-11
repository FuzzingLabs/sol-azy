RULE_METADATA = {
    "version": "0.1.0",
    "author": "MohaFuzzingLabs",
    "name": "Saturating math operation usage",
    "severity": "Low",
    "certainty": "Low",
    "description": "The use of operations like saturating_add, saturating_mul, or saturating_sub in Rust is generally intended to prevent integer overflow and underflow, ensuring that the result remains within the valid range for the data type. However, in certain cases, relying on these functions alone can lead to inaccurate or unexpected results. This occurs when the application logic assumes that saturation alone guarantees accurate results, but ignores the potential loss of precision or accuracy."
}

SATURATING_FUNCTIONS = ["saturating_add", "saturating_mul", "saturating_sub", "saturating_add_signed", "saturating_sub_signed"]

def syn_ast_rule(root: dict) -> list[dict]:
    matches = []

    def saturating_collector(node: dict):
        if node.get("ident", "") in SATURATING_FUNCTIONS:
            matches.append(syn_ast.to_result(node))

    list(map(saturating_collector, syn_ast.flatten_tree(root)))
    return matches