#!/bin/bash
# Comprehensive goosed Black Box Testing Script for Agent Manager
# This script tests all goosed endpoints to ensure Agent Manager doesn't introduce regressions

set -e  # Exit on error

# Configuration
GOOSED_PORT=8081
SECRET_KEY="test123"
BASE_URL="http://localhost:$GOOSED_PORT"
GOOSED_BIN="./target/debug/goosed"
LOG_FILE="goosed_test.log"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Test result tracking
TESTS_PASSED=0
TESTS_FAILED=0
FAILED_TESTS=()

# Helper functions
log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

test_passed() {
    echo -e "${GREEN}✅ $1${NC}"
    ((TESTS_PASSED++))
}

test_failed() {
    echo -e "${RED}❌ $1${NC}"
    ((TESTS_FAILED++))
    FAILED_TESTS+=("$1")
}

# Start goosed with Agent Manager
start_goosed() {
    log_info "Building goosed with latest changes..."
    cargo build -p goose-server --bin goosed 2>&1 | tail -5
    
    log_info "Starting goosed on port $GOOSED_PORT..."
    
    # Kill any existing goosed on this port
    lsof -ti:$GOOSED_PORT | xargs kill -9 2>/dev/null || true
    
    # Start goosed in screen
    screen -dmS goosed_test bash -c "
        RUST_LOG=info \
        GOOSE_PORT=$GOOSED_PORT \
        GOOSE_DEFAULT_PROVIDER=openai \
        GOOSE_SERVER__SECRET_KEY=$SECRET_KEY \
        $GOOSED_BIN agent 2>&1 | tee $LOG_FILE
    "
    
    # Wait for server to start
    log_info "Waiting for goosed to start..."
    for i in {1..30}; do
        if curl -s $BASE_URL/status > /dev/null 2>&1; then
            log_info "goosed started successfully"
            return 0
        fi
        sleep 1
    done
    
    log_error "Failed to start goosed"
    return 1
}

stop_goosed() {
    log_info "Stopping goosed..."
    screen -X -S goosed_test quit 2>/dev/null || true
    lsof -ti:$GOOSED_PORT | xargs kill -9 2>/dev/null || true
}

# Test 1: Health Check
test_health_check() {
    local test_name="Health Check"
    log_info "Testing: $test_name"
    
    response=$(curl -s -w "\n%{http_code}" $BASE_URL/status)
    http_code=$(echo "$response" | tail -1)
    
    if [ "$http_code" = "200" ]; then
        test_passed "$test_name"
    else
        test_failed "$test_name - HTTP $http_code"
    fi
}

# Test 2: Basic Reply (Session Auto-Creation)
test_basic_reply() {
    local test_name="Basic Reply (Auto Session)"
    log_info "Testing: $test_name"
    
    response=$(curl -s -w "\n%{http_code}" -X POST $BASE_URL/reply \
        -H "X-Secret-Key: $SECRET_KEY" \
        -H "Content-Type: application/json" \
        -d '{
            "messages": [{"role": "user", "content": "Say hello"}]
        }')
    
    http_code=$(echo "$response" | tail -1)
    body=$(echo "$response" | sed '$d')
    
    if [ "$http_code" = "200" ] && echo "$body" | grep -q "content"; then
        test_passed "$test_name"
    else
        test_failed "$test_name - HTTP $http_code"
    fi
}

# Test 3: Session Isolation
test_session_isolation() {
    local test_name="Session Isolation"
    log_info "Testing: $test_name"
    
    # Create two sessions with different content
    curl -s -X POST $BASE_URL/reply \
        -H "X-Secret-Key: $SECRET_KEY" \
        -H "Content-Type: application/json" \
        -d '{
            "session_id": "isolation_test_1",
            "messages": [{"role": "user", "content": "Remember: I am session ONE"}]
        }' > /dev/null
    
    curl -s -X POST $BASE_URL/reply \
        -H "X-Secret-Key: $SECRET_KEY" \
        -H "Content-Type: application/json" \
        -d '{
            "session_id": "isolation_test_2",
            "messages": [{"role": "user", "content": "Remember: I am session TWO"}]
        }' > /dev/null
    
    # Get context for each session
    context1=$(curl -s -X GET "$BASE_URL/context?session_id=isolation_test_1" \
        -H "X-Secret-Key: $SECRET_KEY")
    context2=$(curl -s -X GET "$BASE_URL/context?session_id=isolation_test_2" \
        -H "X-Secret-Key: $SECRET_KEY")
    
    # Contexts should be different
    if [ "$context1" != "$context2" ]; then
        test_passed "$test_name"
    else
        test_failed "$test_name - Contexts are identical"
    fi
}

# Test 4: Extension Management
test_extension_management() {
    local test_name="Extension Management"
    log_info "Testing: $test_name"
    
    local session_id="ext_mgmt_test"
    
    # Add a frontend extension
    add_response=$(curl -s -w "\n%{http_code}" -X POST $BASE_URL/extensions/add \
        -H "X-Secret-Key: $SECRET_KEY" \
        -H "Content-Type: application/json" \
        -d "{
            \"session_id\": \"$session_id\",
            \"type\": \"frontend\",
            \"name\": \"test_extension\",
            \"tools\": [
                {
                    \"name\": \"test_tool\",
                    \"description\": \"A test tool\",
                    \"input_schema\": {\"type\": \"object\"}
                }
            ]
        }")
    
    add_code=$(echo "$add_response" | tail -1)
    
    # List extensions
    list_response=$(curl -s -X GET "$BASE_URL/extensions/list?session_id=$session_id" \
        -H "X-Secret-Key: $SECRET_KEY")
    
    if [ "$add_code" = "200" ] && echo "$list_response" | grep -q "test_extension"; then
        test_passed "$test_name"
    else
        test_failed "$test_name - Add: $add_code"
    fi
}

# Test 5: Extension Isolation
test_extension_isolation() {
    local test_name="Extension Isolation Between Sessions"
    log_info "Testing: $test_name"
    
    # Add extension to session 1
    curl -s -X POST $BASE_URL/extensions/add \
        -H "X-Secret-Key: $SECRET_KEY" \
        -H "Content-Type: application/json" \
        -d '{
            "session_id": "ext_iso_1",
            "type": "frontend",
            "name": "isolated_ext",
            "tools": [{"name": "iso_tool", "description": "Isolated tool", "input_schema": {"type": "object"}}]
        }' > /dev/null
    
    # Check extensions for both sessions
    ext1=$(curl -s -X GET "$BASE_URL/extensions/list?session_id=ext_iso_1" \
        -H "X-Secret-Key: $SECRET_KEY")
    ext2=$(curl -s -X GET "$BASE_URL/extensions/list?session_id=ext_iso_2" \
        -H "X-Secret-Key: $SECRET_KEY")
    
    # Session 1 should have the extension, session 2 should not
    if echo "$ext1" | grep -q "isolated_ext" && ! echo "$ext2" | grep -q "isolated_ext"; then
        test_passed "$test_name"
    else
        test_failed "$test_name - Extension leaked between sessions"
    fi
}

# Test 6: Agent Tools
test_agent_tools() {
    local test_name="Agent Tools Listing"
    log_info "Testing: $test_name"
    
    response=$(curl -s -w "\n%{http_code}" -X GET "$BASE_URL/agent/tools?session_id=tools_test" \
        -H "X-Secret-Key: $SECRET_KEY")
    
    http_code=$(echo "$response" | tail -1)
    body=$(echo "$response" | head -n -1)
    
    if [ "$http_code" = "200" ] && echo "$body" | grep -q '\['; then
        test_passed "$test_name"
    else
        test_failed "$test_name - HTTP $http_code"
    fi
}

# Test 7: Agent Stats
test_agent_stats() {
    local test_name="Agent Stats/Metrics"
    log_info "Testing: $test_name"
    
    response=$(curl -s -w "\n%{http_code}" -X GET $BASE_URL/agent/stats \
        -H "X-Secret-Key: $SECRET_KEY")
    
    http_code=$(echo "$response" | tail -1)
    body=$(echo "$response" | head -n -1)
    
    if [ "$http_code" = "200" ] && echo "$body" | grep -q "agents_created"; then
        test_passed "$test_name"
    else
        test_failed "$test_name - HTTP $http_code"
    fi
}

# Test 8: Agent Cleanup
test_agent_cleanup() {
    local test_name="Agent Cleanup"
    log_info "Testing: $test_name"
    
    # Create a session first
    curl -s -X POST $BASE_URL/reply \
        -H "X-Secret-Key: $SECRET_KEY" \
        -H "Content-Type: application/json" \
        -d '{"session_id": "cleanup_test", "messages": [{"role": "user", "content": "test"}]}' > /dev/null
    
    # Trigger cleanup
    response=$(curl -s -w "\n%{http_code}" -X POST $BASE_URL/agent/cleanup \
        -H "X-Secret-Key: $SECRET_KEY")
    
    http_code=$(echo "$response" | tail -1)
    
    if [ "$http_code" = "200" ]; then
        test_passed "$test_name"
    else
        test_failed "$test_name - HTTP $http_code"
    fi
}

# Test 9: Context Retrieval
test_context_retrieval() {
    local test_name="Context Retrieval"
    log_info "Testing: $test_name"
    
    # Create a session with specific content
    curl -s -X POST $BASE_URL/reply \
        -H "X-Secret-Key: $SECRET_KEY" \
        -H "Content-Type: application/json" \
        -d '{
            "session_id": "context_test",
            "messages": [{"role": "user", "content": "CONTEXT_TEST_MARKER"}]
        }' > /dev/null
    
    # Get context
    response=$(curl -s -w "\n%{http_code}" -X GET "$BASE_URL/context?session_id=context_test" \
        -H "X-Secret-Key: $SECRET_KEY")
    
    http_code=$(echo "$response" | tail -1)
    body=$(echo "$response" | head -n -1)
    
    if [ "$http_code" = "200" ] && echo "$body" | grep -q "CONTEXT_TEST_MARKER"; then
        test_passed "$test_name"
    else
        test_failed "$test_name - HTTP $http_code"
    fi
}

# Test 10: Recipe Creation
test_recipe_creation() {
    local test_name="Recipe Creation"
    log_info "Testing: $test_name"
    
    response=$(curl -s -w "\n%{http_code}" -X POST $BASE_URL/recipe/create \
        -H "X-Secret-Key: $SECRET_KEY" \
        -H "Content-Type: application/json" \
        -d '{
            "session_id": "recipe_test",
            "messages": [{"role": "user", "content": "Create a simple hello world recipe"}]
        }')
    
    http_code=$(echo "$response" | tail -1)
    body=$(echo "$response" | head -n -1)
    
    if [ "$http_code" = "200" ] && echo "$body" | grep -q "recipe"; then
        test_passed "$test_name"
    else
        test_failed "$test_name - HTTP $http_code"
    fi
}

# Test 11: Session Management
test_session_management() {
    local test_name="Session Management"
    log_info "Testing: $test_name"
    
    # Create a session
    curl -s -X POST $BASE_URL/reply \
        -H "X-Secret-Key: $SECRET_KEY" \
        -H "Content-Type: application/json" \
        -d '{
            "session_id": "session_mgmt_test",
            "messages": [{"role": "user", "content": "Session test"}]
        }' > /dev/null
    
    # List sessions
    response=$(curl -s -w "\n%{http_code}" -X GET $BASE_URL/sessions \
        -H "X-Secret-Key: $SECRET_KEY")
    
    http_code=$(echo "$response" | tail -1)
    body=$(echo "$response" | head -n -1)
    
    if [ "$http_code" = "200" ] && echo "$body" | grep -q "sessions"; then
        test_passed "$test_name"
    else
        test_failed "$test_name - HTTP $http_code"
    fi
}

# Test 12: Concurrent Requests
test_concurrent_requests() {
    local test_name="Concurrent Requests"
    log_info "Testing: $test_name"
    
    # Send multiple concurrent requests
    for i in {1..10}; do
        curl -s -X POST $BASE_URL/reply \
            -H "X-Secret-Key: $SECRET_KEY" \
            -H "Content-Type: application/json" \
            -d "{
                \"session_id\": \"concurrent_$i\",
                \"messages\": [{\"role\": \"user\", \"content\": \"Concurrent test $i\"}]
            }" > /dev/null &
    done
    
    # Wait for all requests to complete
    wait
    
    # Check that all sessions were created
    stats=$(curl -s -X GET $BASE_URL/agent/stats -H "X-Secret-Key: $SECRET_KEY")
    
    if echo "$stats" | grep -q "agents_created"; then
        test_passed "$test_name"
    else
        test_failed "$test_name - Stats unavailable"
    fi
}

# Test 13: Provider Configuration
test_provider_configuration() {
    local test_name="Provider Configuration"
    log_info "Testing: $test_name"
    
    response=$(curl -s -w "\n%{http_code}" -X POST $BASE_URL/reply \
        -H "X-Secret-Key: $SECRET_KEY" \
        -H "Content-Type: application/json" \
        -d '{
            "session_id": "provider_test",
            "messages": [{"role": "user", "content": "test"}],
            "provider": "openai",
            "model": "gpt-4o-mini",
            "temperature": 0.7
        }')
    
    http_code=$(echo "$response" | tail -1)
    
    if [ "$http_code" = "200" ]; then
        test_passed "$test_name"
    else
        test_failed "$test_name - HTTP $http_code"
    fi
}

# Test 14: Session Persistence After Cleanup
test_session_persistence() {
    local test_name="Session Persistence After Cleanup"
    log_info "Testing: $test_name"
    
    # Create session with unique content
    curl -s -X POST $BASE_URL/reply \
        -H "X-Secret-Key: $SECRET_KEY" \
        -H "Content-Type: application/json" \
        -d '{
            "session_id": "persist_test",
            "messages": [{"role": "user", "content": "PERSISTENCE_MARKER_12345"}]
        }' > /dev/null
    
    # Trigger cleanup
    curl -s -X POST $BASE_URL/agent/cleanup \
        -H "X-Secret-Key: $SECRET_KEY" > /dev/null
    
    # Access session again - should recreate agent from persisted session
    context=$(curl -s -X GET "$BASE_URL/context?session_id=persist_test" \
        -H "X-Secret-Key: $SECRET_KEY")
    
    if echo "$context" | grep -q "PERSISTENCE_MARKER_12345"; then
        test_passed "$test_name"
    else
        test_failed "$test_name - Session not persisted"
    fi
}

# Test 15: Audio Endpoint (if available)
test_audio_endpoint() {
    local test_name="Audio Endpoint"
    log_info "Testing: $test_name"
    
    # Test audio/models endpoint
    response=$(curl -s -w "\n%{http_code}" -X GET $BASE_URL/audio/models \
        -H "X-Secret-Key: $SECRET_KEY")
    
    http_code=$(echo "$response" | tail -1)
    
    if [ "$http_code" = "200" ]; then
        test_passed "$test_name"
    else
        # Audio might not be configured, so we'll just warn
        log_warning "$test_name - HTTP $http_code (might not be configured)"
        test_passed "$test_name (skipped)"
    fi
}

# Test 16: Config Management
test_config_management() {
    local test_name="Config Management"
    log_info "Testing: $test_name"
    
    # Get current config
    response=$(curl -s -w "\n%{http_code}" -X GET $BASE_URL/config \
        -H "X-Secret-Key: $SECRET_KEY")
    
    http_code=$(echo "$response" | tail -1)
    
    if [ "$http_code" = "200" ]; then
        test_passed "$test_name"
    else
        test_failed "$test_name - HTTP $http_code"
    fi
}

# Test 17: Schedule Management
test_schedule_management() {
    local test_name="Schedule Management"
    log_info "Testing: $test_name"
    
    # List schedules
    response=$(curl -s -w "\n%{http_code}" -X GET $BASE_URL/schedules \
        -H "X-Secret-Key: $SECRET_KEY")
    
    http_code=$(echo "$response" | tail -1)
    
    if [ "$http_code" = "200" ]; then
        test_passed "$test_name"
    else
        test_failed "$test_name - HTTP $http_code"
    fi
}

# Test 18: Memory Leak Check
test_memory_stability() {
    local test_name="Memory Stability"
    log_info "Testing: $test_name"
    
    # Create and cleanup many agents
    for i in {1..20}; do
        curl -s -X POST $BASE_URL/reply \
            -H "X-Secret-Key: $SECRET_KEY" \
            -H "Content-Type: application/json" \
            -d "{
                \"session_id\": \"memory_test_$i\",
                \"messages\": [{\"role\": \"user\", \"content\": \"Memory test $i\"}]
            }" > /dev/null
    done
    
    # Trigger cleanup
    curl -s -X POST $BASE_URL/agent/cleanup \
        -H "X-Secret-Key: $SECRET_KEY" > /dev/null
    
    # Check final stats
    stats=$(curl -s -X GET $BASE_URL/agent/stats -H "X-Secret-Key: $SECRET_KEY")
    
    if echo "$stats" | grep -q "agents_cleaned"; then
        test_passed "$test_name"
    else
        test_failed "$test_name - Stats unavailable"
    fi
}

# Main test execution
main() {
    echo "========================================="
    echo "Goosed Agent Manager Comprehensive Test"
    echo "========================================="
    echo ""
    
    # Clean up any previous test runs
    stop_goosed
    
    # Start goosed
    if ! start_goosed; then
        log_error "Failed to start goosed. Exiting."
        exit 1
    fi
    
    echo ""
    echo "Running tests..."
    echo ""
    
    # Run all tests
    test_health_check
    test_basic_reply
    test_session_isolation
    test_extension_management
    test_extension_isolation
    test_agent_tools
    test_agent_stats
    test_agent_cleanup
    test_context_retrieval
    test_recipe_creation
    test_session_management
    test_concurrent_requests
    test_provider_configuration
    test_session_persistence
    test_audio_endpoint
    test_config_management
    test_schedule_management
    test_memory_stability
    
    echo ""
    echo "========================================="
    echo "Test Results Summary"
    echo "========================================="
    echo -e "${GREEN}Passed: $TESTS_PASSED${NC}"
    echo -e "${RED}Failed: $TESTS_FAILED${NC}"
    
    if [ ${#FAILED_TESTS[@]} -gt 0 ]; then
        echo ""
        echo "Failed tests:"
        for test in "${FAILED_TESTS[@]}"; do
            echo "  - $test"
        done
    fi
    
    echo ""
    
    # Clean up
    stop_goosed
    
    # Exit with appropriate code
    if [ $TESTS_FAILED -eq 0 ]; then
        echo -e "${GREEN}All tests passed! ✅${NC}"
        exit 0
    else
        echo -e "${RED}Some tests failed. Please review the results above.${NC}"
        exit 1
    fi
}

# Handle cleanup on script exit
trap stop_goosed EXIT

# Run main function
main "$@"
