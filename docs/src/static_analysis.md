# Static Analysis

Sol-azy includes a flexible static analysis engine designed to scan Solana Rust source code for programs vulnerabilities, code smells, or user-defined patterns.

This engine leverages the `Starlark` language to express detection logic in `.star` files, and operates directly on the parsed Rust Abstract Syntax Tree (AST).

---

## Key Concepts

- **AST-Based**: Operates purely on the Rust syntax tree using the [`syn`] crate — no type inference or semantic resolution is performed. _(We're working on a future MIR...)_
- **Declarative Rules**: Users write `.star` scripts to describe what they want to detect.
- **Safe & Sandboxed**: Rules are evaluated inside a restricted Starlark runtime.

---

## Rule Engine Capabilities

The rule engine gives you access to:

- Node inspection (e.g. calls, structs, attributes, visibility)
- Parent-child relationships in AST
- Span and file location tracking
- Metadata enrichment (severity, certainty, etc.)
- JSON-compatible output for integration

---

## Use Cases

- Anchor account declaration validation
- Detection of unsafe CPI (Cross Program Invocation)
- Missing signer or owner checks
- Misuse of `invoke_signed` or unchecked sysvars
- Custom security checks during CI

---

## Related Pages

- 📘 [How to write Rules](rules/format.md)
- ✅ [Use case example](rules/example.md) 

## Note

The sast engineering core in Sol-azy is based on the excellent open-source project  
[`radar`](https://github.com/Auditware/radar) by [Auditware)](https://github.com/Auditware).

We've been heavily inspired by their approach and wanted a standalone binary capable of it.