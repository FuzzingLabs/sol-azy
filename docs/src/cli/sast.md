# `sast` Command

The `sast` command performs **Static Application Security Testing** on Solana projects using a custom rule engine.  
It parses the Rust source code, builds an AST, and applies **Starlark-based rules** to detect potential vulnerabilities or design patterns.

---

## Usage

```bash
cargo run -- sast \
  --target-dir ./my_project \
  --rules-dir ./rules/ \
  --syn-scan-only
```

**Arguments:**

- `--target-dir`: Path to the root of the Solana project.
- `--rules-dir`: Directory containing `.star` rule files.
- `--syn-scan-only`: If true, only perform syntactic scanning (no build required).

---

## How It Works

The SAST engine:

1. **Parses all `.rs` files** under the target project (Anchor or native SBF)
2. **Builds a `syn` AST** enriched with source spans
3. **Loads all `.star` rule files** from the provided rules directory
4. Applies the rules and collects any matches (vulnerabilities, code smells, patterns)

Rules are written in [Starlark](https://github.com/bazelbuild/starlark), making them:
- Secure
- Sandboxable
- Easy to reason about

---

## Rule File Example

```python
load("syn_ast.star", "syn_ast")

RULE_METADATA = struct(
    name = "DangerousPanicUsage",
    author = "FuzzingLabs",
    version = "0.1",
    severity = "High",
    certainty = "High",
    description = "Detects usage of `panic!` in logic paths",
)

def syn_ast_rule(ast):
    return [node for node in ast if node["ident"] == "panic"]
```

---

## Output

Sol-azy prints results in a terminal table or as JSON.

- Rule metadata
- File names
- Matches and associated spans (if available)

---

## Example

```bash
cargo run -- sast \
  --target-dir test_cases/base_anchor/programs/base_anchor \
  --rules-dir ./rules/ \
  --syn-scan-only
```

---

## Related

- [Rule Format](../rules/format.md)
- [Example Rules](../rules/examples.md)
- [SAST Engine Architecture](../architecture/sast_engine.md)