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
    return {
        "children": map(to_result, node["children"]),
        "access_path": node.get("access_path", EMPTY_ACCESS_PATH),
        "metadata": node.get("metadata", {}),
        "ident": node.get("ident", EMPTY_IDENT),
        "parent": node.get("parent", {}).get("ident", EMPTY_IDENT),
    }


def traverse_tree(node: dict, collector) -> list[dict]:
    return collector(node) + _sum(
        list(map(lambda child: traverse_tree(child, collector), node.get("children", []))),
        []
    )


def flatten_tree(root: dict) -> list[dict]:
    return traverse_tree(root, lambda node: [node])


# def find_by_parent(self: dict, parent_ident: str):
#     pass
#
#
# def find_by_child(self: dict, child_ident: str):
#     pass


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
    return _deduplicate(matches)



# def find_by_access_path(self: dict, access_path_part: str):
#     pass
#
#
# def find_macro_attribute_by_names(self: dict, *idents: tuple[str, ...]):
#     pass


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


# def find_comparison_to_any(self: dict, ident: str):
#     pass
#
#
# def find_negative_of_operation(self: dict, operation_name: str, *args: tuple):
#     pass
#
#
# def find_functions_by_names(self: dict, *function_names: tuple[str, ...]):
#     pass
#
#
# def find_by_names(self: dict, *idents: tuple[str, ...]):
#     pass
#
#
# def find_method_calls(self: dict, caller: str, method: str):
#     pass
#
#
# def find_assignments(self: dict, ident: str, value_ident: str):
#     pass
#
#
# def find_mutables(self: dict):
#     pass
#
#
# def find_account_typed_nodes(self: dict, ident: str):
#     pass
#
#
# def find_member_accesses(self: dict, ident: str):
#     pass

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
        if ast.get("ident", False):
            metadata = {}
            if "mut" in ast:
                metadata["mut"] = ast["mut"]
            node = new_ast_node(ast, metadata, access_path)
            node["parent"] = parent
            nodes.append(node)
            parent = node
        elif ast.get("mut", False):
            metadata = {"mut": ast["mut"]}
            node = find_ident_src_node(ast, access_path, metadata)
            if node != EMPTY_NODE:
                ast_node_add_child(parent, node)
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


syn_ast = struct(
    EMPTY_NODE=EMPTY_NODE,
    new_ast_node=new_ast_node,
    to_result=to_result,
    traverse_tree=traverse_tree,
    flatten_tree=flatten_tree,
    find_chained_calls=find_chained_calls,
    find_by_similar_access_path=find_by_similar_access_path,
    find_comparisons=find_comparisons,
    first=first,
    prepare_ast=prepare_ast,
)
