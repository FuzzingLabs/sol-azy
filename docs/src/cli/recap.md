# `recap` Command

The `recap` command generates a compact, audit-friendly summary for an Anchor project.  
It inspects IDL(s) under `target/idl/`, tries to map each IDL to its Anchor crate, performs lightweight source parsing of `#[derive(Accounts)]` blocks, and emits per-program Markdown tables with: **Instruction | Signers | Writable | Constrained | Seeded | Memory**.

---
## Usage

```bash
# run recap on current directory (default) -> creates ./recap-solazy.md
cargo run -- recap

# run recap on a specific project path (optional -p) -> creates ./recap-solazy.md in the cwd
cargo run -- recap -d ../my-solana-project
````

**Arguments:**

* `-d, --target-dir <PATH>` — optional, path to the project root. If omitted the current working directory is used.

---

## How It Works

1. Verify the target directory looks like an Anchor project (presence of `Anchor.toml`).
2. Discover IDL JSON files under `target/idl/`.
3. Parse each IDL to obtain instruction and account lists.
4. Find Anchor crates in the repo by scanning `Cargo.toml` files for an `anchor-lang` dependency, then attempt to map each IDL to the best-matching crate:
   * prefer crate with the same package name as `idl.name`,
   * otherwise pick the crate with the largest overlap between IDL instruction names and functions discovered in the crate source.
5. For each mapped crate, the implementation concatenates `src/*.rs` into a single string (heuristic) and performs lightweight parsing that:
   * searches functions for `Context<...>` (the parser scans for any `Context<...>` occurrence in a function’s parameter list) and extracts the last top-level generic as the Accounts struct name (e.g. `Context<'_, '_, '_, 'info, Foo<'info>>` → `Foo`),
   * extracts `#[derive(Accounts)]` structs and aggregates stacked `#[account(...)]` attributes attached to fields,
   * detects markers inside the `#[account(...)]` attributes: `seeds = [...]`, `has_one = ...`, `address = ...`, `constraint`/`constraints`, SPL helpers like `token::mint`, `associated_token::mint`, `mint::authority`, and memory-related flags like `space`, `realloc`, `realloc::zero`,
   * flattens IDL account trees (via `flatten_accounts`) to map IDL leaf account names to struct fields and then annotates table columns (Constrained, Seeded, Memory).
6. Produce `recap-solazy.md` containing one section per IDL/program and a Markdown table per program.

---

## Output

* File generated: `recap-solazy.md` (created in the current working directory).
* The file contains one section per IDL/program with a Markdown table listing, for each instruction:

  * **Signers** — accounts flagged as signers in the IDL
  * **Writable** — accounts flagged writable / `mut` in the IDL
  * **Constrained** — fields with `has_one`, `address`, `owner`, `constraint(s)` or recognized SPL attribute markers
  * **Seeded** — fields using `seeds = [...]` (detected from `#[account(...)]`)
  * **Memory** — fields using `space` or `realloc` / `realloc::zero`

The output is intended as a quick-start audit report — readable, compact, and suitable for inclusion in initial findings.

---

## Limitations & Notes (important)

* **Anchor-only**: the command expects Anchor-style IDLs and an `Anchor.toml` project marker. Native Rust / Shank projects are not covered by this command.
* **Heuristic file handling**: the tool **concatenates `src/*.rs`** as a quick heuristic (it does not perform Rust module resolution). This is fast and works for many repos, but can miss or mis-attribute items in projects that rely heavily on `mod ...;` file layout, `pub(crate)` scope tricks, or macros that generate the `Accounts` structs.
* **Text-based parsing**: account/attribute detection is implemented with lightweight parsing / regex heuristics:
  * it finds `#[derive(Accounts)]` and groups stacked `#[account(...)]` attributes,
  * it searches inside attributes for tokens like `seeds = [`, `has_one =`, `address =`, `constraint`, `space`, `realloc`, and SPL shorthand forms (e.g. `associated_token::mint = ...`),
  * these heuristics are fast but can produce false negatives on extremely exotic code constructs, unusual macro expansions, or heavily nested generics inside attributes.
* **`Context` detection**: the function-mapper looks for **any** `Context<...>` usage in the fn parameters (qualified or unqualified).
* **IDL → crate mapping**: mapping is best-effort: exact `idl.name` match preferred; otherwise instruction-name overlap is used. In multi-program monorepos this heuristic generally works but may need manual review for ambiguous cases.
* **Output filename is fixed**: the current tool writes results to `recap-solazy.md`. Changing this behavior is a small code tweak if you prefer stdout or a configurable filename.

---

## Example

```bash
# analyze current project and generate recap-solazy.md
cargo run -- recap
# or if compiled under solazy bin
solazy recap

# or analyze a specific project path
cargo run -- recap -d ../helium-program-library
# or if compiled under solazy bin
solazy recap -d ../helium-program-library
```

---

## Related

* [Recap overview](../recap_module.md)
* [Recap columns](../recap/columns.md)
* [Recap constraints](../recap/constraints.md)
* [Recap tips & example](../recap/tips_and_example.md)
