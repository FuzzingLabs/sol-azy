# `AppState` Architecture

The `AppState` struct acts as the central orchestrator for sol-azy’s CLI runtime.  
It coordinates the execution of commands like `build`, `sast` and `reverse`, and stores the resulting internal states across executions.

---

## Where It Lives

File: `src/state/app_state.rs`

---

## Responsibilities

- Parse and dispatch the correct CLI subcommand via `run_cli()`
- Track cumulative state (e.g., build results, SAST matches)
- Encapsulate application-wide control flow

---

## Structure

```rust
pub struct AppState {
    pub cli: Cli,
    pub build_states: Vec<BuildState>,
    pub sast_states: Vec<SastState>,
}
```

- `cli`: The parsed `Cli` object from Clap, holding user input
- `build_states`: Stores results of `build_command::run(...)`
- `sast_states`: Stores results of `sast_command::run(...)`

---

## Core Method: `run_cli`

This is the entrypoint for CLI usage:

```rust
pub fn run_cli(&mut self)
```

It matches on the selected `Commands` enum variant:

```rust
match &self.cli.command {
    Commands::Build { ... } => self.build_project(...),
    Commands::Sast { ... } => self.run_sast(...),
    Commands::Reverse { ... } => self.run_reverse(...),
    ...
}
```

Each arm delegates to a method that:

1. Executes the logic of the command
2. Logs the outcome
3. Updates the internal state vector (`build_states`, `sast_states`) → _except for the reverse command_

---

## Why is `AppState` needed?

sol-azy is a **multi-command CLI application**, and `AppState` provides:

- A consistent runtime container to track what’s been executed
- A clean separation of CLI logic from actual analysis logic

---

## Example Flow

```bash
cargo run -- build --target-dir myproj --out-dir myproj/out
```

Leads to:

1. `AppState::run_cli()`
2. → `AppState::build_project(...)`
3. → `build_command::run(...)`
4. → Result stored in `app_state.build_states`

---

## Related

- [CLI Overview](../cli/cli_usage.md)
- [Architecture](../architecture.md)