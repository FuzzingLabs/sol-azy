# What Are Templates?

Templates in Sol-azy are reusable pattern matchers designed to identify **specific AST fragments** in Rust source code.
They allow users to describe **common logic constructs** in a simple, declarative way, and can be used as building blocks in Starlark rules or during static pattern matching.

They are especially useful when:

* A pattern appears frequently (e.g., `ctx.accounts.authority.is_signer`)
* You want to simplify rule definitions by abstracting repetitive AST shapes

## Template Anatomy

Each template includes:

* A `pattern`: describing a shape to find in the AST, using simplified `idents` (Rust path segments)
* A `priority_rule`: used to guide traversal and maintain node order consistency

### Example Template

```python
TEMPLATES["CHECK_CTX_ACCOUNT_AUTHORITY_KEY_TOKEN_OWNER"] = {
    "pattern": {
        "cond": {
            "binary": {
                "left": {"idents": ["ctx", "accounts", "authority", "key"]},
                "op": "!=",
                "right": {"idents": ["token", "owner"]},
            }
        }
    },
    "priority_rule": ["left", "op", "right"],
}
```

This matches AST code like:

```rust
if ctx.accounts.authority.key != &token.owner
```

Each template defines a **shallow structural pattern** over **AST nodes of maximum depth 3**.

> For now, a maximum depth of 3 levels in AST node matching has proven sufficient, but deeper recursive pattern support could be added in the future if needed, directly within the `template_manager.star` logic

## Fields Used

### `idents`

Lists of identifier segments (e.g., `["ctx", "accounts", "authority", "key"]`) that are matched **in order**, exactly.

### `method`

Optional field for method calls like `.key()`.

### `op`

Operator such as `"=="` or `"!="`.

### `macro`, `call`, `unary`, `binary`, `field`

These correspond to the Rust AST node types (extracted from `syn`) that can be matched.

## `priority_rule`

The `priority_rule` defines the traversal order of keys inside a pattern node.
It ensures that, during linearization of the AST, the relevant fields are matched in the correct order, especially in constructs like:

```python
"priority_rule": ["left", "op", "right"]
```

This guarantees consistent matching across pattern instances.

## Wildcard Support

You can use a wildcard `*` in the `idents` list to match **any one identifier**.
For example:

```python
"idents": ["ctx", "accounts", "*"]
```

...matches any field under `ctx.accounts`, such as `ctx.accounts.user_a`.

## Dynamic Template Creation

For convenience, you can add generator for classical templates programmatically, this one is an example:

```python
def generate_call_fn_template(*idents):
    return {
        "pattern": {
            "call": {
                "args": "",  # ignored for now
                "func": {"idents": idents},
            }
        },
        "priority_rule": ["func", "args"],
    }
```

This allows you to match function calls like:

```rust
solana_program::program::invoke(...)
```

and by using the dynamic generation with `generate_call_fn_template("solana_program", "program", "invoke")`, you don't have to manually write a full template.

## Template Testing

The folder `test_starlark_condition_template/` contains a `test.py` script that acts as a an example place for templates.

It defines AST snippets and verifies that each pattern matches correctly:

```python
    # if
    assert is_matching_template_by_key(AST, "CHECK_CTX_ACCOUNT_AUTHORITY_KEY_TOKEN_OWNER")
    assert is_matching_template_by_key(AST2, "CHECK_SPLTOKEN_ID_CTX_ACCOUNT_AUTHORITY_KEY")
    assert is_matching_template_by_key(AST3, "CHECK_NOT_CTX_ACCOUNTS_AUTHORITY_ISSIGNER")
    assert is_matching_template_by_key(AST4, "CHECK_CTX_ACCOUNTS_WILDCARD_KEY_EQ")

    # require
    assert is_matching_template_by_key(AST5, "REQUIRE_CTX_ACCOUNTS_RENT_KEY_SYSVAR_RENT_ID")

    # called function
    assert is_matching_template_by_key(AST2, "CALL_FN_SOLANAPROGRAM_PROGRAM_INVOKE")

    # dynamic template
    assert is_matching_template(AST2, generate_call_fn_template("solana_program", "program", "invoke"))
```

## Summary of supported pattern types already implemented (can easily be extended)

Templates can express:

* **Binary comparisons** (`==`, `!=`)
* **Unary operations** (`!some_flag`)
* **Field access** (`ctx.accounts.x.is_signer`)
* **Macro calls** (`require_eq!(...)`)
* **Method calls** (`ctx.accounts.user_a.key()`)
* **Function calls** (example: `solana_program::program::invoke(...)`)

They support wildcards like `"*"` to generalize over certain path segments.

## Usage in Rules

Templates are often used within `.star` files like this:

```python
if template_manager.is_matching_template_by_key(node, "CHECK_CTX_ACCOUNT_AUTHORITY_KEY_TOKEN_OWNER"):
    continue # continue the loop over all nodes, the check is there, so it's probably not vuln
```

This allows auditors or developers to write rules that check for high-level semantic conditions without diving into low-level AST fields every time.

## Advantages

* 🔁 Reusability: templates can be applied across multiple rules
* 🔍 Precision: match deeply nested expressions in structured order
* 🔧 Extensibility: you can write custom templates without editing core logic
