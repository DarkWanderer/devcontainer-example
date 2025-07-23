# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust project using Cargo with a simple structure:
- `src/main.rs` - Main application entry point with a basic "Hello, world!" program and an `add` function
- `tests/integration_test.rs` - Integration tests that verify the main program output
- Edition: 2024

## Development Commands

### Build
```bash
cargo build
```

### Run
```bash
cargo run
```

### Test
Run all tests (unit tests in src/main.rs and integration tests):
```bash
cargo test
```

Run only unit tests:
```bash
cargo test --lib
```

Run only integration tests:
```bash
cargo test --test integration_test
```

### Other useful commands
```bash
cargo check      # Fast compilation check without producing binaries
cargo clippy      # Linting
cargo fmt         # Code formatting
```

## Development Environment

This project uses a devcontainer with:
- Rust toolchain (mcr.microsoft.com/devcontainers/rust:1-1-bullseye)
- Claude Code extension
- Rust Analyzer extension
- GitHub CLI

The devcontainer configuration is in `.devcontainer/devcontainer.json`.

## Testing Strategy

The project has two types of tests:
1. Unit tests in `src/main.rs` (within `#[cfg(test)]` module)
2. Integration tests in `tests/integration_test.rs` that execute the binary and verify output

Integration tests use `std::process::Command` to run `cargo run` and verify the program output contains "Hello, world!".