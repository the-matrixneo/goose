# Goose Smoke Tests

Simple smoke test for the goose binary with multiple providers.

## Quick Start

```bash
# Set API keys
export ANTHROPIC_API_KEY="sk-ant-..."
export OPENAI_API_KEY="sk-..."

# Build binary
cargo build --release

# Run tests
./tests/smoke/run_smoke_tests.sh
```

## What It Tests

Creates a `hello.txt` file in a temp directory, runs goose with `--with-builtin developer`, asks it to "list files", and verifies the output contains "hello.txt".

Tests that:
- The binary runs with API keys from environment variables
- The developer extension loads via `--with-builtin developer`  
- Basic tool usage (shell) works

## Options

```bash
# Test specific provider
PROVIDERS="anthropic" ./tests/smoke/run_smoke_tests.sh

# Verbose output
VERBOSE=1 ./tests/smoke/run_smoke_tests.sh
```

## CI

Add GitHub secrets `ANTHROPIC_API_KEY` and `OPENAI_API_KEY`. Workflow runs automatically on push/PR.
