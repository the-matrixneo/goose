# Configuration Centralization Migration Plan

## Problem Statement

Currently, Goose has multiple configuration mechanisms scattered throughout the codebase:
1. Direct `std::env::var()` calls (49 files)
2. Config system calls through `Config::global()`
3. Mixed approaches where some values come from env vars, others from config

This makes it:
- Hard to test (can't control env vars easily)
- Not reproducible (env vars override everything)
- Difficult to understand what configuration is actually in use

## Solution

Centralize ALL configuration access through the `Config` system, while still supporting environment variable overrides but in a controlled way.

## Migration Strategy

### Phase 1: Audit and Categorize
1. ✅ Identify all `std::env::var()` calls (49 files found)
2. ✅ Categorize them by type:
   - Provider configuration (API keys, hosts, etc.)
   - Model configuration (context limits, temperature, etc.)
   - System configuration (paths, debugging flags, etc.)
   - Feature flags (experimental features, etc.)

### Phase 2: Enhance Config System
1. ✅ Add `get_param_with_env_override()` method to allow controlled env var access
2. Add configuration validation
3. Add configuration schema/documentation
4. Add test utilities for config isolation

### Phase 3: Migrate Provider Configurations
1. Update all provider `from_env()` methods to use `Config::global()`
2. Ensure all provider config keys are documented in metadata
3. Test that providers work with both config file and env var overrides

### Phase 4: Migrate Model Configuration
1. Update `ModelConfig` to use centralized config
2. Remove direct env var access from model parsing
3. Ensure backward compatibility

### Phase 5: Migrate System Configuration
1. Update all system-level env var usage
2. Add proper config keys for debugging flags
3. Update CLI and server to use centralized config

### Phase 6: Testing and Validation
1. Add comprehensive tests for config precedence
2. Add integration tests that verify env vars still work
3. Add tests that can run without env vars (using config only)

## Implementation Details

### Config Precedence (Highest to Lowest)
1. Environment variables (when `allow_env_override = true`)
2. Configuration file values
3. Default values (provider-specific)

### New Config Methods
```rust
// Existing - allows env override by default
pub fn get_param<T>(&self, key: &str) -> Result<T, ConfigError>

// New - allows controlling env override (useful for tests)
pub fn get_param_with_env_override<T>(&self, key: &str, allow_env_override: bool) -> Result<T, ConfigError>

// Existing secret methods remain unchanged
pub fn get_secret<T>(&self, key: &str) -> Result<T, ConfigError>
```

### Migration Pattern

**Before:**
```rust
let api_key = std::env::var("OPENAI_API_KEY")?;
let host = std::env::var("OPENAI_HOST").unwrap_or_else(|_| "https://api.openai.com".to_string());
```

**After:**
```rust
let config = Config::global();
let api_key: String = config.get_secret("OPENAI_API_KEY")?;
let host: String = config.get_param("OPENAI_HOST").unwrap_or_else(|_| "https://api.openai.com".to_string());
```

### Testing Pattern

**Before:**
```rust
// Hard to test - relies on actual env vars
std::env::set_var("OPENAI_API_KEY", "test_key");
let provider = OpenAiProvider::from_env(model)?;
```

**After:**
```rust
// Easy to test - can use config without env vars
let config = Config::new_with_file_secrets(config_path, secrets_path)?;
config.set_secret("OPENAI_API_KEY", Value::String("test_key".to_string()))?;
// Test with config only
let provider = OpenAiProvider::from_config(&config, model)?;

// Or test with env override disabled
let value: String = config.get_param_with_env_override("OPENAI_HOST", false)?;
```

## Files to Migrate

### High Priority (Core Providers)
- `crates/goose/src/providers/openai.rs` - 1 direct env var
- `crates/goose/src/providers/anthropic.rs` - 2 direct env vars
- `crates/goose/src/providers/litellm.rs` - 1 direct env var
- `crates/goose/src/model.rs` - 4 direct env vars

### Medium Priority (System Configuration)
- `crates/goose/src/providers/factory.rs` - Provider creation logic
- `crates/goose-cli/src/commands/configure.rs` - Configuration management
- `crates/goose/src/providers/pricing.rs` - Cache directory config

### Lower Priority (Debugging/Development)
- Various debugging flags and development utilities
- Test utilities and scenario runners

## Benefits After Migration

1. **Testability**: Tests can run without environment variables
2. **Reproducibility**: Configuration state is explicit and controllable
3. **Documentation**: All config keys documented in provider metadata
4. **Validation**: Config values can be validated at load time
5. **Debugging**: Easy to see what configuration is actually being used
6. **Flexibility**: Can disable env var overrides for specific use cases

## Backward Compatibility

All existing environment variables will continue to work exactly as before. The migration is purely internal - the external API remains the same.
