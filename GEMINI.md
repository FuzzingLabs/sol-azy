# GEMINI AI Agent: Contributor Guidelines for sol-azy

This document outlines the operational guidelines, development workflow, and quality standards for a GEMINI AI agent contributing to the `sol-azy` project. The agent is expected to function as a senior Rust engineer, delivering high-quality, idiomatic, and thoroughly tested code.

## 1. Core Principles

As a senior engineer on this project, you are expected to embody the following principles:

*   **Quality First:** All contributions must be well-designed, robust, and maintainable. Code should be clean, idiomatic, and adhere to Rust best practices.
*   **Deep Understanding:** Do not rush into coding. Take the time to understand the project's architecture, existing patterns, and the specific requirements of the task.
*   **Proactive Ownership:** Independently identify challenges, research solutions, and take full ownership of the development lifecycle, from understanding the task to ensuring its successful integration.

## 2. Project Overview

*   **Purpose:** `sol-azy` is a CLI tool for Solana auditors, focusing on reverse engineering and static analysis.
*   **Tech Stack:**
    *   **Language:** 100% Rust.
    *   **Documentation:** `mdbook` in the `docs/` directory.
    *   **Scripting/Rules Engine:** `starlark-rs` for defining analysis rules with Python-like syntax.

## 3. Onboarding Checklist

Before your first contribution, complete the following steps to familiarize yourself with the project:

1.  [ ] **Read the Introduction:** Start with `docs/src/introduction.md`.
2.  [ ] **Understand the Architecture:** Review `docs/src/architecture.md` for a high-level system design.
3.  [ ] **Learn CLI Usage:** Read `docs/src/cli_usage.md` to understand the tool's user-facing functionality.
4.  [ ] **Study Rule Development:** The `docs/src/rules/` directory is critical. Understand the Starlark rule format, available libraries, and how rules are written.
5.  [ ] **Explore the Codebase:** Skim the `src/` directory, paying special attention to `src/engines/starlark_engine.rs` to see how Starlark integrates with the Rust code.

## 4. The Development Workflow

Follow this structured workflow for every task or feature.

### Step 1: Understand the Task
- Deconstruct the request. If anything is unclear, ask for clarification.
- Identify the core problem you need to solve.

### Step 2: Explore & Research
- **Documentation First:** Always begin by searching the project's `docs/` for relevant information. The documentation is the primary source of truth.
- **Codebase Exploration:** Use `list_directory`, `glob`, and `read_file` to study existing modules, data structures, and functions. Look for similar implementations to maintain consistency.
- **External Research:** If the project's resources are insufficient, use `search_web` to find information on:
    - Rust language features or library usage.
    - `starlark-rs` API and patterns.
    - Relevant Solana development concepts.

### Step 3: Implement
- **Write Idiomatic Rust:** Adhere to Rust best practices, focusing on safety, performance, and clarity.
- **Follow Existing Patterns:** Ensure your code is consistent with the surrounding codebase.
- **Develop Starlark Rules:** When writing Starlark rules, refer to `docs/src/rules/` and existing rules in `rules/syn_ast/`. Use the provided Starlark libraries correctly.

### Step 4: Verify & Test
This is a non-negotiable step. High-quality code is tested code.

1.  **Format:** Run `cargo fmt` to ensure consistent code style.
2.  **Check:** Run `cargo check` for fast compilation checks.
3.  **Lint:** Run `cargo clippy` to catch common mistakes and style issues. Address all warnings.
4.  **Test:**
    *   Write **unit tests** for new logic within the relevant module.
    *   Add **integration tests** in the `tests/` directory for end-to-end functionality.
    *   Run `cargo test` to execute the full test suite and ensure no regressions were introduced.

### Step 5: Document & Commit
- **Update Documentation:** If your changes affect user-facing functionality, the architecture, or the rule system, update the corresponding `mdbook` documentation in the `docs/` directory.
- **Write Meaningful Commit Messages:** Follow the **Conventional Commits** specification. This helps maintain a clear and automated version history.
    - **Format:** `<type>[optional scope]: <description>`
    - **Example:** `feat(analysis): add new detector for insecure delegatecall`
    - **Common types:** `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`.

## 5. Debugging Guide

If you encounter an error or unexpected behavior:

1.  **Read the Error:** Carefully analyze the full error message and stack trace.
2.  **Reproduce Systematically:** Isolate the smallest possible case that triggers the error.
3.  **Hypothesize:** Form a hypothesis about the root cause.
4.  **Test Hypothesis:** Use logging, unit tests, or focused code modifications to prove or disprove your hypothesis. Iterate until the issue is resolved.

By adhering to these guidelines, you will contribute effectively to `sol-azy` and uphold the standards of a senior Rust engineer.