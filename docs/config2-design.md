# Goose Configuration Redesign - config2

This is the new configuration system for Goose, located in `crates/goose/src/config2/`. It addresses all the issues with the current configuration system while providing a cleaner, more maintainable approach.

## Key Improvements

### 1. Everything in One Schema (`schema.rs`)

The entire configuration structure is defined in a single, strongly-typed schema with:
- **Explicit secret marking** using the `#[secret]` attribute and `Secret<T>` type
- **Environment variable mappings** using `#[env_var("...")]` attributes
- **Default values** using `#[serde(default = "...")]`
- **Clear hierarchy** with logical grouping of settings

### 2. Explicit Secret Handling

No more guessing based on field names! Secrets are explicitly marked:

```rust
pub struct OpenAIConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[env_var("OPENAI_API_KEY")]
    #[secret]  // Explicitly marked as secret
    pub api_key: Option<Secret<String>>,
    
    #[serde(default = "default_openai_host")]
    #[env_var("OPENAI_HOST")]
    pub host: String,  // Not a secret
}
```

The `Secret<T>` type:
- Never serializes actual values (always shows `***`)
- Can only be populated from keyring or environment variables
- Has an `expose()` method to get the actual value when needed

### 3. Integrated Environment Variable Mappings

Environment variables are defined right in the schema:

```rust
#[env_var("GOOSE_PROVIDER")]
pub provider: String,

#[env_var("GOOSE_MODEL")]
pub model: String,
```

This makes it:
- Self-documenting
- Impossible to have mismatched mappings
- Easy to see what environment variables are available

### 4. Clear Configuration Hierarchy

```yaml
core:
  provider: openai
  model: gpt-4o
  toolshim:
    enabled: false
  lead_worker:
    provider: null
  router:
    tool_selection_strategy: auto

providers:
  openai:
    api_key: ***  # Secret - never serialized
    host: https://api.openai.com
  anthropic:
    api_key: ***
    host: https://api.anthropic.com

ui:
  cli:
    theme: dark
    show_cost: true
  desktop:
    default_profile: work

developer:
  cache_dir: ~/.cache/goose
  embedding:
    model: text-embedding-3-small
    provider: openai

scheduler:
  type: temporal
  max_concurrent_jobs: 5
```

### 5. Layered Configuration

The `ConfigManager` implements proper precedence:
1. **Runtime overrides** - Programmatic changes
2. **Environment variables** - From schema annotations
3. **Profile config** - Active profile settings
4. **User config** - ~/.config/goose/config.yaml
5. **System config** - /etc/goose/config.yaml (future)
6. **Defaults** - Built into the schema

### 6. Secrets Management

The `SecretsManager` provides automatic fallback:
1. **Environment variables** - Always checked first
2. **Keyring** - Secure system storage (if enabled)
3. **Secrets file** - ~/.config/goose/secrets.yaml (if keyring disabled)

## Usage Examples

### Getting Configuration
```rust
let config = ConfigManager::new().await?;

// Get typed values
let provider: String = config.get("core.provider").await?;
let max_turns: u32 = config.get("core.max_turns").await?;

// Get provider config with resolved secrets
let openai = config.get_provider_config("openai").await?;
```

### Setting Runtime Overrides
```rust
config.set_runtime("core.provider", json!("anthropic")).await?;
```

### Loading Profiles
```rust
config.load_profile("work").await?;
```

## Migration Strategy

1. **Phase 1**: New system coexists with old
   - New code uses `config2`
   - Old code continues using existing config

2. **Phase 2**: Gradual migration
   - Update providers to use new config
   - Remove direct `env::var()` calls

3. **Phase 3**: Complete transition
   - Remove old config system
   - Move `config2` to `config`

## Benefits

1. **Self-documenting** - Schema shows all options
2. **Type-safe** - Compile-time checking
3. **Explicit secrets** - No guessing what's sensitive
4. **Desktop-friendly** - No reliance on env vars
5. **Maintainable** - Everything in one place
6. **Discoverable** - Users can see all options

## Next Steps

1. Add proc macro to extract `#[env_var]` annotations automatically
2. Implement profile management commands
3. Add configuration validation
4. Create migration tool for existing configs
5. Build configuration UI for desktop app
