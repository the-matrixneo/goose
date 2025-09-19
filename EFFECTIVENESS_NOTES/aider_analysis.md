# Aider Analysis

## Key Features

### 1. Repository Map
- Tracks file dependencies and relationships
- Builds a graph of imports/exports
- Helps identify which files to include in context

### 2. Edit Formats
Multiple edit formats for different models:
- **whole**: Returns entire file (token-intensive)
- **diff/editblock**: Search/replace blocks with clear markers
- **udiff**: Unified diff format (most token-efficient, 90% reduction)
- **diff-fenced**: Similar to diff but with file path inside fence
- **patch**: Standard git patch format

### 3. Code Search
- Uses tree-sitter for AST parsing
- Builds semantic understanding of code structure
- Can navigate imports and dependencies

### 4. Git Integration
- Automatic commit generation with context
- Tracks dirty files
- Handles staging and committing
- Generates meaningful commit messages

## Implementation Details

### Edit Coders
- `editblock_coder.py`: Main search/replace implementation
- `udiff_coder.py`: Unified diff implementation
- `patch_coder.py`: Git patch format
- `wholefile_coder.py`: Whole file replacement

### Repository Management (`repo.py`)
- Git operations wrapper
- File tracking (staged, dirty, tracked)
- Ignore file handling (.aiderignore)
- Commit message generation with LLM

### Search & Context
- Uses pathspec for gitignore-style patterns
- Caches file states for performance
- Subtree-only mode for large repos
