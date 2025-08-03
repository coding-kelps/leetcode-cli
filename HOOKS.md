# Git Hooks Setup

This repository uses pre-commit and pre-push hooks to maintain code quality, matching the CI pipeline.

## Setup

Run the following commands to install and configure everything:

```bash
pipx install pre-commit
pre-commit install
git config core.hooksPath .githooks
chmod +x .githooks/pre-commit .githooks/pre-push
```

## What runs when

### Pre-commit hooks (on `git commit`)

- `trailing-whitespace` - Remove trailing whitespace
- `end-of-file-fixer` - Ensure files end with a newline
- `check-yaml` - Validate YAML files
- `check-toml` - Validate TOML files
- `cargo +nightly fmt --check` - Check code formatting
- `cargo clippy --all-features -- -D warnings` - Lint code
- `cargo build` - Ensure code compiles

### Pre-push hooks (on `git push`)

- `cargo test` - Run all tests
- `cargo +nightly fmt --check` - Check code formatting
- `cargo clippy --all-features -- -D warnings` - Lint code

## Manual execution

```bash

pre-commit run --all-files

./.githooks/pre-push
```

## Configuration files

- `.pre-commit-config.yaml` - Pre-commit configuration
- `.githooks/pre-commit` - Pre-commit hook script
- `.githooks/pre-push` - Pre-push hook script
