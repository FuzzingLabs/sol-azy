# Tips & Example

## Reading the table like an Auditor

Use columns together to quickly spot intent and risk. All facts below come **directly** from the table; any inference is explicitly marked.

1. **Privilege map**
   - Cross-check **Signers** vs **Writable**. A signer writing to critical state is normal, but deserves a closer look.
   - If an instruction has many writables but few constraints, flag for manual review.

2. **Authorization evidence**
   - In **Constrained**, the presence of `has_one`, `address`, `constraint` (or `spl` shorthands) indicates binding between accounts.
   - If a **Writable** account has **no** `has_one`/`address`/`constraint` and is **not** **Seeded**, that’s a probable weak binding.

3. **Seed usage**
   - **Seeded** lists PDAs with `seeds = [...]`.
   - If a Seeded PDA is **also** used as a signer in CPIs (you’ll only see this by reading code; `recap` itself doesn’t parse CPI signers), it’s likely the program signs with `with_signer`; verify the seeds match the intended authority scope.

4. **SPL wiring**
   - `spl` in **Constrained** tells you the field uses SPL shorthands (e.g., `associated_token::mint`, `token::authority`, `mint::authority`).
   - This **usually** reduces misbinding risk, but still review who controls those authorities.

5. **Memory lifecycle**
   - **Memory** shows `space`, `realloc`, `realloc::zero` on fields. Any `realloc` **likely** implies the account can change size; verify who can trigger it and whether zeroing/rent logic is present in code.

> The table is a **map**, not the territory. Treat it as a starting shortlist for manual code review.

---

## Examples (based on truncated Helium protocol's output)

### `fanout` (excerpt)

```

| Instruction   | Signers                  | Writable                                                                                                                  | Constrained                                                                                        | Seeded                                                                            | Memory         |
| ------------- | ------------------------ | ------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------- | -------------- |
| distribute_v0 | payer                    | fanout, payer, to_account, token_account, voucher                                                                         | fanout(has_one); receipt_account(constraint,spl); to_account(spl); voucher(has_one)                | voucher                                                                           | —              |
| stake_v0      | payer, staker            | collection_metadata, fanout, from_account, master_edition, metadata, mint, payer, receipt_account, stake_account, voucher | fanout(has_one); from_account(spl); mint(constraint,spl); receipt_account(spl); stake_account(spl) | collection_master_edition, collection_metadata, master_edition, metadata, voucher | voucher(space) |
```

How to read (facts + cautious inferences):
- `stake_v0`:
  - **Fact:** there are **two signers** (`payer, staker`).
  - **Fact:** several token-related writables use `spl` shorthands (`from_account`, `stake_account`, `receipt_account`, `mint`).
  - **Fact:** `fanout` shows `has_one`; multiple PDAs appear in **Seeded** (e.g., `voucher`, collection-related PDAs).
  - **Likely implication:** the action depends on both user funding (`payer`) and staking authority (`staker`). Verify in code which signer is actually used for authority gates and CPIs; ensure the SPL account authorities match the intended signer(s).

- `distribute_v0` (for contrast):
  - **Fact:** single signer (`payer`) with SPL-bound accounts and `voucher(has_one)`.
  - **Likely implication:** distribution flow is scoped via `has_one` and SPL wiring; review any CPI that signs with program PDAs to confirm seed coverage.

---

### `lazy_transactions` (excerpt)

```

| Instruction                     | Signers | Writable                                                     | Constrained                                                       | Seeded             | Memory                   |
| ------------------------------- | ------- | ------------------------------------------------------------ | ----------------------------------------------------------------- | ------------------ | ------------------------ |
| initialize_lazy_transactions_v0 | payer   | canopy, executed_transactions, lazy_transactions, payer      | canopy(owner,constraint); executed_transactions(owner,constraint) | lazy_transactions  | lazy_transactions(space) |
| execute_transaction_v0          | payer   | executed_transactions, lazy_signer, lazy_transactions, payer | block(constraint); lazy_transactions(has_one,constraint)          | block, lazy_signer | —                        |

```

How to read (facts + cautious inferences):
- `initialize_lazy_transactions_v0`:
  - **Fact:** `lazy_transactions` is **Seeded** and allocated (`space`).
  - **Likely implication:** central state initialized here; check later instructions for writes or possible size changes.

- `execute_transaction_v0`:
  - **Fact:** `lazy_transactions` carries both `has_one` and `constraint`.
  - **Likely implication:** strong binding; still verify constraint logic (e.g., balance-vs-amount checks, signer-vs-owner checks).

---

## Practical Tips for security researcher

- **Writables without bindings:** For each row, compare **Writable** with **Constrained**/**Seeded**. If an account is writable but neither seeded nor constrained, mark it for manual review (probable weak binding).
- **Signer misuse:** If a **Signer** is present but the writable target has no binding, watch for confused-deputy style flows (probable mis-authorization).
- **SPL edges:** `spl` tags are good signs, but confirm that the authority in `token::authority` / `mint::authority` is the intended one (not user-controlled across instructions).
- **Memory growth:** Any `realloc` in **Memory** should correlate with strong constraints (`has_one`/`address`/predicate). If not present in the table, **probable** risk: size-controlled DoS or storage smuggling.
- **PDA spoofing attempts:** For any **Seeded** PDA that later signs in CPIs (you’ll see it in code, not the table), try seed-collision or context-mismatch angles (e.g., same seeds across tenants). If seeds include user-controlled data, test cross-instance manipulation.
- **Token misrouting:** When **Writable** includes token accounts **without** `spl` or `has_one`, you can probably attempt deposit/withdrawal to user-owned ATAs if constraints allow it.
