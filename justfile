# after-effects justfile
# Run `just --list` to see all available commands.
#
# NOTE: this workspace does NOT build on native Linux (the Adobe SDK FFI
# targets Windows/macOS). The compiling recipes therefore default to the
# Windows target `x86_64-pc-windows-gnu`, which only needs the target
# installed (`rustup target add x86_64-pc-windows-gnu`) -- no linker/SDK.
# Override the target from the command line, e.g.:
#   just TARGET=x86_64-apple-darwin check
TARGET := "x86_64-pc-windows-gnu"

# Default recipe: show available commands.
default:
    @just --list

# Common aliases.
alias c := check
alias f := fmt
alias l := lint

# Aggregate: what CI runs. Uses non-fixing variants.
ci: fmt-check check lint-check test

# Type-check the whole workspace (all targets) for the chosen target.
check:
    cargo check --workspace --all-targets --target {{TARGET}}

# Run clippy with autofix (modifies working tree).
lint:
    cargo clippy --fix --allow-dirty --allow-staged --workspace --all-targets --target {{TARGET}} -- -D warnings

# Advisory today (pre-existing backlog): does not yet pass with `-D warnings`.
lint-check:
    cargo clippy --workspace --all-targets --target {{TARGET}}

# Format code (modifies working tree). Needs no compile, so no target.
fmt:
    cargo fmt --all

# Verify formatting without writing (CI-safe). Needs no compile.
fmt-check:
    cargo fmt --all -- --check

# Run the test suite for the chosen target.
test:
    cargo test --workspace --target {{TARGET}}
