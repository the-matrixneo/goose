# Prompt for Implementing the Analyze Tool

## Context & Mission

You are implementing an `analyze` tool for Goose's Developer MCP that will provide semantic code analysis capabilities. This tool will help the LLM understand code structure and relationships, leading to more accurate modifications and fewer errors when working with codebases.

## Key Documents to Read First

1. **ANALYZE_TOOL_REPORT.md** - The complete implementation plan with code examples
2. **crates/goose-mcp/src/developer/rmcp_developer.rs** - Study the existing patterns, especially:
   - Lines 85-100: How parameter structs are defined
   - Lines 600-650: How tools are implemented with the `#[tool()]` macro
   - Lines 1650-1750: The `shell` tool implementation pattern
   - Lines 2500-3500: Test patterns to follow
3. **crates/goose-mcp/src/developer/lang.rs** - Existing language detection (reuse this!)

## Implementation Approach

### Phase 1: Set Up Development Cycle (First 30 minutes)
```bash
# Set up your iteration cycle:
cd /Users/tlongwell/Development/goose

# Create a branch for this work
git checkout -b feat/analyze-tool

# Set up watch commands in separate terminals:
# Terminal 1: Auto-format on save
cargo watch -x fmt

# Terminal 2: Check compilation
cargo watch -c -x "build -p goose-mcp"

# Terminal 3: Run linter
cargo watch -c -x "run --bin cargo -- clippy -p goose-mcp -- -W clippy::all"

# Terminal 4: Run tests
cargo watch -c -x "test -p goose-mcp analyze"
```

### Phase 2: Add Dependencies (10 minutes)
Edit `crates/goose-mcp/Cargo.toml`:
```toml
# Add these dependencies
tree-sitter = "0.25"
tree-sitter-loader = "0.25"
tree-sitter-rust = "0.24"
tree-sitter-python = "0.23"
tree-sitter-javascript = "0.25"
tree-sitter-go = "0.25"
tree-sitter-java = "0.23"
lru = "0.12"  # For caching
```

### Phase 3: Implementation Order (Follow this exactly!)

1. **Add Parameter Struct** (15 minutes)
   - Add `AnalyzeParams` struct after line 85 in `rmcp_developer.rs`
   - Follow the exact pattern of `TextEditorParams`
   - Run `cargo build -p goose-mcp` to verify it compiles

2. **Add to DeveloperServer Struct** (15 minutes)
   - Add parser cache and analysis cache fields (around line 155)
   - Update the `new()` method to initialize them
   - Ensure `cargo fmt` is happy

3. **Implement Core Analysis Functions** (45 minutes)
   - Add `get_or_create_parser()` 
   - Add `analyze_file()`
   - Add `extract_functions()` using tree-sitter queries
   - Test each function in isolation first

4. **Add the Tool Implementation** (30 minutes)
   - Add the `#[tool()]` decorated `analyze` function
   - Follow the EXACT pattern of the `shell` tool
   - Use the same error handling patterns
   - Use the same `CallToolResult::success()` pattern

5. **Write Tests** (30 minutes)
   - Add tests at the end of the file following existing patterns
   - Use `#[tokio::test]` and `#[serial]` attributes
   - Test single file analysis first
   - Test directory analysis second
   - Test error cases

## Testing Strategy

### Test File 1: Simple Python File
```python
# test_simple.py
def calculate(x, y):
    return x + y

class Calculator:
    def add(self, a, b):
        return calculate(a, b)
```

Expected output should show:
- Functions: calculate (line 2)
- Classes: Calculator (line 5)
- Class methods: add (line 6)

### Test File 2: Simple Rust File
```rust
// test_simple.rs
fn main() {
    println!("Hello");
    helper();
}

fn helper() {
    println!("Helper");
}
```

Expected output should show:
- Functions: main (line 2), helper (line 7)

### Integration Test
```rust
#[tokio::test]
#[serial]
async fn test_analyze_python_file() {
    // Follow the pattern from test_text_editor_write_and_view_file
    let temp_dir = tempfile::tempdir().unwrap();
    std::env::set_current_dir(&temp_dir).unwrap();
    
    // Create test file
    std::fs::write("test.py", "def hello():\n    pass").unwrap();
    
    let server = create_test_server();
    let params = Parameters(AnalyzeParams {
        path: temp_dir.path().join("test.py").to_str().unwrap().to_string(),
        depth: "structure".to_string(),
        focus: None,
        max_depth: 3,
    });
    
    let result = server.analyze(params).await.unwrap();
    
    // Check the output
    let content = &result.content[0];
    assert!(content.as_text().unwrap().text.contains("hello"));
    assert!(content.as_text().unwrap().text.contains("line 1"));
}
```

## Success Criteria

### Milestone 1: Compilation ✅
- [ ] Code compiles without errors
- [ ] `cargo fmt` produces no changes
- [ ] `cargo clippy` shows no warnings

### Milestone 2: Basic Functionality ✅
- [ ] Can analyze a single Python file and extract functions
- [ ] Can analyze a single Rust file and extract functions
- [ ] Returns properly formatted output using `formatdoc!`

### Milestone 3: Full Implementation ✅
- [ ] Supports Python, Rust, JavaScript, Go, Java
- [ ] Can analyze directories recursively
- [ ] Respects `.gooseignore` patterns
- [ ] Has caching to avoid re-parsing unchanged files
- [ ] All tests pass

### Milestone 4: Integration ✅
- [ ] Tool appears in Goose's tool list
- [ ] Can be called from the Goose CLI/desktop
- [ ] Output format is clear and useful to LLMs
- [ ] Performance is acceptable (<1s for typical project)

## Why We're Doing This

### The Problem
Currently, when Goose needs to understand code relationships (e.g., "rename this function"), it has to:
1. Use multiple `rg` commands to search for text
2. Read entire files to understand structure
3. Often misses call sites or dependencies
4. Uses excessive tokens for exploration

### The Solution
The `analyze` tool provides structured understanding in one command:
- Shows all functions, classes, and their relationships
- Can focus on specific symbols to find all usages
- Reduces token usage by 50%+
- Prevents breaking changes by showing all dependencies

### Expected Impact
- **Refactoring tasks**: From 60% success → 95% success
- **Code navigation**: 80% faster
- **Token usage**: 50% reduction
- **Terminal-Bench scores**: 25-35% improvement on code tasks

## Common Pitfalls to Avoid

1. **Don't forget to handle ignore patterns** - Check `self.is_ignored()`
2. **Use existing language detection** - Don't reimplement `get_language_identifier()`
3. **Follow error patterns exactly** - `ErrorData::new(ErrorCode::INTERNAL_ERROR, msg, None)`
4. **Test incrementally** - Don't try to implement everything before testing
5. **Use `cargo watch`** - It catches issues immediately

## Final Checklist Before PR

- [ ] All tests pass: `cargo test -p goose-mcp analyze`
- [ ] Formatting is correct: `cargo fmt --check -p goose-mcp`
- [ ] No clippy warnings: `./scripts/clippy-lint.sh`
- [ ] Documentation comments on public functions
- [ ] Manual test with real Python/Rust project
- [ ] Output is helpful and clear
- [ ] Performance is acceptable

## Quick Start Commands

```bash
# Start here:
cd /Users/tlongwell/Development/goose
git checkout -b feat/analyze-tool

# Open the main file to edit:
code crates/goose-mcp/src/developer/rmcp_developer.rs

# Run tests for your new feature:
cargo test -p goose-mcp analyze

# Test the full build:
cargo build --workspace

# Try it manually:
cargo run --bin goose -- analyze --path src/
```

Remember: The goal is to give Goose's LLM the ability to understand code structure and relationships with a single command, dramatically improving its ability to make correct modifications. Keep the implementation minimal, elegant, and well-tested.
