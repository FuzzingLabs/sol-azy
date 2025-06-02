EMPTY_ACCESS_PATH = "EMPTY_ACCESS_PATH"
EMPTY_IDENT = "EMPTY_IDENT"
EMPTY_METADATA = {}
EMPTY_NODE = {
    "raw_node": {},
    "access_path": EMPTY_ACCESS_PATH,
    "metadata": EMPTY_METADATA,
    "children": [],
    "parent": {},
    "root": False,
    "args": []
}


def _sum(lists: list[list], start: list[dict]) -> list[dict]:
    # TODO: Investigate why need to filter, can be due to starlark-rust typing considering empty array as [None]
    return list(filter(lambda l: type(l) == 'dict', map(start.extend, lists))) or start


def _deduplicate(nodes: list[dict]) -> list[dict]:
    unique_items = []
    seen = set()

    for item in nodes:
        item_id = str(to_result(item))
        if item_id not in seen:
            seen.add(item_id)
            unique_items.append(item)

    return unique_items


def new_ast_node(syn_ast_node: dict, metadata: dict, access_path: str) -> dict:
    children = syn_ast_node.get("children", [])
    parent = syn_ast_node.get("parent", EMPTY_NODE)
    root = syn_ast_node.get("root", False)
    args = syn_ast_node.get("args", [])
    ident = syn_ast_node.get("ident", EMPTY_IDENT)
    # TODO: investigate the"ident": {"ident": "user", "mut": True} case
    if type(ident) == "dict":
        ident = ident.get("ident", EMPTY_IDENT)
    return {
        "raw_node": syn_ast_node,
        "access_path": access_path,
        "metadata": metadata,
        "children": children,
        "parent": parent,
        "root": root,
        "args": args,
        "ident": ident,
    }


def ast_node_add_child(node: dict, child: dict) -> dict:
    node["children"].append(child)
    child["parent"] = node
    return node


def ast_node_add_children(node: dict, children: list[dict]) -> dict:
    node["children"].extend(children)
    for child in children:
        child["parent"] = node
    return node


def to_result(node: dict) -> dict:
    metadata = node.get("metadata", {})

    if "position" not in metadata:
        parent = node.get("parent", {})
        if parent:
            parent_meta = parent.get("metadata", {})
            if "position" in parent_meta:
                metadata["position"] = parent_meta["position"]
            elif parent.get("parent"):
                parent_position = to_result(parent).get("metadata", {}).get("position")
                if parent_position:
                    metadata["position"] = parent_position

    return {
        "children": map(to_result, node["children"]),
        "access_path": node.get("access_path", EMPTY_ACCESS_PATH),
        "metadata": metadata,
        "ident": node.get("ident", EMPTY_IDENT),
        "parent": node.get("parent", {}).get("ident", EMPTY_IDENT),
    }

def filter_result(result: list[dict]) -> list[dict]:
    unique_items = []
    seen_positions = set()

    for item in result:
        position = str(item.get("metadata", {}).get("position", {}))
        if position and position not in seen_positions:
            seen_positions.add(position)
            unique_items.append(item)
        elif not position:
            unique_items.append(item)

    return unique_items


def traverse_tree(node: dict, collector) -> list[dict]:
    return collector(node) + _sum(
        list(map(lambda child: traverse_tree(child, collector), node.get("children", []))),
        []
    )


def flatten_tree(root: dict) -> list[dict]:
    return traverse_tree(root, lambda node: [node])


def find_by_child(self: dict, child_ident: str) -> list[dict]:
    matches = []

    def check_node(node: dict):
        children = node.get("children", [])
        if any(map(lambda n: n.get("ident", "") == child_ident, children)):
            matches.append(node)

    list(map(check_node, flatten_tree(self)))
    return matches


def find_chained_calls(self: dict, *idents: tuple[str, ...]) -> list[dict]:
    matches = []

    def check_node(node: dict):
        children = node.get("children", [])
        length = len(idents)

        def match_at_index(i):
            return all(map(lambda j: children[i + j].get("ident", "") == idents[j], range(length)))

        indices = filter(match_at_index, range(len(children) - length + 1))
        matches.extend(_sum(list(map(lambda i: children[i:i + length], indices)), []))

    list(map(check_node, flatten_tree(self)))
    return matches


def find_macro_attribute_by_names(self: dict, *idents: tuple[str, ...]) -> list[dict]:
    matches = []

    def check_node(node: dict):
        if node.get("ident", "") in idents and ".meta.list.tokens" in node.get("access_path", ""):
            matches.append(node)

    list(map(check_node, flatten_tree(self)))
    return matches


def find_by_similar_access_path(self: dict, access_path: str, stop_keyword: str) -> list[dict]:
    index = access_path.rfind(stop_keyword)
    truncated_path = access_path[: index + len(stop_keyword)] if index != -1 else access_path
    predicate = lambda node: (truncated_path in node.get("access_path", "")
                              and node.get("access_path", "") != truncated_path)
    return list(filter(predicate, flatten_tree(self)))


def find_comparisons(self: dict, ident1: str, ident2: str) -> list[dict]:
    nodes = flatten_tree(self)

    def find_node_by_access_path(access_path: str):
        filtered_nodes = list(filter(lambda n: access_path in n.get("access_path", ""), nodes))
        return filtered_nodes[0] if filtered_nodes else None

    def check_ident(node: dict, ident: str) -> bool:
        return any(filter(lambda n: n.get("ident", "") == ident, flatten_tree(node)))

    def check_conditions(left_node: dict, right_node: dict) -> bool:
        return (check_ident(left_node, ident1) and check_ident(right_node, ident2)) or \
            (check_ident(left_node, ident2) and check_ident(right_node, ident1))

    def process_node(node: dict):
        access_path = node.get("access_path", "")
        if "cond.binary.left" in access_path or "cond.binary.right" in access_path:
            truncated = access_path.split(".cond.binary")[0] + ".cond.binary"
            if "cond.binary.left" in access_path:
                right_path = truncated + ".right"
                right_node = find_node_by_access_path(right_path)
                if right_node and check_conditions(node, right_node):
                    return [node, right_node]
            elif "cond.binary.right" in access_path:
                left_path = truncated + ".left"
                left_node = find_node_by_access_path(left_path)
                if left_node and check_conditions(left_node, node):
                    return [left_node, node]
        return None

    comparisons = list(filter(None, map(process_node, nodes)))
    return comparisons


def find_comparison_to_any(self: dict, ident: str) -> list[dict]:
    matches = []

    def check_ident(node: dict, _ident: str) -> bool:
        return any(filter(lambda n: n.get("ident", "") == _ident, flatten_tree(node)))

    def check_node(node: dict):
        if "cond.binary.left" in node.get("access_path", "") or "cond.binary.right" in node.get("access_path",
                                                                                                "") or "cond.unary" in node.get(
                "access_path", ""):
            if check_ident(node, ident):
                matches.append(node)

    list(map(check_node, flatten_tree(self)))
    return matches


def find_functions_by_names(self: dict, *function_names: tuple[str, ...]) -> list[dict]:
    matches = []

    def check_node(node: dict):
        if node.get("ident", "") in function_names:
            matches.append(node)

    list(map(check_node, flatten_tree(self)))
    return matches


def find_by_names(self: dict, *idents: tuple[str, ...]) -> list[dict]:
    matches = []

    def check_node(node: dict):
        if node.get("ident", "") in idents:
            matches.append(node)

    list(map(check_node, flatten_tree(self)))
    return matches


def find_method_calls(self: dict, caller: str, method: str) -> list[dict]:
    matches = []

    def check_node(node: dict):
        if node.get("access_path", "").endswith("method_call") and node.get("ident", "") == method:
            if node.get("children", [{"ident": ""}])[0].get("ident", "") == caller:
                matches.append(node)
                return

    list(map(check_node, flatten_tree(self)))
    return matches


def find_assignments(self: dict, ident: str, value_ident: str) -> list[dict]:
    matches = []

    nodes = flatten_tree(self)

    def find_node_by_access_path(access_path: str):
        filtered_nodes = list(filter(lambda n: access_path in n.get("access_path", ""), nodes))
        return filtered_nodes[0] if filtered_nodes else None

    def check_conditions(left_node: dict, right_node: dict, _value_ident: str) -> bool:
        left_access_path = left_node.get("access_path", "").rsplit(".assign.left", 1)[0]
        right_access_path = right_node.get("access_path", "").rsplit(".assign.right", 1)[0]
        return (left_access_path == right_access_path
                and right_node.get("ident", "") == _value_ident)

    def check_node(node: dict):
        if node.get("ident", "") == ident and ".assign.left" in node.get("access_path", ""):
            assigment_path = node.get("access_path", "").split(".assign.left")[0] + ".assign"
            right_node = find_node_by_access_path(assigment_path + ".right")
            if right_node and check_conditions(node, right_node, value_ident):
                matches.append(node)
                return

    list(map(check_node, nodes))
    return matches


def find_mutables(self: dict) -> list[dict]:
    matches = []

    def check_node(node: dict):
        if node.get("metadata", {}).get("mut", False):
            matches.append(node)

    list(map(check_node, flatten_tree(self)))
    return matches


def find_account_typed_nodes(self: dict, ident: str) -> list[dict]:
    matches = []

    def ends_with_ty_path_segments(access_path: str) -> bool:
        parts = access_path.split(".")
        segments_index = -1
        for i in range(len(parts)):
            if parts[i] == "ty" and i + 1 < len(parts) and parts[i + 1] == "path":
                segments_index = i + 2
                break
        if segments_index == -1:
            return False
        remaining_parts = parts[segments_index:]
        is_segment = lambda part: part.startswith("segments[") or part == "segments"
        return all(map(is_segment, remaining_parts))

    def check_node(node: dict):
        if node.get("parent", EMPTY_NODE).get("ident", "") == ident and ends_with_ty_path_segments(
                node.get("access_path", "")):
            matches.append(node)

    list(map(check_node, flatten_tree(self)))
    return matches


def find_member_accesses(self: dict, ident: str) -> list[dict]:
    matches = []

    def check_node(node: dict):
        if node.get("ident", "") == ident and (
                "tokens" in node.get("access_path", "") or "call.args" in node.get("access_path", "")):
            matches.append(node)

    list(map(check_node, flatten_tree(self)))
    return matches


def first(nodes: list[dict]) -> dict:
    return nodes[0] if nodes else EMPTY_NODE


def find_ident_src_node(sub_data, sub_access_path: str, metadata: dict) -> dict:
    if type(sub_data) == "dict":
        if "ident" in sub_data:
            return new_ast_node(sub_data, metadata, sub_access_path)

        for key, value in sub_data.items():
            new_path = (
                f"{sub_access_path}.{key}" if sub_access_path else key
            )
            result = find_ident_src_node(value, new_path, metadata)
            if result:
                return result

    elif type(sub_data) == "list":
        for i, item in enumerate(sub_data):
            new_path = f"{sub_access_path}[{i}]"
            result = find_ident_src_node(item, new_path, metadata)
            if result:
                return result

    return EMPTY_NODE


def prepare_syn_ast(ast, access_path, parent) -> list[dict]:
    nodes = []

    if type(ast) == "list":
        for i, item in enumerate(ast):
            new_path = f"{access_path}[{i}]"
            nodes.extend(prepare_syn_ast(item, new_path, parent))
        return nodes

    if type(ast) == "dict":
        # ? Method node https://github.com/Auditware/radar/blob/main/api/utils/ast.py#L95
        if ast.get("method", False):
            metadata = {}
            if "position" in ast:
                metadata["position"] = ast["position"]
            if "mut" in ast:
                metadata["mut"] = ast["mut"]
            node = new_ast_node(ast, metadata, access_path)
            node["parent"] = parent
            node["ident"] = ast["method"]
            nodes.append(node)
            parent = node
        # ? Int node https://github.com/Auditware/radar/blob/main/api/utils/ast.py#L95
        elif ast.get("int", False):
            metadata = {}
            if "position" in ast:
                metadata["position"] = ast["position"]
            if "mut" in ast:
                metadata["mut"] = ast["mut"]
            node = new_ast_node(ast, metadata, access_path)
            node["parent"] = parent
            node["ident"] = str(ast["int"])
            nodes.append(node)
            parent = node
        elif ast.get("mut", False):
            metadata = {"mut": ast["mut"]}
            if "position" in ast:
                metadata["position"] = ast["position"]
            node = find_ident_src_node(ast, access_path, metadata)
            if node != EMPTY_NODE:
                ast_node_add_child(parent, node)
                nodes.append(node)
                parent = node
        # ? Ident node
        elif ast.get("ident", False):
            metadata = {}
            if "position" in ast:
                metadata["position"] = ast["position"]
            if "mut" in ast:
                metadata["mut"] = ast["mut"]
            node = new_ast_node(ast, metadata, access_path)
            node["parent"] = parent
            nodes.append(node)
            parent = node

        for key, value in ast.items():
            new_path = f"{access_path}.{key}" if access_path else key
            nodes.extend(prepare_syn_ast(value, new_path, parent))

    return nodes


def prepare_ast(ast: list[dict]) -> dict:
    nodes = prepare_syn_ast(ast, "", EMPTY_NODE)
    assigned_children = set()
    path_to_node = {}
    for node in nodes:
        path_to_node[node.get("access_path", EMPTY_ACCESS_PATH)] = node
    for node in nodes:
        if node.get("access_path", EMPTY_ACCESS_PATH) in assigned_children:
            continue
        parent_path = ".".join(node.get("access_path", "").split(".")[:-1])
        for _ in range(len(parent_path.split("."))):
            parent_node = path_to_node.get(parent_path, False)
            if parent_node:
                ast_node_add_child(parent_node, node)
                assigned_children.add(node.get("access_path", EMPTY_ACCESS_PATH))
                break
            parent_path = ".".join(parent_path.split(".")[:-1])
    root = new_ast_node({}, {}, "root")
    for node in nodes:
        if node.get("parent", EMPTY_NODE) == EMPTY_NODE:
            ast_node_add_child(root, node)
    return root


# AST Traversal Helpers
def walk_ast(node, visit_fn):
    pass


def filter_nodes_by_type(nodes, node_type):
    pass


def find_nodes(ast, condition_fn):
    pass


def map_nodes(ast, transform_fn):
    pass


# Entry Points and Public Functions
def find_sol_public_functions(ast):
    pass


# Mutability and Ownership
def find_mutable_parameters(function_node):
    pass


def find_immutable_parameters(function_node):
    pass


def find_mutable_variables(ast):
    pass


# Access Control and Authority Patterns
def find_require_calls(ast):
    pass


def find_assert_calls(ast):
    pass


def find_functions_with_attribute(ast, attribute_name):
    pass


# Data Structure Access
def find_struct_field_accesses(ast, struct_name=None, field_name=None):
    pass


def find_state_variable_access(ast, var_name=None):
    pass


def find_mapping_access(ast, mapping_name=None, key=None):
    pass


def find_function_arg_access(function_node, arg_name):
    pass


def find_array_access(ast, array_name=None):
    pass


# Token Transfers and Balance Manipulations
def find_sol_token_transfer_calls(ast):
    pass


def find_sol_token_balance_updates(ast):
    pass


def find_sol_token_mint_calls(ast):
    pass


def find_sol_token_burn_calls(ast):
    pass


# Solana serialization Patterns
def find_sol_borsh_serialize_calls(ast):
    pass


def find_sol_borsh_deserialize_calls(ast):
    pass


def find_sol_serde_serialize_calls(ast):
    pass


def find_sol_serde_deserialize_calls(ast):
    pass


syn_ast = struct(
    EMPTY_NODE=EMPTY_NODE,
    new_ast_node=new_ast_node,
    to_result=to_result,
    filter_result=filter_result,
    traverse_tree=traverse_tree,
    flatten_tree=flatten_tree,
    find_by_child=find_by_child,
    find_chained_calls=find_chained_calls,
    find_macro_attribute_by_names=find_macro_attribute_by_names,
    find_by_similar_access_path=find_by_similar_access_path,
    find_comparisons=find_comparisons,
    find_comparison_to_any=find_comparison_to_any,
    find_functions_by_names=find_functions_by_names,
    find_by_names=find_by_names,
    find_method_calls=find_method_calls,
    find_assignments=find_assignments,
    find_mutables=find_mutables,
    find_account_typed_nodes=find_account_typed_nodes,
    find_member_accesses=find_member_accesses,
    first=first,
    prepare_ast=prepare_ast,
)
