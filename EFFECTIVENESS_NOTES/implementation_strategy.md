# Implementation Strategy for Multi-File Edit + Code Search Steering

## Current Goose Architecture Analysis

### Strengths
1. **MCP Infrastructure**: Already has robust MCP implementation
2. **Developer Extension**: Text editor with str_replace, insert, write, undo
3. **Subagent System**: Can parallelize tasks
4. **Shell Integration**: Uses ripgrep for file search

### Limitations
1. **Edit Formats**: Only has str_replace (exact match), not unified diff
2. **Code Understanding**: No AST/semantic search capabilities
3. **Repository Map**: No dependency tracking or file relationship mapping
4. **Context Management**: No intelligent code context retrieval

## Implementation Options

### Option 1: Enhance Developer MCP (Recommended)
**Pros:**
- Minimal changes to existing architecture
- Leverages existing MCP infrastructure
- Can be deployed immediately
- Works with all Goose interfaces (CLI, desktop, server)

**Cons:**
- Limited by MCP protocol constraints
- May need protocol extensions for advanced features

### Option 2: Platform Tools
**Pros:**
- Direct integration with core Goose
- Can access internal state directly
- More flexibility in implementation

**Cons:**
- Requires changes to core crate
- More complex deployment
- Harder to test in isolation

### Option 3: Bundled CLI Tools
**Pros:**
- Can leverage existing tools (tree-sitter, etc.)
- Language agnostic
- Easy to update independently

**Cons:**
- Requires external dependencies
- Platform compatibility issues
- Performance overhead of process spawning

### Option 4: Internal MCP Servers
**Pros:**
- Can create specialized servers for each capability
- Clean separation of concerns
- Can be written in different languages if needed

**Cons:**
- More complex architecture
- Additional processes to manage
- Communication overhead

## Recommended Implementation Plan

### Phase 1: Enhanced Edit Formats (Minimal MVP)
Add to developer MCP:

1. **Unified Diff Support**
   - New command: `udiff` for text_editor tool
   - Apply unified diff patches
   - 90% token reduction for large edits

2. **Multi-File Transaction**
   - New tool: `multi_edit` that accepts array of edits
   - Atomic application with rollback on failure
   - Dependency-aware ordering

3. **Repository Map**
   - New tool: `repo_map` to show file dependencies
   - Cache file relationships
   - Track imports/exports

### Phase 2: Code Search Enhancement
1. **Semantic Search Tool**
   - New tool: `code_search` with semantic understanding
   - Use tree-sitter for AST parsing (via shell command initially)
   - Return relevant code snippets with context

2. **Context Retrieval**
   - Smart context window management
   - Prioritize relevant code sections
   - Track which files are in context

### Phase 3: Advanced Features
1. **Planning Mode**
   - Separate planning from execution
   - Generate edit plans before applying
   - Preview changes

2. **Git Integration**
   - Auto-commit with meaningful messages
   - Track changes across sessions
   - Diff viewing capabilities

## Minimal Implementation for Quick Wins

### Step 1: Add Unified Diff to text_editor (1-2 days)
```rust
// In text_editor.rs
pub async fn text_editor_udiff(
    path: &PathBuf,
    diff: &str,
    file_history: &Arc<Mutex<HashMap<PathBuf, Vec<String>>>>,
) -> Result<Vec<Content>, ErrorData> {
    // Apply unified diff using patch command or rust library
    // Save history for undo
    // Return success/failure
}
```

### Step 2: Add Repository Map Tool (1 day)
```rust
// New tool in developer MCP
#[tool(
    name = "repo_map",
    description = "Generate a map of file dependencies and relationships"
)]
pub async fn repo_map(
    &self,
    params: Parameters<RepoMapParams>,
) -> Result<CallToolResult, ErrorData> {
    // Use ripgrep to find imports/exports
    // Build dependency graph
    // Return formatted map
}
```

### Step 3: Add Code Search Tool (2 days)
```rust
#[tool(
    name = "code_search",
    description = "Search for code semantically across the repository"
)]
pub async fn code_search(
    &self,
    params: Parameters<CodeSearchParams>,
) -> Result<CallToolResult, ErrorData> {
    // Use ripgrep with context lines
    // Parse results for relevance
    // Return ranked results
}
```

## Expected Impact on Terminal-Bench

### Immediate Improvements (Phase 1)
- **File Operations**: 30-40% faster with unified diff
- **Multi-File Tasks**: 50% improvement with atomic operations
- **Token Usage**: 70-90% reduction on large edits

### Medium-term Improvements (Phase 2)
- **Code Navigation**: 60% faster finding relevant code
- **Context Quality**: 40% better task understanding
- **Success Rate**: 20-30% improvement overall

### Long-term Improvements (Phase 3)
- **Complex Tasks**: 50% better handling
- **Error Recovery**: 70% fewer failures
- **Overall Score**: 40-50% improvement expected

## Risk Mitigation

1. **Backward Compatibility**: Keep existing commands, add new ones
2. **Testing**: Comprehensive test suite for each new feature
3. **Gradual Rollout**: Feature flags for new capabilities
4. **Fallback Mechanisms**: Revert to existing methods if new ones fail

## Timeline

- **Week 1**: Implement unified diff support
- **Week 2**: Add repository map and multi-edit
- **Week 3**: Basic code search
- **Week 4**: Testing and optimization
- **Month 2**: Advanced features and refinement

This approach provides maximum impact with minimal disruption to existing Goose architecture.
