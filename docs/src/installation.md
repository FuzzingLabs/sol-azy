# Installation

This page describes how to set up sol-azy and its required dependencies.

---

## 1. Prerequisites

Make sure the following tools are installed on your system:

| Tool         | Purpose                     | Install link / command                     |
|--------------|-----------------------------|--------------------------------------------|
| **Rust**     | Required to compile sol-azy | https://rustup.rs                          |
| **cargo**    | Rust package manager        | Included with `rustup`                     |
| **Solana CLI** | Needed for SBF builds       | https://docs.solana.com/cli/install-solana-cli |
| **anchor** *(optional)* | For Anchor-based projects   | `cargo install --git https://github.com/coral-xyz/anchor anchor-cli --locked` |

Verify installations:

```bash
rustc --version
cargo --version
solana --version
anchor --version # optional
```

---

## 2. Clone the Repository

```bash
git clone https://github.com/FuzzingLabs/sol-azy
cd sol-azy
```

---

## 3. Build the Tool

```bash
cargo build --release
```

The binary will be available at:

```bash
./target/release/sol-azy
```

You can also run sol-azy in development using:

```bash
cargo run -- <command> [options]
```

---

## 4. Install `mdBook` (optional)

To build or view the documentation locally:

```bash
cargo install mdbook
mdbook serve docs
```

Then open [http://localhost:3000](http://localhost:3000)

---

Certainly! Here's a "Known Issues & Troubleshooting" section that addresses the `Cargo.lock` version mismatch and related errors when building Solana programs with Anchor:

---

## Known Issues & Troubleshooting

### ⚠️ `Cargo.lock` Version Mismatch Error

When running the `build` command, you might encounter the following error:

```
error: failed to parse lock file at: ...
Caused by: lock file version 4 requires -Znext-lockfile-bump
```

#### Root Cause

This issue arises due to a mismatch between the `Cargo.lock` file version and the Rust compiler version used by Solana's build tools. Specifically:

- `Cargo.lock` version 4 is generated by newer versions of Cargo and requires `rustc` 1.78 or newer.
- However, Solana's `cargo-build-sbf` and `anchor build` commands may use an older `rustc` version (e.g., 1.75), leading to this incompatibility.

This discrepancy occurs because Solana's build tools bundle their own Rust toolchain, which might not match the system's Rust version managed by `rustup`.

#### Solutions

1. **Update Solana CLI and Anchor**

   Ensure you're using compatible versions of Solana and Anchor that support the newer `Cargo.lock` format:

   ```bash
   # Update Solana CLI to version 2.1.x or newer
   sh -c "$(curl -sSfL https://release.anza.xyz/v2.1.0/install)"
   # Update Anchor CLI
   cargo install --git https://github.com/coral-xyz/anchor avm --locked
   avm install latest
   avm use latest
   ```

   These updates align the toolchains with the expected `Cargo.lock` version and Rust compiler requirements.

2. **Manually Downgrade `Cargo.lock` Version (Temporary Workaround)**

   If updating is not feasible, you can temporarily modify the `Cargo.lock` file:

   - Open `Cargo.lock` in your project root.
   - Change the version line from:

     ```toml
     version = 4
     ```

     to:

     ```toml
     version = 3
     ```

   **Note:** This is a temporary fix. Running `cargo update` or similar commands may regenerate the `Cargo.lock` file with version 4.

3. **Ensure Consistent Rust Toolchain**

   Verify that the Rust version used by Solana's build tools matches the required version:

   ```bash
   # Check Rust version used by Solana's cargo-build-sbf
   cargo-build-sbf --version
   ```

   If the version is older than required, updating the Solana CLI as shown above should resolve the issue.

#### Additional Resources

- [Anchor Issue #3392](https://github.com/solana-foundation/anchor/issues/3392)
- [Solana Stack Exchange Discussion](https://solana.stackexchange.com/questions/18620/lock-file-version-4-was-found-but-this-version-of-cargo-does-not-understand-t)

By following these steps, you should be able to resolve the `Cargo.lock` version mismatch error and continue building your Solana programs successfully.

--- 

## Need Help?

If something doesn't work, check:

- Error messages in the CLI output
- That `cargo`, `solana`, or `anchor` are in your `PATH`
- That the bytecode you are reversing is a valid `.so` file, for instance:
```sh
test_cases/base_sbf_addition_checker/bytecodes/addition_checker.so: ELF 64-bit LSB shared object, eBPF, version 1 (SYSV), dynamically linked, stripped
test_cases/base_sbf_addition_checker/bytecodes/addition_checker_sbpf_solana.so: ELF 64-bit LSB shared object, eBPF, version 1 (SYSV), dynamically linked, not stripped
```
You can also open an issue or contact the maintainers.