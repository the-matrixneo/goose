# Deep Dive Report: Recipe System in Goose

## Executive Summary

This report provides a comprehensive analysis of the recipe and sub-recipe system in Goose, with the goal of designing a new `recipe__create` platform tool that enables agents to programmatically create and validate recipe files. The tool should integrate seamlessly with existing infrastructure while maintaining minimal code changes.

## Recipe System Architecture

### Core Recipe Structure

The recipe system is defined in `crates/goose/src/recipe/mod.rs` with the main `Recipe` struct containing:

**Required Fields:**
- `version`: Semantic version (defaults to "1.0.0")
- `title`: Short descriptive name
- `description`: Detailed purpose explanation
- At least one of `instructions` or `prompt` must be set

**Optional Fields:**
- `prompt`: Initial session prompt
- `extensions`: List of required extensions
- `context`: Supplementary context
- `activities`: UI activity labels
- `author`: Creator information
- `settings`: Model/provider settings
- `parameters`: Dynamic recipe parameters
- `response`: JSON schema validation
- `sub_recipes`: Nested recipe configurations
- `retry`: Retry configuration

### Recipe Validation System

The validation system is sophisticated and multi-layered:

1. **Format Validation**: Supports both JSON and YAML formats via `Recipe::from_content()`
2. **Parameter Validation**: 
   - Ensures optional parameters have default values
   - Validates template variables match parameter definitions
   - Checks for missing/extra parameter definitions
3. **Security Validation**: 
   - `check_for_security_warnings()` detects harmful Unicode tags
   - Path traversal protection for sub-recipes
4. **Retry Configuration Validation**: Validates retry settings if present

### Template System

Recipes support Jinja2-style templating via `template_recipe.rs`:

- **Variable Extraction**: Identifies template variables in recipe content
- **Complex Variable Handling**: Handles invalid variable names by wrapping in `{% raw %}` blocks
- **Recursive Expansion**: Supports file references (@-mentions) with depth limits
- **Built-in Variables**: `recipe_dir` is automatically available
- **Rendering Pipeline**:
  1. Pre-process template variables
  2. Handle empty quotes replacement
  3. Apply parameter values
  4. Validate rendered content

### Sub-Recipe System

Sub-recipes enable modular recipe composition:

**Structure** (`SubRecipe` type):
```rust
pub struct SubRecipe {
    pub name: String,
    pub path: String,
    pub values: Option<HashMap<String, String>>,
    pub sequential_when_repeated: bool,
    pub description: Option<String>,
}
```

**Key Features:**
- Dynamic tool generation for each sub-recipe
- Parameter inheritance and override
- Execution mode control (sequential/parallel)
- Path resolution (relative to parent recipe)

**Integration Points:**
- `SubRecipeManager`: Manages sub-recipe tools and dispatch
- `create_sub_recipe_task_tool()`: Generates dynamic tools
- Task execution via `TasksManager`

### Recipe Building Pipeline

The build process (`build_recipe/mod.rs`) follows these steps:

1. **Template Rendering**: Apply parameters to template
2. **Validation**: Check required parameters
3. **Path Resolution**: Resolve sub-recipe paths
4. **Content Parsing**: Parse rendered YAML/JSON to Recipe struct
5. **Final Validation**: Validate complete recipe

### File Management

Recipes interact with the file system through several mechanisms:

1. **Recipe Files**: Read via `read_recipe_file_content.rs`
2. **Temporary Files**: Used in various places (e.g., shell output truncation in developer extension)
3. **Session Storage**: Recipe execution results stored in JSONL format
4. **Path Security**: Comprehensive path validation to prevent traversal attacks

## Existing Tool Patterns

### Platform Tools Architecture

Platform tools follow a consistent pattern in `platform_tools.rs`:

```rust
pub fn tool_name() -> Tool {
    Tool::new(
        TOOL_NAME_CONSTANT.to_string(),
        description,
        input_schema
    ).annotate(ToolAnnotations {
        title: Some(...),
        read_only_hint: Some(...),
        destructive_hint: Some(...),
        idempotent_hint: Some(...),
        open_world_hint: Some(...),
    })
}
```

### Temporary File Handling

The codebase uses several approaches for temporary files:

1. **Session Storage** (`session/storage.rs`):
   - Uses atomic file operations with `.tmp` extension
   - Implements file locking for concurrent access
   - Automatic cleanup on error

2. **Developer Extension** (`goose-mcp/src/developer/mod.rs`):
   - Uses `tempfile::NamedTempFile` for shell output
   - Calls `.keep()` to persist when needed
   - Returns path for reference

3. **Best Practices Observed**:
   - Always use secure permissions (0600 on Unix)
   - Atomic operations via rename
   - Proper error handling with cleanup

## Design Recommendations for `recipe__create` Tool

### Tool Signature

```rust
pub const PLATFORM_CREATE_RECIPE_TOOL_NAME: &str = "recipe__create";

pub fn create_recipe_tool() -> Tool {
    Tool::new(
        PLATFORM_CREATE_RECIPE_TOOL_NAME.to_string(),
        indoc! {r#"
            Create a validated recipe file for Goose.
            
            This tool creates a recipe with the provided configuration, validates it,
            and saves it to a temporary file. The recipe can then be executed or
            scheduled using other platform tools.
            
            The tool returns the path to the created recipe file and any validation
            warnings or errors encountered.
        "#}.to_string(),
        object!({
            "type": "object",
            "required": ["recipe"],
            "properties": {
                "recipe": {
                    "type": "object",
                    "required": ["title", "description"],
                    "properties": {
                        "title": {"type": "string", "description": "Short descriptive name"},
                        "description": {"type": "string", "description": "Detailed purpose"},
                        "instructions": {"type": "string", "description": "Agent instructions"},
                        "prompt": {"type": "string", "description": "Initial prompt"},
                        "extensions": {
                            "type": "array",
                            "items": {"type": "object"},
                            "description": "Required extensions"
                        },
                        "parameters": {
                            "type": "array",
                            "items": {"type": "object"},
                            "description": "Recipe parameters"
                        },
                        "sub_recipes": {
                            "type": "array",
                            "items": {"type": "object"},
                            "description": "Sub-recipes"
                        },
                        // ... other fields
                    }
                },
                "format": {
                    "type": "string",
                    "enum": ["yaml", "json"],
                    "default": "yaml",
                    "description": "Output format"
                }
            }
        })
    ).annotate(ToolAnnotations {
        title: Some("Create a recipe file".to_string()),
        read_only_hint: Some(false),
        destructive_hint: Some(false),
        idempotent_hint: Some(false),
        open_world_hint: Some(false),
    })
}
```

### Implementation Strategy

#### 1. Minimal Code Changes Approach

**Location**: Add to `crates/goose/src/agents/platform_tools.rs`

**Dependencies**: Reuse existing recipe validation:
- Import `crate::recipe::Recipe`
- Use `Recipe::from_content()` for validation
- Leverage existing builder pattern

#### 2. Temporary File Management

Follow the session storage pattern:

```rust
use tempfile::Builder;

fn create_recipe_file(recipe: &Recipe, format: &str) -> Result<PathBuf> {
    // Create temp file with .yaml or .json extension
    let temp_file = Builder::new()
        .prefix("goose_recipe_")
        .suffix(&format!(".{}", format))
        .tempfile()?;
    
    // Serialize recipe
    let content = match format {
        "yaml" => serde_yaml::to_string(&recipe)?,
        "json" => serde_json::to_string_pretty(&recipe)?,
        _ => return Err(anyhow!("Invalid format"))
    };
    
    // Write with secure permissions
    temp_file.as_file().write_all(content.as_bytes())?;
    
    // Persist the file
    let (_, path) = temp_file.keep()?;
    Ok(path)
}
```

#### 3. Validation Pipeline

Implement comprehensive validation:

```rust
fn validate_recipe(recipe: &Recipe) -> Result<Vec<String>> {
    let mut warnings = Vec::new();
    
    // Security check
    if recipe.check_for_security_warnings() {
        warnings.push("Recipe contains potentially harmful content".to_string());
    }
    
    // Validate retry config if present
    if let Some(ref retry) = recipe.retry {
        if let Err(e) = retry.validate() {
            return Err(anyhow!("Invalid retry config: {}", e));
        }
    }
    
    // Check sub-recipe paths are accessible
    if let Some(ref sub_recipes) = recipe.sub_recipes {
        for sub in sub_recipes {
            // Validate path exists or is resolvable
            // Add warnings for missing sub-recipes
        }
    }
    
    Ok(warnings)
}
```

#### 4. Integration Points

The tool should integrate with:

1. **Recipe Execution**: Created recipes can be executed via existing recipe tools
2. **Scheduling**: Use `platform__manage_schedule` with the created recipe path
3. **Sub-Recipe System**: Validate sub-recipe references
4. **Extension System**: Validate required extensions exist

### Testing Strategy

1. **Unit Tests**:
   - Recipe creation with all field combinations
   - Validation of invalid recipes
   - Format conversion (YAML/JSON)
   - Temporary file cleanup

2. **Integration Tests**:
   - Create and execute recipe
   - Create and schedule recipe
   - Recipe with sub-recipes
   - Security validation

3. **Edge Cases**:
   - Empty required fields
   - Invalid extension references
   - Circular sub-recipe references
   - Large recipe files

### Error Handling

Follow existing patterns:

1. Return descriptive error messages
2. Clean up temporary files on error
3. Validate before file creation
4. Use `Result<>` types consistently

### Security Considerations

1. **Path Validation**: Prevent path traversal in sub-recipe paths
2. **Content Validation**: Use existing Unicode tag detection
3. **File Permissions**: Set 0600 on Unix systems
4. **Size Limits**: Implement reasonable recipe size limits
5. **Template Injection**: Validate template variables

## Implementation Checklist

- [ ] Add tool definition to `platform_tools.rs`
- [ ] Implement recipe creation logic
- [ ] Add validation pipeline
- [ ] Implement temporary file management
- [ ] Add comprehensive tests
- [ ] Update tool router to include new tool
- [ ] Document tool usage
- [ ] Add example recipes for testing

## Benefits of This Approach

1. **Minimal Changes**: Reuses existing validation and serialization code
2. **Consistent**: Follows established patterns for platform tools
3. **Secure**: Leverages existing security validations
4. **Testable**: Clear separation of concerns
5. **Maintainable**: Simple, focused implementation
6. **Elegant**: Clean API that fits naturally into the system

## Conclusion

The `recipe__create` tool can be implemented with minimal changes to the existing codebase by:

1. Adding a new platform tool following established patterns
2. Reusing the existing Recipe validation infrastructure
3. Following the temporary file patterns from session storage
4. Integrating with existing recipe execution and scheduling tools

This approach ensures the tool is independent, elegant, minimal, readable, maintainable, and fully tested while requiring no changes to the core recipe system.
