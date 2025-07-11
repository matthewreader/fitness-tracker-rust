# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Development Commands

- Build: `cargo build`
- Run: `cargo run`
- Test: `cargo test`
- Format: `cargo fmt`
- Lint: `cargo clippy`
- Build for release: `cargo build --release`

## Architecture

This is a Rust fitness tracker application built with:
- **Axum** for HTTP web framework
- **Tokio** for async runtime (full feature set enabled)
- Standard Rust project structure with `src/main.rs` as entry point

The project is currently in initial setup phase with a basic "Hello, world!" application.