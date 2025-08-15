#!/bin/bash

# Test script to verify session state reset functionality

echo "Testing session state reset functionality..."

# Create a simple test that writes to TODO and then starts a new session
cat << 'EOF' | cargo run --bin goose session --no-session 2>/dev/null | grep -A 5 "TODO"
Write a todo list with: "Task 1: Test session reset"
Read the todo list
EOF

echo ""
echo "First session completed. Starting second session to verify reset..."
echo ""

# Start a second session and check if TODO is empty
cat << 'EOF' | cargo run --bin goose session --no-session 2>/dev/null | grep -A 5 "TODO"
Read the todo list
EOF

echo ""
echo "Test complete. If the second session shows an empty TODO list, the reset is working correctly."
