# Fetcher

The `fetcher` command allows you to retrieve the deployed bytecode of a Solana program and save it locally as `fetched_program.so`.

This is useful for performing offline analysis, reverse engineering, or static checks without relying on local source code or Solana toolchain.

## Usage

```sh
cargo run -- fetcher \
  --program-id <PROGRAM_ID> \
  --out-file <OUTPUT_DIR> \
  [--rpc-url <CUSTOM_RPC_ENDPOINT>]
````

* `--program-id`: The Solana program ID to fetch.
* `--out-file`: Directory where the bytecode file will be saved (as `fetched_program.so`).
* `--rpc-url`: (Optional) Custom Solana RPC endpoint. Defaults to `https://api.mainnet-beta.solana.com`.

## Behavior

* Checks if the output directory exists (if not it creates the folder).
* Validates the program exists on-chain and is executable.
* Writes the bytecode to the specified directory.
* Logs the output file path & the RPC used, including when default is applied.

## Example

```sh
cargo run -- fetcher \
  --program-id 4MangoMjqJ2firMokCjjGgoK8d4MXcrgL7XJaL3w6fVg \
  --out-file ./out
```

This will fetch the bytecode of the program and save it to `./out/fetched_program.so`.