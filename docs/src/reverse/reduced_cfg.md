# Reduced Control Flow Graph (CFG)

Analyzing large Solana eBPF programs can produce overwhelming control flow graphs (CFGs) due to the sheer number of functions and basic blocks.
sol-azy offers two modes to reduce graph complexity:

* `--reduced`: Only include functions defined *after* the entrypoint.
* `--only-entrypoint`: Include **only** the function cluster of the entrypoint itself.

---

## 1. `--reduced`

The `--reduced` flag filters the generated CFG by discarding functions that are likely part of the runtime or standard library.

### Example

```bash
cargo run -- reverse \
  --mode cfg \
  --out-dir ./out/ \
  --bytecodes-file ./program.so \
  --labeling \
  --reduced
```

### What It Does

* Keeps only functions that appear **after** the `entrypoint` in the binary layout.
* Typically corresponds to user-defined logic.
* Excludes Solana runtime boilerplate (e.g., `abort_internal`, `core::fmt`, etc.)

---

## 2. `--only-entrypoint`

The `--only-entrypoint` flag isolates just the **entrypoint function**, without including its callees or any other clusters.

### Example

```bash
cargo run -- reverse \
  --mode cfg \
  --out-dir ./out/ \
  --bytecodes-file ./program.so \
  --labeling \
  --only-entrypoint
```

### What It Does

* Only exports the cluster corresponding to the `entrypoint`.
* Skips all other functions, even if they are part of user logic.
* Ideal for initializing a **minimal CFG** for manual extension.

---

## Why It Matters

* ✅ Greatly improves readability for large programs
* ✅ Speeds up rendering in tools like `xdot` or Graphviz
* ✅ Useful for focused auditing and vulnerability research

⚠️ With `reduced`, depending on program structure, some utility functions may still be present if called after the entrypoint.

---

## Comparison

| Flag                | Includes Entry? | Includes Callees? | Includes Library Code? |
| ------------------- | --------------- | ----------------- | ---------------------- |
| (default / full)    | ✅               | ✅                 | ✅                      |
| `--reduced`         | ✅               | ✅                 | ❌                      |
| `--only-entrypoint` | ✅               | ❌                 | ❌                      |

---

## Visualization

You can render the resulting `.dot` files as usual:

```bash
dot -Tsvg cfg.dot -o cfg.svg
xdot cfg.dot
```

Reduced graphs will render faster and be easier to navigate.

> ⚠️ For very large programs, even the `--reduced` version of the CFG can take significant time to generate due to the size and complexity of the bytecode being analyzed and rendered by `dot`.

---

## When to Use

| Scenario                                  | Recommended Flag       |
| ----------------------------------------- | ---------------------- |
| You want to analyze app logic only        | `--reduced`            |
| You want to isolate `entrypoint` manually | `--only-entrypoint`    |
| You need full picture including libraries | *(default - no flags)* |

---

## Related

* [Control Flow Graph (CFG)](cfg.md)
* [Dotting (CFG editing)](dotting.md)
* [Reverse CLI Options](../cli/reverse.md)
