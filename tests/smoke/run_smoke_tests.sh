#!/usr/bin/env bash

set -e
set -u

echo "=== Goose Smoke Test ==="

# Find the goose binary
if [ -f "target/release/goose" ]; then
    GOOSE_BINARY="./target/release/goose"
else
    echo "ERROR: goose binary not found at target/release/goose"
    exit 1
fi

# Check API keys based on provider
PROVIDER="${PROVIDERS:-anthropic}"
if [ "$PROVIDER" = "anthropic" ]; then
    if [ -z "${ANTHROPIC_API_KEY:-}" ]; then
        echo "ERROR: ANTHROPIC_API_KEY not set"
        exit 1
    fi
    MODEL="claude-sonnet-4-5-20250929"
elif [ "$PROVIDER" = "openai" ]; then
    if [ -z "${OPENAI_API_KEY:-}" ]; then
        echo "ERROR: OPENAI_API_KEY not set"
        exit 1
    fi
    MODEL="gpt-5"
else
    echo "ERROR: Unknown provider: $PROVIDER"
    exit 1
fi

# Setup isolated test environment
TEST_DIR=$(mktemp -d -t goose-smoke.XXXXXX)
trap "rm -rf $TEST_DIR" EXIT

export HOME="$TEST_DIR"
export GOOSE_DISABLE_KEYRING=1
export GOOSE_PROVIDER="$PROVIDER"
export GOOSE_MODEL="$MODEL"

# Create required directories
mkdir -p "$HOME/.local/share/goose/sessions"
mkdir -p "$HOME/.config/goose"

# Create test file
echo "hello" > "$TEST_DIR/hello.txt"

echo "Provider: $PROVIDER"
echo "Model: $MODEL"
echo "Binary: $GOOSE_BINARY"
echo "Test dir: $TEST_DIR"
echo ""

# Run the test
cd "$TEST_DIR"
echo "Running: $GOOSE_BINARY run --text 'please list files in dir $TEST_DIR' --with-builtin developer"
OUTPUT=$($GOOSE_BINARY run --text "please list files in dir $TEST_DIR" --with-builtin developer 2>&1)

echo "Output:"
echo "$OUTPUT"
echo ""

# Check if hello.txt appears in output
if echo "$OUTPUT" | grep -q "hello.txt"; then
    echo "✓ SUCCESS: Test passed - found hello.txt in output"
    exit 0
else
    echo "✗ FAILED: Test failed - hello.txt not found in output"
    exit 1
fi
