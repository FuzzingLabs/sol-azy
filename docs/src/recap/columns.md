# Columns

For every instruction, it emits a compact Markdown table with six columns:

    | Instruction | Signers | Writable | Constrained | Seeded | Memory |

This page explains what each column means.

## Details

### Instruction

- The instruction name as declared in the IDL (e.g., `distribute_v0`, `initialize_fanout_v0`).
- Source of truth: **IDL**.

### Signers

- Accounts that must sign the transaction for this instruction.
- Comes from the IDL flags (`signer` / `isSigner`).
- Audit cues:
  * Unexpected sensitive signers (e.g., PDAs showing up as signers) are red flags.
  * Privileged-path mapping: confirm that “owner” or “admin” signers align with the spec.

### Writable

- Accounts marked as writable/mutable.
- Comes from the IDL flags (`writable` / `isMut`), flattened even if IDL uses nested account groups.
- Audit cues:
  * Writable + signer is a powerful combo; ensure it’s necessary.
  * High-value state (treasuries, config, counters) being writable should be justified.

### Constrained

- Fields from the `#[account(...)]` attributes that impose **relationships** or **safety checks**.
- Shows as `field(tag1,tag2,...)`, separated by semicolons for multiple fields.
- Examples:
  * `fanout(has_one)` — field has a `has_one = ...` constraint
  * `receipt_account(constraint,spl)` — field has an explicit `constraint = ...` and SPL helper constraints
  * `sysvar_instructions(address)` — explicit `address = ...` on a sysvar
- Audit cues:
  * More constraints generally means tighter coupling and safer invariants.
  * Match constrained fields to business rules (who should “own” what, which mint pairs with which ATA, etc.).

### Seeded

- Accounts that are Program-Derived Addresses (PDAs) with explicit `seeds = [...]` in attributes.
- Lists the **field names** (e.g., `voucher`, `collection`, `lazy_signer`).
- Audit cues:
  * PDA derivations reveal authorization schemes. Verify that seeds include both program constants and caller-specific data where appropriate.
  * Cross-check against CPIs that expect a signer PDA derived by these seeds.

### Memory

- Per-field memory management annotations found in `#[account(...)]`:
  * `space` — hardcoded allocation size at init
  * `realloc` — post-init reallocation
  * `realloc::zero` — reallocation with zero-init
- Shows as `field(space)` / `field(realloc)` / `field(realloc::zero)`.
- Audit cues:
  * `space`: confirm it matches the struct + any dynamic payload; off-by-one or growth vectors matter.
  * `realloc`: ensure rent, zeroing, and access control around growth are handled to avoid state smuggling.

## How values are derived (at a glance)

- **Signers / Writable**: read from the IDL per instruction (supports nested account groups via flattening).
- **Constrained**: parsed from `#[account(...)]` attributes on the associated `#[derive(Accounts)]` struct for the instruction:
  * `has_one`, `address`, `constraint`
  * `owner` constraint
  * SPL helpers (grouped as `spl`)
- **Seeded**: presence of `seeds = [ ... ]` in the same attributes.
- **Memory**: presence of `space = ...`, `realloc = ...`, or `realloc::zero`.

> Note: constraints are attached to the field names, and only shown for fields that also appear in the instruction’s IDL account list (to avoid surfacing unrelated context fields).