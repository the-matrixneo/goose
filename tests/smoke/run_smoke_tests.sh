#!/usr/bin/env bash

# Smoke tests for goose binary with multiple providers
# Usage: ./run_smoke_tests.sh
# Requires: ANTHROPIC_API_KEY and OPENAI_API_KEY environment variables

set -e  # Exit on error
set -u  # Exit on undefined variable

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
TIMEOUT=${TIMEOUT:-60}
VERBOSE=${VERBOSE:-0}
PROVIDERS=${PROVIDERS:-"anthropic openai"}

# Get script directory and repo root
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
# Look for binary in multiple locations (for CI sparse checkout)
if [ -f "${REPO_ROOT}/target/release/goose" ]; then
    GOOSE_BINARY="${REPO_ROOT}/target/release/goose"
elif [ -f "target/release/goose" ]; then
    GOOSE_BINARY="$(pwd)/target/release/goose"
else
    GOOSE_BINARY="${REPO_ROOT}/target/release/goose"
fi

# Temporary directory for test artifacts
TEST_TEMP_DIR=""

# Test counters
TESTS_RUN=0
TESTS_PASSED=0
TESTS_FAILED=0

# Cleanup function
cleanup() {
    if [ -n "${TEST_TEMP_DIR}" ] && [ -d "${TEST_TEMP_DIR}" ]; then
        rm -rf "${TEST_TEMP_DIR}"
    fi
}

trap cleanup EXIT

# Logging functions
log_info() {
    echo -e "${GREEN}[INFO]${NC} $*"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $*"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $*"
}

log_verbose() {
    if [ "${VERBOSE}" = "1" ]; then
        echo -e "${YELLOW}[VERBOSE]${NC} $*"
    fi
}

# Setup test environment
setup_test_env() {
    # Create temporary directory for this test run
    TEST_TEMP_DIR=$(mktemp -d -t goose-smoke-test.XXXXXX)
    log_verbose "Created temp directory: ${TEST_TEMP_DIR}"
    
    # Set environment variables to use temp directory
    export HOME="${TEST_TEMP_DIR}"
    export GOOSE_DISABLE_KEYRING=1
    
    # Create config directory (config will be swapped per test)
    mkdir -p "${HOME}/.config/goose"
    
    log_verbose "Test environment configured"
}

# Verify prerequisites
check_prerequisites() {
    log_info "Checking prerequisites..."
    
    # Check if binary exists
    if [ ! -f "${GOOSE_BINARY}" ]; then
        log_error "Goose binary not found at: ${GOOSE_BINARY}"
        log_error "Please build it first: cargo build --release"
        exit 1
    fi
    log_verbose "✓ Binary found: ${GOOSE_BINARY}"
    
    # Check for required environment variables based on selected providers
    if echo "${PROVIDERS}" | grep -q "anthropic"; then
        if [ -z "${ANTHROPIC_API_KEY:-}" ]; then
            log_error "ANTHROPIC_API_KEY environment variable not set"
            exit 1
        fi
        log_verbose "✓ ANTHROPIC_API_KEY is set"
    fi
    
    if echo "${PROVIDERS}" | grep -q "openai"; then
        if [ -z "${OPENAI_API_KEY:-}" ]; then
            log_error "OPENAI_API_KEY environment variable not set"
            exit 1
        fi
        log_verbose "✓ OPENAI_API_KEY is set"
    fi
    
    log_info "All prerequisites met"
}

# Run a single test
run_test() {
    local test_name="$1"
    local provider="$2"
    local prompt="$3"
    local expected_pattern="$4"
    local use_developer="${5:-no}"
    
    TESTS_RUN=$((TESTS_RUN + 1))
    
    log_info "Running test: ${test_name}"
    log_verbose "  Provider: ${provider}"
    log_verbose "  Prompt: ${prompt}"
    
    # Build command (model comes from config file)
    local cmd="${GOOSE_BINARY} run --text \"${prompt}\" --no-session --provider ${provider}"
    
    # Add developer extension if requested
    if [ "${use_developer}" = "yes" ]; then
        cmd="${cmd} --with-builtin developer"
        log_verbose "  Using developer extension"
    fi
    
    # Create output file for this test
    local output_file="${TEST_TEMP_DIR}/test_${TESTS_RUN}.out"
    
    log_verbose "  Running command: ${cmd}"
    
    # Run the command with timeout (use gtimeout on macOS if available, otherwise skip timeout)
    local timeout_cmd=""
    if command -v timeout >/dev/null 2>&1; then
        timeout_cmd="timeout ${TIMEOUT}"
    elif command -v gtimeout >/dev/null 2>&1; then
        timeout_cmd="gtimeout ${TIMEOUT}"
    fi
    
    if [ -n "${timeout_cmd}" ]; then
        eval "${timeout_cmd} bash -c \"${cmd}\"" > "${output_file}" 2>&1 || local exit_code=$?
        [ -z "${exit_code:-}" ] && local exit_code=0
    else
        # No timeout available, run without it
        bash -c "${cmd}" > "${output_file}" 2>&1 || local exit_code=$?
        [ -z "${exit_code:-}" ] && local exit_code=0
    fi
    
    if [ ${exit_code} -eq 0 ]; then
        log_verbose "  Command completed successfully"
        
        # Check if output matches expected pattern
        if grep -q "${expected_pattern}" "${output_file}"; then
            log_info "✓ Test passed: ${test_name}"
            TESTS_PASSED=$((TESTS_PASSED + 1))
            
            if [ "${VERBOSE}" = "1" ]; then
                log_verbose "Output:"
                cat "${output_file}"
            fi
            return 0
        else
            log_error "✗ Test failed: ${test_name}"
            log_error "  Expected pattern not found: ${expected_pattern}"
            log_error "  Output:"
            cat "${output_file}"
            TESTS_FAILED=$((TESTS_FAILED + 1))
            return 1
        fi
    else
        local exit_code=$?
        log_error "✗ Test failed: ${test_name}"
        log_error "  Command failed with exit code: ${exit_code}"
        log_error "  Output:"
        cat "${output_file}"
        TESTS_FAILED=$((TESTS_FAILED + 1))
        return 1
    fi
}

# Smoke test: Developer extension with file listing
test_developer_extension() {
    local provider="$1"
    
    # Swap in the provider-specific config
    local config_file="${SCRIPT_DIR}/config.${provider}.yaml"
    if [ -f "${config_file}" ]; then
        cp "${config_file}" "${HOME}/.config/goose/config.yaml"
        log_verbose "Swapped in config for ${provider}"
    else
        log_error "Config file not found: ${config_file}"
        return 1
    fi
    
    # Create hello.txt in temp directory
    echo "hello" > "${TEST_TEMP_DIR}/hello.txt"
    
    # Change to temp directory and run test
    cd "${TEST_TEMP_DIR}"
    
    run_test \
        "Developer extension smoke test (${provider})" \
        "${provider}" \
        "list files" \
        "hello.txt" \
        "yes"
    
    cd "${REPO_ROOT}"
}

# Run all tests for a provider
run_provider_tests() {
    local provider="$1"
    
    log_info "========================================="
    log_info "Testing provider: ${provider}"
    log_info "========================================="
    
    test_developer_extension "${provider}"
    
    echo ""
}

# Main execution
main() {
    log_info "Starting Goose smoke tests"
    log_info "========================================="
    
    check_prerequisites
    setup_test_env
    
    # Run tests for each provider
    if echo "${PROVIDERS}" | grep -q "anthropic"; then
        run_provider_tests "anthropic"
    fi
    
    if echo "${PROVIDERS}" | grep -q "openai"; then
        run_provider_tests "openai"
    fi
    
    # Print summary
    echo ""
    log_info "========================================="
    log_info "Test Summary"
    log_info "========================================="
    log_info "Total tests run: ${TESTS_RUN}"
    log_info "Tests passed: ${TESTS_PASSED}"
    log_info "Tests failed: ${TESTS_FAILED}"
    
    if [ "${TESTS_FAILED}" -eq 0 ]; then
        log_info "✓ All tests passed!"
        exit 0
    else
        log_error "✗ Some tests failed"
        exit 1
    fi
}

main "$@"
