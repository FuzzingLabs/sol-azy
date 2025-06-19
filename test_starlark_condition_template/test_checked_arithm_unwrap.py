from test_ast import *

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


matches = find_checked_unwrap_patterns(AST6)

for m in matches:
    print(m)
