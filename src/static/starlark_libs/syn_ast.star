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
    """
    Combines multiple lists into a single list, filtering out None values.

    Args:
        lists: List of lists to combine
        start: Starting list to extend with items from other lists

    Returns:
        Combined list with None values filtered out
    """
    # TODO: Investigate why need to filter, can be due to starlark-rust typing considering empty array as [None]
    return list(filter(lambda l: type(l) == 'dict', map(start.extend, lists))) or start


def _deduplicate(nodes: list[dict]) -> list[dict]:
    """
    Removes duplicate nodes from a list based on their string representation.

    Args:
        nodes: List of nodes to deduplicate

    Returns:
        List with duplicate nodes removed
    """
    unique_items = []
    seen = set()

    for item in nodes:
        item_id = str(to_result(item))
        if item_id not in seen:
            seen.add(item_id)
            unique_items.append(item)

    return unique_items


def new_ast_node(syn_ast_node: dict, metadata: dict, access_path: str) -> dict:
    """
    Creates a new AST node with the given syntax node data, metadata, and access path.

    Args:
        syn_ast_node: Raw syntax AST node data
        metadata: Metadata dictionary to attach to the node
        access_path: String representing the path to this node in the AST

    Returns:
        New AST node dictionary with all required fields
    """
    children = syn_ast_node.get("children", [])
    parent = syn_ast_node.get("parent", EMPTY_NODE)
    if parent != EMPTY_NODE:
        parent_ident = parent.get("ident", EMPTY_IDENT)
        parent_metadata = parent.get("metadata", {})
        parent_root = parent.get("root", False)
        parent_access_path = parent.get("access_path", "")
        parent_args = parent.get("args", [])
        parent = {
            "raw_node": {"ident": parent_ident},
            "access_path": parent_access_path,
            "metadata": parent_metadata,
            "children": [],
            "parent": EMPTY_NODE,
            "root": parent_root,
            "args": parent_args
        }

    root = syn_ast_node.get("root", False)
    args = syn_ast_node.get("args", [])
    ident = syn_ast_node.get("ident", EMPTY_IDENT)
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
    """
    Adds a child node to a parent node's children list.
    Sets the child's parent to EMPTY_NODE to avoid circular references.

    Args:
        node: Parent node to add child to
        child: Child node to be added

    Returns:
        The parent node with the child added
    """
    if node == EMPTY_NODE:
        return node
    node["children"].append(child)
    child["parent"] = EMPTY_NODE  # Just set to EMPTY_NODE to avoid cycles
    return node


def to_result(node: dict, position = {}) -> dict:
    """
    Converts an AST node to a result format suitable for output.

    Args:
        node: AST node to convert

    Returns:
        Dictionary containing the node's essential information in result format
    """
    metadata = node.get("metadata", {})

    children = []
    if "children" in node:
        children = map(to_result, node["children"])

    if position != {}:
        metadata["position"] = position

    return {
        "children": children,
        "access_path": node.get("access_path", EMPTY_ACCESS_PATH),
        "metadata": metadata,
        "ident": node.get("ident", EMPTY_IDENT),
        "parent": node.get("parent", {}).get("ident", EMPTY_IDENT),
    }



def filter_result(result: list[dict]) -> list[dict]:
    """
    Filters a result list to remove duplicates based on metadata position.
    Items without position metadata are always included.

    Args:
        result: List of result nodes to filter

    Returns:
        Filtered list with position-based duplicates removed
    """
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
    """
    Traverses an AST tree and applies a collector function to each node.

    Args:
        node: Root node to start traversal from
        collector: Function to apply to each node during traversal

    Returns:
        List of collected results from all nodes
    """
    return collector(node) + _sum(
        list(map(lambda child: traverse_tree(child, collector), node.get("children", []))),
        []
    )


def flatten_tree(root: dict) -> list[dict]:
    """
    Flattens an AST tree into a list of all nodes.

    Args:
        root: Root node of the tree to flatten

    Returns:
        List containing all nodes in the tree
    """
    return traverse_tree(root, lambda node: [node])


def find_by_child(self: dict, child_ident: str) -> list[dict]:
    """
    Finds all nodes that have a direct child with the specified identifier.

    Args:
        self: Root node to search from
        child_ident: Identifier of the child to look for

    Returns:
        List of nodes that have a child with the given identifier
    """
    matches = []

    def check_node(node: dict):
        children = node.get("children", [])
        if any(map(lambda n: n.get("ident", "") == child_ident, children)):
            matches.append(node)

    list(map(check_node, flatten_tree(self)))
    return matches


def find_chained_calls(self: dict, *idents: tuple[str, ...]) -> list[dict]:
    """
    Finds sequences of chained method calls with the specified identifiers.

    Args:
        self: Root node to search from
        *idents: Sequence of identifiers to match in order

    Returns:
        List of nodes representing the chained calls
    """
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
    """
    Finds macro attributes by their names in the AST.

    Args:
        self: Root node to search from
        *idents: Identifiers of macro attributes to find

    Returns:
        List of nodes representing macro attributes with the given names
    """
    matches = []

    def check_node(node: dict):
        if node.get("ident", "") in idents and ".meta.list.tokens" in node.get("access_path", ""):
            matches.append(node)

    list(map(check_node, flatten_tree(self)))
    return matches


def find_by_similar_access_path(self: dict, access_path: str, stop_keyword: str) -> list[dict]:
    """
    Finds nodes with access paths similar to the given path, truncated at the stop keyword.

    Args:
        self: Root node to search from
        access_path: Access path to match against
        stop_keyword: Keyword to truncate the path at

    Returns:
        List of nodes with similar access paths
    """
    index = access_path.rfind(stop_keyword)
    truncated_path = access_path[: index + len(stop_keyword)] if index != -1 else access_path
    predicate = lambda node: (truncated_path in node.get("access_path", "")
                              and node.get("access_path", "") != truncated_path)
    return list(filter(predicate, flatten_tree(self)))


def find_comparisons(self: dict, ident1: str, ident2: str) -> list:
    """
    Finds binary comparison operations between two specific identifiers.

    Args:
        self: Root node to search from
        ident1: First identifier to compare
        ident2: Second identifier to compare

    Returns:
        List of comparison pairs [left_node, right_node]
    """
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
    """
    Finds any comparison operations (binary or unary) involving the specified identifier.

    Args:
        self: Root node to search from
        ident: Identifier to find in comparisons

    Returns:
        List of nodes representing comparisons involving the identifier
    """
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
    """
    Finds function declarations by their names.

    Args:
        self: Root node to search from
        *function_names: Names of functions to find

    Returns:
        List of nodes representing functions with the given names
    """
    matches = []

    def check_node(node: dict):
        if node.get("ident", "") in function_names:
            matches.append(node)

    list(map(check_node, flatten_tree(self)))
    return matches


def find_by_names(self: dict, *idents: tuple[str, ...]) -> list[dict]:
    """
    Finds nodes by their identifiers.

    Args:
        self: Root node to search from
        *idents: Identifiers to search for

    Returns:
        List of nodes with matching identifiers
    """
    matches = []

    def check_node(node: dict):
        if node.get("ident", "") in idents:
            matches.append(node)

    list(map(check_node, flatten_tree(self)))
    return matches


def find_method_calls(self: dict, caller: str, method: str) -> list[dict]:
    """
    Finds method call expressions with a specific caller and method name.

    Args:
        self: Root node to search from
        caller: Identifier of the object calling the method
        method: Name of the method being called

    Returns:
        List of nodes representing method calls matching the criteria
    """
    matches = []

    def check_node(node: dict):
        if node.get("access_path", "").endswith("method_call") and node.get("ident", "") == method:
            if node.get("children", [{"ident": ""}])[0].get("ident", "") == caller:
                matches.append(node)
                return

    list(map(check_node, flatten_tree(self)))
    return matches


def find_assignments(self: dict, ident: str, value_ident: str) -> list[dict]:
    """
    Finds assignment expressions where a specific identifier is assigned a specific value.

    Args:
        self: Root node to search from
        ident: Identifier being assigned to
        value_ident: Identifier of the value being assigned

    Returns:
        List of nodes representing assignments matching the criteria
    """
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
    """
    Finds all mutable variable declarations in the AST.

    Args:
        self: Root node to search from

    Returns:
        List of nodes representing mutable variables
    """
    matches = []

    def check_node(node: dict):
        if node.get("metadata", {}).get("mut", False):
            matches.append(node)

    list(map(check_node, flatten_tree(self)))
    return matches


def find_account_typed_nodes(self: dict, ident: str) -> list[dict]:
    """
    Finds nodes related to account types with a specific identifier.

    Args:
        self: Root node to search from
        ident: Identifier to match for account types

    Returns:
        List of nodes representing account-typed elements
    """
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
    """
    Finds member access expressions for a specific identifier.

    Args:
        self: Root node to search from
        ident: Identifier to find member accesses for

    Returns:
        List of nodes representing member access expressions
    """
    matches = []

    def check_node(node: dict):
        if node.get("ident", "") == ident and (
                "tokens" in node.get("access_path", "") or "call.args" in node.get("access_path", "")):
            matches.append(node)

    list(map(check_node, flatten_tree(self)))
    return matches


def first(nodes: list[dict]) -> dict:
    """
    Returns the first node from a list, or EMPTY_NODE if the list is empty.

    Args:
        nodes: List of nodes

    Returns:
        First node in the list or EMPTY_NODE if empty
    """
    return nodes[0] if nodes else EMPTY_NODE


def find_ident_src_node(sub_data, sub_access_path: str, metadata: dict) -> dict:
    """
    Recursively searches for a node containing an 'ident' key and creates an AST node.

    Args:
        sub_data: Data structure to search through
        sub_access_path: Access path for the current search location
        metadata: Metadata to attach to the found node

    Returns:
        New AST node if ident found, otherwise EMPTY_NODE
    """
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


def find_fn_names(node):
    """
    Finds all function names in an AST node by searching for 'fn.ident' patterns.

    Args:
        node: AST node to search through

    Returns:
        List of function names found in the node
    """
    found = []
    stack = [node]
    seen = set()
    _str_node = str(node)
    approx_nb_element = _str_node.count(",") + 1 + _str_node.count("[") + _str_node.count("{")

    for _ in range(approx_nb_element):
        if not stack:
            break

        current = stack.pop()
        current_step = repr(current)
        # TODO: verify why this is problem to not check whether the node is seen (in starlark engine)
        # starlark memory management magic problem bypass, without this it will do infinite cycle probably due to pointer reference things
        if current_step in seen:
            continue
        seen.add(current_step)

        if isinstance(current, dict):
            function_name_found = False
            if "fn" in current:
                if "ident" in current["fn"]:
                    found.append(current["fn"]["ident"])
                    function_name_found = True
            if not function_name_found:
                for value in current.values():
                    stack.append(value)
        elif isinstance(current, list):
            for item in current:
                stack.append(item)

    return found

def find_raw_nodes_by_fn_names(node, func_names):
    """
    Finds raw AST nodes that match specific function names.

    Args:
        node: AST node to search through
        func_names: List of function names to match

    Returns:
        List of dictionaries containing matched raw nodes and their metadata
    """
    found = []

    stack = [node]
    _str_node = str(node)
    approx_nb_element = _str_node.count(",") + 1 + _str_node.count("[") + _str_node.count("{")
    seen = set()

    for _ in range(approx_nb_element):
        if not stack:
            break
        current = stack.pop()
        current_step = repr(current)
        # TODO: verify why this is problem to not check whether the node is seen (in starlark engine)
        # starlark memory management magic problem bypass, without this it will do infinite cycle probably due to pointer reference things
        if current_step in seen:
            continue
        seen.add(current_step)

        if isinstance(current, dict):
            function_node_found = False
            if "raw_node" in current:
                if "ident" in current["raw_node"] and current["raw_node"]["ident"] in func_names:
                    found.append({"root": current["raw_node"], "metadata": current["metadata"]})
                    function_node_found = True
            if not function_node_found:
                for value in current.values():
                    stack.append(value)
        elif isinstance(current, list):
            for item in current:
                stack.append(item)

    return found

def find_raw_nodes(ast):
    """
    Finds all raw nodes in an AST by first extracting function names and then finding matching nodes.

    Args:
        ast: AST to search through

    Returns:
        List of raw nodes with their metadata
    """
    fn_names = find_fn_names(ast)
    return find_raw_nodes_by_fn_names(ast, fn_names)


def prepare_syn_ast_iterative(ast, access_path, parent):
    """
    Iteratively processes a syntax AST to create structured nodes with metadata and relationships.

    Args:
        ast: Raw AST data structure to process
        access_path: Current access path in the AST
        parent: Parent node for the current processing context

    Returns:
        List of processed AST nodes with proper structure and metadata
    """
    nodes = []
    stack = [(ast, access_path, parent)]
    last_known_position = None

    # Use a for loop over a large range to simulate a while loop,
    # as 'while' is not supported in all Starlark environments.
    for _ in range(100000000):
        if not stack:
            break

        current_ast, current_path, current_parent = stack.pop()

        if type(current_ast) == "list":
            _handle_list_node(current_ast, current_path, current_parent, stack)
            continue

        if type(current_ast) == "dict":
            node, last_known_position = _handle_dict_node(
                current_ast, current_path, current_parent, last_known_position
            )

            if node:
                nodes.append(node)
                parent_for_children = node
            else:
                parent_for_children = current_parent

            _add_children_to_stack(current_ast, current_path, parent_for_children, stack)

    return nodes


def _handle_list_node(ast_list, current_path, current_parent, stack):
    """
    Handle processing of list nodes by adding items to stack in reverse order.

    Args:
        ast_list: List of AST items to process
        current_path: Current access path
        current_parent: Parent node for the list items
        stack: Processing stack to add items to
    """
    for i in range(len(ast_list) - 1, -1, -1):
        item = ast_list[i]
        new_path = "{}[{}]".format(current_path, i)
        stack.append((item, new_path, current_parent))


def _handle_dict_node(ast_dict, current_path, current_parent, last_known_position):
    """
    Handle processing of dict nodes and return created node and updated position.

    Args:
        ast_dict: Dictionary AST node to process
        current_path: Current access path
        current_parent: Parent node
        last_known_position: Last known position for metadata

    Returns:
        Tuple of (created_node, updated_position)
    """
    node_type = _get_node_type(ast_dict)

    if not node_type:
        return None, last_known_position

    metadata, updated_position = _build_metadata(ast_dict, last_known_position)

    if node_type == "mut":
        return _handle_mut_node(ast_dict, current_path, metadata, current_parent), updated_position
    else:
        return _create_standard_node(ast_dict, node_type, metadata, current_path, current_parent), updated_position


def _get_node_type(ast_dict):
    """
    Determine the type of AST node based on its keys.

    Args:
        ast_dict: Dictionary to check for node type

    Returns:
        String representing the node type, or None if no type found
    """
    node_types = ["method", "int", "mut", "ident"]
    for node_type in node_types:
        if ast_dict.get(node_type, False):
            return node_type
    return None


def _build_metadata(ast_dict, last_known_position):
    """
    Build metadata dict with position and mut information.

    Args:
        ast_dict: Dictionary to extract metadata from
        last_known_position: Previous position to use if current has none

    Returns:
        Tuple of (metadata_dict, updated_position)
    """
    metadata = {}
    updated_position = last_known_position

    if "position" in ast_dict:
        metadata["position"] = ast_dict["position"]
        updated_position = ast_dict["position"]
    elif last_known_position:
        metadata["position"] = last_known_position

    if "mut" in ast_dict:
        metadata["mut"] = ast_dict["mut"]

    return metadata, updated_position


def _handle_mut_node(ast_dict, current_path, metadata, current_parent):
    """
    Handle mut node type specifically.

    Args:
        ast_dict: Dictionary containing mut node data
        current_path: Current access path
        metadata: Metadata for the node
        current_parent: Parent node

    Returns:
        Created node or None if no ident found
    """
    found_node = find_ident_src_node(ast_dict, current_path, metadata)
    if found_node != EMPTY_NODE:
        ast_node_add_child(current_parent, found_node)
        return found_node
    return None


def _create_standard_node(ast_dict, node_type, metadata, current_path, current_parent):
    """
    Create a standard AST node for method, int, or ident types.

    Args:
        ast_dict: Dictionary containing node data
        node_type: Type of node to create
        metadata: Metadata for the node
        current_path: Current access path
        current_parent: Parent node

    Returns:
        Created AST node
    """
    node = new_ast_node(ast_dict, metadata, current_path)
    node["parent"] = current_parent

    if node_type == "method":
        node["ident"] = ast_dict["method"]
    elif node_type == "int":
        node["ident"] = str(ast_dict["int"])
    return node


def _add_children_to_stack(ast_dict, current_path, parent_for_children, stack):
    """
    Add all child items to the stack in reverse order.

    Args:
        ast_dict: Dictionary containing child items
        current_path: Current access path
        parent_for_children: Parent node for the children
        stack: Processing stack to add items to
    """
    child_items = list(ast_dict.items())
    for i in range(len(child_items) - 1, -1, -1):
        key, value = child_items[i]
        new_path = "{}.{}".format(current_path, key) if current_path else key
        stack.append((value, new_path, parent_for_children))

def prepare_ast(ast: list[dict]) -> dict:
    """
    Prepares a complete AST from raw syntax data by processing nodes and establishing parent-child relationships.

    Args:
        ast: List of raw AST dictionaries to process

    Returns:
        Root AST node with all children properly linked
    """
    nodes = prepare_syn_ast_iterative(ast, "", EMPTY_NODE)
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
    root = new_ast_node({ "root": True }, {}, "root")
    for node in nodes:
        if node.get("parent", EMPTY_NODE) == EMPTY_NODE:
            ast_node_add_child(root, node)
    return root


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
    find_fn_names=find_fn_names,
    find_raw_nodes_by_fn_names=find_raw_nodes_by_fn_names,
    find_raw_nodes=find_raw_nodes,
    prepare_ast=prepare_ast,
)
