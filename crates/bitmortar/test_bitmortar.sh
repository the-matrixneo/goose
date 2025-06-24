#!/bin/bash
set -e

echo "Building bitmortar..."
cd /Users/micn/Development/goose/crates/bitmortar
cargo build --release

echo "Creating test configuration..."
cat > test_config.toml << 'EOF'
[server]
host = "127.0.0.1"
port = 8080
timeout = 60

[routing]
default_provider = "openai"
load_balancing = "FirstAvailable"

[providers.openai]
provider_type = "openai"
enabled = true
priority = 100

[providers.openai.config]
OPENAI_API_KEY = "test-key"
OPENAI_HOST = "https://api.openai.com"
EOF

echo "Starting bitmortar server in background..."
cargo run --release -- --config test_config.toml --port 8080 &
SERVER_PID=$!

# Wait for server to start
sleep 3

echo "Testing health endpoint..."
curl -s http://127.0.0.1:8080/health | jq .

echo "Testing endpoints list..."
curl -s http://127.0.0.1:8080/serving-endpoints | jq .

echo "Stopping server..."
kill $SERVER_PID

echo "Test completed successfully!"
