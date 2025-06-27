# Starlark Libraries References

## AST Node Structure

An AST node represents a single element in the Abstract Syntax Tree with a standardized structure. Each node contains: a
`raw_node` with the original AST data, an `access_path` string representing the node's location in the tree, `metadata`
for additional information like position and mutability flags, `children` and `parent` references for tree navigation,
`ident` for the node's identifier, and optional fields like `args` for function arguments and `root` to indicate if it's
a root node. This consistent structure facilitates tree traversal and pattern matching operations throughout the
codebase.

```python
{
    "raw_node": {},   # Original AST data from parser
    "access_path": "",# Path string showing location in tree (e.g. "root.expr.binary.left")
    "metadata": {},   # Additional data like position and mutability flags
    "children": [],   # List of child nodes
    "parent": {},     # Reference to parent node
    "root": False,    # Boolean indicating if this is a root node
    "args": [],       # Function arguments (if applicable)
    "ident": ""       # Node identifier/name
}
```

## Syn AST Utilities

The Syn AST utilities module provides functions for working with Rust's syntactic ASTs, particularly for security
analysis. 

### Core Components

#### Constants

- `EMPTY_ACCESS_PATH`, `EMPTY_IDENT`, `EMPTY_METADATA`, `EMPTY_NODE`: Default values for empty nodes

#### Node Management

- `new_ast_node(syn_ast_node, metadata, access_path)`: Creates a new AST node
- `ast_node_add_child(node, child)`: Adds a child to an AST node
- `ast_node_add_children(node, children)`: Adds multiple children to an AST node
- `to_result(node)`: Converts a node to a result format
- `filter_result(result)`: Filters duplicate results

#### Tree Traversal

- `traverse_tree(node, collector)`: Traverses a tree with a collector function
- `flatten_tree(root)`: Flattens a tree into a list of nodes
- `first(nodes)`: Returns the first node from a list

#### Node Finding Functions

- `find_by_child(self, child_ident)`: Finds nodes by child identifier
- `find_chained_calls(self, *idents)`: Finds chained method calls
- `find_macro_attribute_by_names(self, *idents)`: Finds macro attributes by name
- `find_by_similar_access_path(self, access_path, stop_keyword)`: Finds nodes with similar access paths
- `find_comparisons(self, ident1, ident2)`: Finds comparisons between two identifiers
- `find_comparison_to_any(self, ident)`: Finds comparisons involving a specific identifier
- `find_functions_by_names(self, *function_names)`: Finds functions by name
- `find_by_names(self, *idents)`: Finds nodes by identifier names
- `find_method_calls(self, caller, method)`: Finds method calls on a specific caller
- `find_assignments(self, ident, value_ident)`: Finds assignment operations
- `find_mutables(self)`: Finds mutable variables
- `find_account_typed_nodes(self, ident)`: Finds account-typed nodes
- `find_member_accesses(self, ident)`: Finds member accesses for a specific identifier

#### AST Preparation

- `find_ident_src_node(sub_data, sub_access_path, metadata)`: Finds identifier source nodes
- `find_fn_names(node)`: Extracts function names from an AST
- `find_raw_nodes_by_fn_names(node, func_names)`: Finds raw nodes by function names
- `find_raw_nodes(ast)`: Finds all raw nodes in an AST
- `prepare_syn_ast(ast, access_path, parent)`: Prepares a Syn AST for analysis
- `prepare_ast(ast)`: Main function to prepare an AST for analysis

### Usage Examples

For finding specific code patterns:

```python
```

This documentation provides an overview of the functionality available in these Starlark libraries, which are designed
for security analysis of Solana programs by detecting specific code patterns in their AST representations.