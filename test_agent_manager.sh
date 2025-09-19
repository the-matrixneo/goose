#!/bin/bash

# Test script for Agent Manager functionality

echo "Starting goosed server..."
RUST_LOG=info,goose=trace GOOSE_PROVIDER=openai GOOSE_MODEL=gpt-4o-mini cargo run --release --bin goosed &
SERVER_PID=$!

# Wait for server to start
sleep 3

echo "Testing multi-session isolation..."

# Create session 1
echo "Creating session 1..."
curl -X POST http://localhost:3000/reply \
  -H "Content-Type: application/json" \
  -H "x-secret-key: test" \
  -d '{
    "messages": [{"role": "user", "content": [{"type": "text", "text": "Hello session 1"}]}],
    "session_id": "test_session_1"
  }' &

# Create session 2
echo "Creating session 2..."
curl -X POST http://localhost:3000/reply \
  -H "Content-Type: application/json" \
  -H "x-secret-key: test" \
  -d '{
    "messages": [{"role": "user", "content": [{"type": "text", "text": "Hello session 2"}]}],
    "session_id": "test_session_2"
  }' &

# Wait for requests
sleep 2

# Check agent metrics
echo "Checking agent metrics..."
curl -X GET http://localhost:3000/agent/metrics \
  -H "x-secret-key: test"

echo ""
echo "Killing server..."
kill $SERVER_PID

echo "Test completed"
