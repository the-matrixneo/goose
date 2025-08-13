#!/usr/bin/env bash
set -euo pipefail

# Convert all references of mcp_core::tool::ToolCall to rmcp::model::CallToolRequest
# and construct CallToolRequest at call sites using helper functions in goose::call_tool.
# Run from repo root.

# Collect files to process using ripgrep
FILES=$(rg -l "ToolCall|mcp_core::tool::ToolCall|mcp_core::ToolCall" crates --glob '!crates/mcp-core/**' || true)

if [ -z "$FILES" ]; then
  echo "No files to process"
  exit 0
fi

# 1) Replace imports and qualified paths
for f in $FILES; do
  # Replace specific imports and paths
  perl -0777 -i -pe '
    s#use\s+mcp_core::tool::ToolCall;#use rmcp::model::CallToolRequest; use rmcp::model::CallToolRequestParam;#g;
    s#use\s+mcp_core::ToolCall;#use rmcp::model::CallToolRequest; use rmcp::model::CallToolRequestParam;#g;
    s#mcp_core::tool::ToolCall#rmcp::model::CallToolRequest#g;
    s#mcp_core::ToolCall#rmcp::model::CallToolRequest#g;
  ' "$f"
done

# 2) Replace constructor calls ToolCall::new(NAME, ARGS) with helper make_call_tool_request(NAME, ARGS)
perl -0777 -i -pe '
  s/ToolCall::new\(\s*([^,]+?)\s*,\s*([^\)]+?)\s*\)/CallToolRequest { params: rmcp::model::CallToolRequestParam { name: (\1).to_string().into(), arguments: match (\2) { serde_json::Value::Object(map) => Some(map), _ => None } }, method: Default::default(), extensions: Default::default() }/gs;
' $FILES

# 3) Replace struct literals ToolCall { name: X, arguments: Y } -> make_call_tool_request(X, Y)
perl -0777 -i -pe '
  s/ToolCall\s*\{\s*name\s*:\s*([^,}]+?)\s*,\s*arguments\s*:\s*([^}]+?)\s*\}/CallToolRequest { params: rmcp::model::CallToolRequestParam { name: (\1).to_string().into(), arguments: match (\2) { serde_json::Value::Object(map) => Some(map), _ => None } }, method: Default::default(), extensions: Default::default() }/gs;
' $FILES

# 4) Replace type names ToolCall -> CallToolRequest
perl -0777 -i -pe '
  s/\bToolCall\b/CallToolRequest/g;
' $FILES

# 5) Replace field access .name and .arguments on variables (simple identifiers only)
perl -0777 -i -pe '
  s/([A-Za-z_][A-Za-z0-9_]*)\.name\b/goose::call_tool::name(&\1)/g;
  s/([A-Za-z_][A-Za-z0-9_]*)\.arguments\b/goose::call_tool::args_value(&\1)/g;
' $FILES

# 6) Ensure helper module is exported
if ! rg -n "pub mod call_tool" crates/goose/src/lib.rs >/dev/null; then
  printf "\n// Expose CallTool helpers\npub mod call_tool;\n" >> crates/goose/src/lib.rs
fi

# 7) Done
echo "Conversion script completed."
