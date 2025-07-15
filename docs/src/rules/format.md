# Rule Format

sol-azy allows developers and auditors to write custom **static analysis rules** using the [Starlark language](https://github.com/bazelbuild/starlark) — a Python-like configuration language used by projects like [Bazel](https://bazel.build/rules/language) and [Buck/Buck2](https://github.com/facebook/buck2) ([Buck2 docs](https://buck2.build/docs/developers/starlark/environment/)).

These rules are evaluated against the **Rust AST** (Abstract Syntax Tree) of a Solana program, enabling precise pattern matching to detect vulnerabilities or code smells.

## Rule File Structure

A valid rule file is a `.star` script containing two main parts:

1. **`RULE_METADATA`** — a dictionary with basic info
2. **`syn_ast_rule(root)`** — the entrypoint function, called on each parsed file

```python
RULE_METADATA = {
    "version": "0.1.0",
    "author": "your-name",
    "name": "Rule Name",
    "severity": "Low" | "Medium" | "High" | "Critical",
    "certainty": "Low" | "Medium" | "High",
    "description": "What the rule checks for"
}
```

## Example Rule: Arbitrary CPI

```python
RULE_METADATA = {
    "version": "0.1.0",
    "author": "forefy",
    "name": "Arbitrary Cross-Program Invocation",
    "severity": "Medium",
    "certainty": "Medium",
    "description": "Detects CPIs made to arbitrary or unchecked program IDs."
}

def syn_ast_rule(root: dict) -> list[dict]:
    matches = []
    raw_nodes = syn_ast.find_raw_nodes(root)
    for sink in raw_nodes:
        if template_manager.is_matching_template_by_key(sink, "CALL_FN_SOLANAPROGRAM_PROGRAM_INVOKE") and not template_manager.is_matching_template_by_key(sink, "CHECK_SPLTOKEN_ID_CTX_ACCOUNT_AUTHORITY_KEY"):
            matches.append(syn_ast.to_result(sink))
    return matches
```

## Execution Flow

When sol-azy runs a rule:

1. It parses the source code into an AST
2. Converts it to JSON
3. Passes it as the `root` parameter to `syn_ast_rule`
4. The rule inspects the tree using helper functions (typically, here the `syn_ast.find_raw_nodes()` here is used to gather each function independently)
5. Any result added to `matches` is reported

## Helper Libraries

sol-azy ships with built-in sol-azy helpers (written in Starlark):

```
src/static/starlark_libs/
├── syn_ast.star            # AST navigation utilities
└── template_manager.star   # Match against common templates
```

These can be imported and used in any rule. Examples:

```python
raw_nodes = syn_ast.find_raw_nodes(root)
template_manager.is_matching_template_by_key(node, "CHECK_INSTRUCTION_DISCRIMINATOR")
```

> 📌 **Note:** The `template_manager` logic enables reusable pattern detection (documented in [Templates](templates.md)).

## Writing New Rules

To create a new rule:

1. Create a `.star` file in your rules directory
2. Define `RULE_METADATA` and `syn_ast_rule(...)`
3. Use `cargo run -- sast ...` to apply the rule


## Documentation

* [Starlark language reference (GitHub)](https://github.com/bazelbuild/starlark)
* [Starlark spec & built-ins](https://github.com/bazelbuild/starlark/blob/master/spec.md)
* [Starlark in Bazel (docs)](https://bazel.build/rules/language)
* [Starkark in Buck2 (docs)](https://buck2.build/docs/developers/starlark/environment/)

## Related

* [Use case example](example.md)
* [Writing Templates](templates.md)
* [Running Static Analysis](../cli/sast.md)