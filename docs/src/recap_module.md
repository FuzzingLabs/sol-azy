# `recap` — What it produces and How to read it

The `recap` module builds an audit-friendly snapshot of each program in an Anchor project.
For every instruction, it emits a compact Markdown table with six columns:

    | Instruction | Signers | Writable | Constrained | Seeded | Memory |

This section explains what each column means, how values are derived, and how to interpret the tags you’ll see inside **Constrained** and **Memory**.

## See [Columns](./recap/columns.md)
## See [Constraints](./recap/constraints.md)
## See [Tips & Example](./recap/tips_and_example.md)
## See [CLI & How it works](./cli/recap.md)

## Limitations

- **Anchor-focused**: uses Anchor IDLs and attributes. Native Rust/Shank not (yet) covered.
- **Heuristic parsing**: attributes are parsed from source text; exotic macro expansions or unusual patterns may not be detected.
- **File layout**: analysis aggregates `src/*.rs`; complex module layouts may occasionally hide or duplicate definitions until deeper parsing is added.