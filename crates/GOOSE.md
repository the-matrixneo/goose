# RMCP Migration

Context:

* This is a rust project with multiple crates. It's an AI agent called goose that uses MCP.
* We're going to migrate between some internally defined crates implementing parts of the MCP spec and the official rust SDK for MCP available at https://github.com/modelcontextprotocol/rust-sdk and rmcp on crates.io
* The official rust SDK is documented at https://raw.githubusercontent.com/modelcontextprotocol/rust-sdk/refs/heads/main/README.md
* The internally defined crates are the ones in this directory prefixed with mcp: mcp-client, mcp-core, mcp-macros, mcp-server

Goal:

* Work with me to create a migration plan with concrete steps we can progress through to perform the migration
* First make the plan and put it in a file I can review to understand the process
* Use this file both to outline the plan and also to capture progress through the plan, so we can do repeated sessions with a coding agent and it will be able to pick up each phase
* Then we can go step by step through the plan and put a PR up to complete each step. Make sure the steps are well suited to this
* We have about a week to do this work - make it achievable on that timescale
