#!/bin/bash
# Simple goosed test for Agent Manager

set -e

# Configuration
PORT=8081
SECRET="test123"
BASE="http://localhost:$PORT"

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m'

echo "Building goosed..."
cargo build -p goose-server --bin goosed

echo "Starting goosed..."
GOOSE_PORT=$PORT GOOSE_SERVER__SECRET_KEY=$SECRET ./target/debug/goosed agent &
GOOSED_PID=$!

# Wait for server
echo "Waiting for server to start..."
sleep 5

echo ""
echo "Running tests..."
echo ""

# Test 1: Health check
echo -n "1. Health check: "
if curl -s "$BASE/status" | grep -q "ok"; then
    echo -e "${GREEN}✓${NC}"
else
    echo -e "${RED}✗${NC}"
fi

# Test 2: Basic reply (auto session)
echo -n "2. Basic reply: "
REPLY=$(curl -s -X POST "$BASE/reply" \
    -H "X-Secret-Key: $SECRET" \
    -H "Content-Type: application/json" \
    -d '{"messages": [{"role": "user", "content": "test"}]}')
if echo "$REPLY" | grep -q "content"; then
    echo -e "${GREEN}✓${NC}"
else
    echo -e "${RED}✗${NC}"
fi

# Test 3: Session isolation
echo -n "3. Session isolation: "
curl -s -X POST "$BASE/reply" \
    -H "X-Secret-Key: $SECRET" \
    -H "Content-Type: application/json" \
    -d '{"session_id": "test1", "messages": [{"role": "user", "content": "I am session 1"}]}' > /dev/null

curl -s -X POST "$BASE/reply" \
    -H "X-Secret-Key: $SECRET" \
    -H "Content-Type: application/json" \
    -d '{"session_id": "test2", "messages": [{"role": "user", "content": "I am session 2"}]}' > /dev/null

CTX1=$(curl -s "$BASE/context?session_id=test1" -H "X-Secret-Key: $SECRET")
CTX2=$(curl -s "$BASE/context?session_id=test2" -H "X-Secret-Key: $SECRET")

if [ "$CTX1" != "$CTX2" ]; then
    echo -e "${GREEN}✓${NC}"
else
    echo -e "${RED}✗${NC}"
fi

# Test 4: Agent stats
echo -n "4. Agent stats: "
STATS=$(curl -s "$BASE/agent/stats" -H "X-Secret-Key: $SECRET")
if echo "$STATS" | grep -q "agents_created"; then
    echo -e "${GREEN}✓${NC}"
else
    echo -e "${RED}✗${NC}"
fi

# Test 5: Agent cleanup
echo -n "5. Agent cleanup: "
CLEANUP=$(curl -s -X POST "$BASE/agent/cleanup" -H "X-Secret-Key: $SECRET")
if echo "$CLEANUP" | grep -q "cleaned"; then
    echo -e "${GREEN}✓${NC}"
else
    echo -e "${RED}✗${NC}"
fi

# Test 6: Extension management
echo -n "6. Extension management: "
curl -s -X POST "$BASE/extensions/add" \
    -H "X-Secret-Key: $SECRET" \
    -H "Content-Type: application/json" \
    -d '{
        "session_id": "ext_test",
        "type": "frontend",
        "name": "test_ext",
        "tools": [{"name": "test_tool", "description": "Test", "input_schema": {"type": "object"}}]
    }' > /dev/null

EXT_LIST=$(curl -s "$BASE/extensions/list?session_id=ext_test" -H "X-Secret-Key: $SECRET")
if echo "$EXT_LIST" | grep -q "test_ext"; then
    echo -e "${GREEN}✓${NC}"
else
    echo -e "${RED}✗${NC}"
fi

# Test 7: Extension isolation
echo -n "7. Extension isolation: "
EXT_LIST2=$(curl -s "$BASE/extensions/list?session_id=other_session" -H "X-Secret-Key: $SECRET")
if ! echo "$EXT_LIST2" | grep -q "test_ext"; then
    echo -e "${GREEN}✓${NC}"
else
    echo -e "${RED}✗${NC}"
fi

# Test 8: Session persistence
echo -n "8. Session persistence: "
curl -s -X POST "$BASE/reply" \
    -H "X-Secret-Key: $SECRET" \
    -H "Content-Type: application/json" \
    -d '{"session_id": "persist", "messages": [{"role": "user", "content": "MARKER123"}]}' > /dev/null

curl -s -X POST "$BASE/agent/cleanup" -H "X-Secret-Key: $SECRET" > /dev/null

CTX_PERSIST=$(curl -s "$BASE/context?session_id=persist" -H "X-Secret-Key: $SECRET")
if echo "$CTX_PERSIST" | grep -q "MARKER123"; then
    echo -e "${GREEN}✓${NC}"
else
    echo -e "${RED}✗${NC}"
fi

echo ""
echo "Tests complete!"

# Cleanup
kill $GOOSED_PID 2>/dev/null || true
