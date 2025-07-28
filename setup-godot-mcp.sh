#!/bin/bash

# Script to add Godot MCP extension to Goose configuration

# Path to the Godot MCP server
GODOT_MCP_PATH="/Users/dkatz/git/goose2/Godot-MCP/server/dist/index.js"

# Check if the server file exists
if [ ! -f "$GODOT_MCP_PATH" ]; then
    echo "Error: Godot MCP server not found at $GODOT_MCP_PATH"
    exit 1
fi

# Create the extension configuration
cat > godot-mcp-extension.json << EOF
{
  "type": "stdio",
  "name": "godot-mcp",
  "cmd": "node",
  "args": ["$GODOT_MCP_PATH"],
  "description": "Godot MCP server for interacting with Godot Engine projects",
  "timeout": 300,
  "bundled": false
}
EOF

echo "Created Godot MCP extension configuration at godot-mcp-extension.json"
echo ""
echo "To use this extension with Goose, you can:"
echo "1. Use it directly in a session with: goose session --with-extension 'node $GODOT_MCP_PATH'"
echo "2. Or add it to your Goose configuration programmatically"
echo ""
echo "The extension provides tools for:"
echo "- Node management (create, modify, delete nodes)"
echo "- Script management (read, write, analyze scripts)"
echo "- Scene management (create, modify scenes)"
echo "- Project management (access project settings and resources)"
echo "- Editor control (run project, get editor state)"
