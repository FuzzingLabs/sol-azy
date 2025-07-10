# Fetcher

The `fetcher` command allows you to retrieve the deployed bytecode of a Solana program and save it locally as `fetched_program.so`.

This is useful for performing offline analysis, reverse engineering, or static checks without relying on local source code or Solana toolchain.

## Usage

```sh
cargo run -- fetcher \
  --program-id <PROGRAM_ID> \
  --out-dir <OUTPUT_DIR> \
  [--rpc-url <CUSTOM_RPC_ENDPOINT>]
````

* `--program-id`: The Solana program ID to fetch.
* `--out-dir`: Directory where the bytecode file will be saved (as `fetched_program.so`).
* `--rpc-url`: (Optional) Custom Solana RPC endpoint. Defaults to `https://api.mainnet-beta.solana.com`.

## Behavior

* Checks if the output directory exists (if not it creates the folder).
* Validates the program exists on-chain and is executable.
* Writes the bytecode to the specified directory.
* Logs the output file path & the RPC used, including when default is applied.

## Example

```sh
cargo run -- fetcher \
  --program-id srmqPvymJeFKQ4zGQed1GFppgkRHL9kaELCbyksJtPX \
  --out-dir ./out
```

This will fetch the bytecode of the program and save it to `./out/fetched_program.so`.

## How does it works?

### Data Accounts vs Executable Accounts

On Solana every account is just a blob of bytes, but the **runtime** sets one special flag:

| Flag `executable` | Typical content                                                                                             | File saved by **fetcher** |
| ----------------- | ----------------------------------------------------------------------------------------------------------- | ------------------------- |
| **`true`**        | BPF byte-code of a program (optionally behind an *Upgradeable Loader* “Program → ProgramData” indirection). | `fetched_program.so`      |
| **`false`**       | Arbitrary user-defined state: SPL token mints, AMM pools, governance realms, Anchor structs, sysvars, …     | `fetched_account.bin`     |

`fetcher` detects this flag automatically:

* If the account is executable it resolves the **ProgramData** pointer (when present), trims everything **before** the ELF header, then writes a clean shared object.
* Otherwise it dumps the raw data unchanged.

If you want to look at the code, there are unit tests that illustrate both paths:

* `test_fetch_executable` fetches the **Serum DEX v3** program and asserts the ELF header is present.
* `test_fetch_non_executable_sysvar` fetches the **Sysvar Rent** account and checks its 17-byte layout.
* `test_anchor_discriminator_for_onchain_account_info` fetches the **Marinade State** PDA (a non-executable Anchor account) and verifies its first 8 bytes match the Anchor *discriminator* for the struct `State`.

### Why the first 8 bytes matter (Anchor discriminator)

Anchor-based programs prefix every account with a **discriminator**:

```
discriminator = sha256("account:<StructName>")[..8]
```

Those 8 bytes uniquely identify the struct on-chain. `fetcher` already prints them for any data account it downloads.
In a **future** version we’ll **reverse-map** the discriminator to the struct name whenever the hash matches a known Anchor IDL, giving you an instant hint such as:

```
[fetcher] First 8 bytes (possible Anchor discriminator): 0xd8926b... -> looks like "State" struct name
```

This automatic recognition will only be possible for accounts that follow the Anchor convention; plain Borsh-only projects will continue to appear as raw bytes.

With these distinctions in mind you can:

* Pull down byte-code for offline disassembly (`.so`).
* Snapshot any on-chain state for local inspection or unit-test fixtures (`.bin`).
* Potentially confirm whether a PDA is an Anchor account and which struct it represents. (WIP)
