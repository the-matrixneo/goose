# Goose Configuration Redesign - Implementation

This document provides the implementation of the new Goose configuration system that addresses the issues with environment variable management and provides a cleaner, more structured approach.

## Overview

The new configuration system provides:
- **Type-safe configuration** with a well-defined schema
- **Explicit environment variable mappings** - no more guessing
- **Layered configuration** with clear precedence
- **Secure secrets management** with keyring priority
- **Profile support** for easy configuration switching
- **Backwards compatibility** during migration

## Key Components

### 1. Configuration Schema (`schema.rs`)

Defines the complete configuration structure with all possible settings:

```yaml
core:
  provider: openai
  model: gpt-4o
  mode: auto
  context_limit: null
  temperature: null
  max_turns: 1000
  toolshim:
    enabled: false
    ollama_model: null
  lead_worker:
    provider: null
    model: null
    turns: null
  router:
    tool_selection_strategy: auto
  recipe:
    github_repo: null
    retry_timeout_seconds: null

providers:
  openai:
    api_key: ***  # Never serialized
    host: https://api.openai.com
    base_path: v1/chat/completions
    organization: null
    project: null
    timeout: 600
  anthropic:
    api_key: ***
    host: https://api.anthropic.com
  # ... other providers

ui:
  cli:
    theme: dark
    show_cost: true
    show_thinking: false
    min_priority: 0.5
  desktop:
    default_profile: null
    auto_save_sessions: true

developer:
  cache_dir: ~/.cache/goose
  log_level: info
  embedding:
    model: text-embedding-3-small
    provider: openai
  server:
    secret_key: ***
    allowlist: null

scheduler:
  type: temporal
  max_concurrent_jobs: 5

tracing:
  langfuse:
    url: https://cloud.langfuse.com
    secret_key: ***
```

### 2. Environment Variable Mappings (`env_mapping.rs`)

Provides explicit mappings for all environment variables:

```rust
pub const ENV_MAPPINGS: &[EnvMapping] = &[
    // Core settings
    EnvMapping::new("GOOSE_PROVIDER", "core.provider", false),
    EnvMapping::new("GOOSE_MODEL", "core.model", false),
    
    // Provider secrets
    EnvMapping::new("OPENAI_API_KEY", "providers.openai.api_key", true),
    EnvMapping::new("ANTHROPIC_API_KEY", "providers.anthropic.api_key", true),
    
    // Provider configs
    EnvMapping::new("OPENAI_HOST", "providers.openai.host", false)
        .with_default("https://api.openai.com"),
    
    // ... all other mappings
];
```

### 3. Configuration Manager (`manager.rs`)

Implements layered configuration with proper precedence:

1. **Runtime overrides** - Programmatic changes
2. **Environment variables** - From ENV_MAPPINGS
3. **Profile config** - Active profile settings
4. **User config** - ~/.config/goose/config.yaml
5. **System config** - /etc/goose/config.yaml (future)
6. **Defaults** - Built-in defaults

### 4. Secrets Management (`secrets.rs`)

Handles secrets with automatic fallback:

1. **Environment variables** - Always checked first
2. **Keyring** - Secure system storage (if enabled)
3. **Secrets file** - ~/.config/goose/secrets.yaml (if keyring disabled)

## Migration Strategy

### Phase 1: Parallel Systems
- New ConfigManager coexists with old Config
- New code uses ConfigManager
- Old code continues using Config

### Phase 2: Provider Migration
```rust
// Old way:
let api_key: String = config.get_secret("OPENAI_API_KEY")?;
let host: String = config.get_param("OPENAI_HOST")
    .unwrap_or_else(|_| "https://api.openai.com".to_string());

// New way:
let openai_config = config_manager.get_provider_config("openai").await?;
let api_key = openai_config.api_key.expose();
let host = &openai_config.host;
```

### Phase 3: Complete Migration
- Remove all direct env::var() calls
- Remove old Config system
- Update all providers and components

## Benefits

1. **Self-documenting** - All configuration options are in the schema
2. **Type-safe** - No more stringly-typed configuration
3. **Discoverable** - Users can see all available options
4. **Secure** - Secrets never serialized to config files
5. **Flexible** - Easy to add new configuration options
6. **Desktop-friendly** - No reliance on environment variables

## Usage Examples

### Getting Configuration Values
```rust
let config = ConfigManager::new().await?;

// Get typed values
let provider: String = config.get("core.provider").await?;
let max_turns: u32 = config.get("core.max_turns").await?;

// Get secrets
let api_key = config.get_secret("providers.openai.api_key").await?;

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

## Environment Variable Reference

All supported environment variables are explicitly defined in `env_mapping.rs`. Some examples:

- `GOOSE_PROVIDER` → `core.provider`
- `GOOSE_MODEL` → `core.model`
- `OPENAI_API_KEY` → `providers.openai.api_key` (secret)
- `GOOSE_CLI_THEME` → `ui.cli.theme`

## Next Steps

1. Add profile management commands to CLI
2. Implement configuration UI for desktop
3. Migrate providers to use new system
4. Add configuration validation
5. Implement hot-reloading for development
