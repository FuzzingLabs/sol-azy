RULE_METADATA = {
    "version": "0.1.0",
    "author": "ectario",
    "name": "Raw unwrapping checked arithmetic",
    "severity": "Low",
    "certainty": "High",
    "description": (
        "Using `unwrap()` on checked arithmetic operations (e.g., `checked_add`, `checked_sub`, etc.) may cause the program to panic if the operation fails. In Solana programs, this leads to a full transaction revert without a clear error, which is not ideal. It's better to return explicit errors instead of unwrapping the result."
    )
}

def find_checked_unwrap_patterns(ast_nodes):
    matches = []
    checked_methods = {
        "checked_add": "add", "checked_sub": "sub", "checked_mul": "mul",
        "checked_div": "div", "checked_rem": "rem", "checked_pow": "pow",
        "checked_shl": "shl", "checked_shr": "shr"
    }
    
    stack = [ast_nodes]
    _str_node = str(ast_nodes)
    approx_max_nb_element = _str_node.count(",") + 1 + _str_node.count("[") + _str_node.count("{")

    for _ in range(approx_max_nb_element):
        if not stack:
            break

        node = stack[0]
        stack = stack[1:]
        
        if "children" in node and isinstance(node["children"], list):
            children_nodes = node["children"]
            for i, node_child in enumerate(children_nodes):
                if "children" in node_child and isinstance(node_child["children"], list):
                    stack.append(node_child)

                if isinstance(node_child, dict):
                    raw_node = node_child.get("raw_node")

                    if not "method" in raw_node or raw_node["method"] != "unwrap":
                        continue
                    if not "receiver" in raw_node or not "method_call" in raw_node["receiver"] or not "method" in raw_node["receiver"]["method_call"] or raw_node["receiver"]["method_call"]["method"] not in checked_methods:
                        continue

                    if len(children_nodes) <= i+1 or not "raw_node" in children_nodes[i+1]:
                        continue
                    
                    assignement_node = children_nodes[i+1]["raw_node"]
                    if not "ident" in assignement_node or not "ident" in assignement_node["ident"] or not "position" in assignement_node["ident"]:
                        continue
                    
                    matches.append((node_child, assignement_node["ident"]["position"]))
    return matches

def syn_ast_rule(root: dict) -> list[dict]:
    matches = []
    results = find_checked_unwrap_patterns(root)
    for r in results:
        matches.append(syn_ast.to_result(r[0], r[1]))
    return matches