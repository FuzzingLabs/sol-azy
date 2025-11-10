# Constraint & Helper Tags

Inside **Constrained** column, tags are attached to the field name to indicate which checks or helper macros apply. These are the tags you’ll see:

- **`has_one`**
  From `has_one = <field>` constraints. Ensures the current account’s stored pubkey equals another account’s key. \
  *Use:* Ownership/binding check.

- **`address`**
  From `address = <expr>` constraints. Pins the account to a specific known address (often sysvars). \
  *Use:* Ensure the provided account is exactly one expected address.

- **`constraint`**
  From `constraint = <predicate>`. Free-form runtime conditions the program checks before continuing. \
  *Use:* Anything from balance thresholds to relationship checks not covered by `has_one`.

- **`owner`**
  From `owner = <Program>` constraints in the attribute. Enforces the owning program for an account (e.g., Token Program). \
  *Use:* Ensure account is controlled by the right program (critical for token/state integrity).

- **`spl`**
  Detected when SPL helper attributes are present on the field, e.g.:
  * `associated_token::mint = ...`
  * `associated_token::authority = ...`
  * `token::mint = ...`, `token::authority = ...`
  * `mint::authority = ...`, `mint::freeze_authority = ...`, `mint::decimals = ...`\
    *Use:* These helpers encode common SPL invariants; presence of `spl` implies ATAs and token relationships are being checked declaratively.

> Formatting reminder: `field(tag1,tag2)` indicates multiple constraint types apply to the same field.

_See more about constraint [here](https://www.quicknode.com/guides/solana-development/anchor/how-to-use-constraints-in-anchor)_