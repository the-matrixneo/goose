---
sidebar_position: 14
title: Unified Configuration
sidebar_label: Unified Configuration
---

This guide explains Goose's unified configuration system. It standardizes how every setting can be provided via:

- Config file (persistent)
- Environment variables
- CLI flags

And lets you inspect the effective value with provenance (where it came from) while redacting secrets by default.

## Key Benefits

- One canonical key per setting (dot notation): `llm.provider`, `llm.model`, `server.port`, etc.

> **One-liner Drop-in Replacements**
>
> Migrating existing config reads should be a one-liner: replace manual env/config parsing with `unified::get` or `unified::get_or`. Example:
>
> ```rust
> // Before
> let model = std::env::var("GOOSE_MODEL").ok().unwrap_or_else(|| "gpt-4o".to_string());
>
> // After (one-liner)
> let model: String = goose::config::unified::get_or("llm.model", "gpt-4o".to_string());
> ```

- Automatic environment mapping with `GOOSE_` prefix: `llm.model` → `GOOSE_LLM_MODEL`
- Clear precedence and observability
- CLI experience that works for all keys (show/get/set/unset) and a global non-persistent overlay

## Canonical Keys and Mapping

The unified configuration system uses canonical dot-notation keys that map to environment variables and config file entries.

### Core Canonical Keys

| Category | Canonical Key | Environment Variable | Description |
|----------|--------------|---------------------|-------------|
| **LLM** | `llm.provider` | `GOOSE_LLM_PROVIDER` | LLM provider (openai, anthropic, etc.) |
| | `llm.model` | `GOOSE_LLM_MODEL` | Model name (gpt-4o, claude-3.5-sonnet, etc.) |
| | `model.temperature` | `GOOSE_MODEL_TEMPERATURE` | Model temperature (0.0-2.0) |
| | `model.context_limit` | `GOOSE_MODEL_CONTEXT_LIMIT` | Override context window size |
| **CLI** | `cli.theme` | `GOOSE_CLI_THEME` | CLI theme (light, dark, ansi) |
| | `cli.show_cost` | `GOOSE_CLI_SHOW_COST` | Show model cost estimates |
| | `cli.min_priority` | `GOOSE_CLI_MIN_PRIORITY` | Tool output verbosity (0.0-1.0) |
| | `cli.show_thinking` | `GOOSE_CLI_SHOW_THINKING` | Show thinking/reasoning output |
| **Server** | `server.host` | `GOOSE_SERVER_HOST` | Server host address |
| | `server.port` | `GOOSE_SERVER_PORT` | Server port (1-65535) |
| | `server.secret_key` | `GOOSE_SERVER_SECRET_KEY` | Desktop auth secret |
| **Security** | `security.allowlist.url` | `GOOSE_SECURITY_ALLOWLIST_URL` | Extension allowlist URL |
| | `security.allowlist.bypass` | `GOOSE_SECURITY_ALLOWLIST_BYPASS` | Bypass allowlist check |
| **Scheduler** | `scheduler.type` | `GOOSE_SCHEDULER_TYPE` | Scheduler type (legacy, temporal) |
| | `scheduler.temporal.bin` | `GOOSE_SCHEDULER_TEMPORAL_BIN` | Path to Temporal binary |
| **Editor** | `editor.api_key` | `GOOSE_EDITOR_API_KEY` | Enhanced code editing API key |
| | `editor.host` | `GOOSE_EDITOR_HOST` | Enhanced code editing API endpoint |
| | `editor.model` | `GOOSE_EDITOR_MODEL` | Enhanced code editing model |
| **Toolshim** | `toolshim.enabled` | `GOOSE_TOOLSHIM_ENABLED` | Enable tool call interpretation |
| | `toolshim.model` | `GOOSE_TOOLSHIM_MODEL` | Model for tool interpretation |
| **Session** | `session.max_turns` | `GOOSE_SESSION_MAX_TURNS` | Max turns without user input |
| | `session.max_tool_repetitions` | `GOOSE_SESSION_MAX_TOOL_REPETITIONS` | Max tool repetitions |
| **Agent** | `agent.mode` | `GOOSE_AGENT_MODE` | Tool execution mode |
| **Router** | `router.tool_selection_strategy` | `GOOSE_ROUTER_TOOL_SELECTION_STRATEGY` | Tool selection strategy |

### Environment Variable Mapping

- Use dot-separated, snake_case keys: e.g., `llm.provider`, `llm.model`, `server.port`
- Environment variables map automatically by converting dots to underscores, uppercasing, and prefixing with `GOOSE_`:
  - `llm.model` → `GOOSE_LLM_MODEL`
  - `providers.openai.api_key` → `GOOSE_PROVIDERS_OPENAI_API_KEY`
  - `cli.show_cost` → `GOOSE_CLI_SHOW_COST`

### Legacy Aliases

Legacy environment variables continue to work but have lower precedence than canonical `GOOSE_*` forms:

| Canonical Variable | Legacy Aliases |
|-------------------|----------------|
| `GOOSE_LLM_PROVIDER` | `GOOSE_PROVIDER`, `PROVIDER` |
| `GOOSE_LLM_MODEL` | `GOOSE_MODEL`, `MODEL` |
| `GOOSE_PROVIDERS_OPENAI_API_KEY` | `OPENAI_API_KEY` |
| `GOOSE_AGENT_MODE` | `GOOSE_MODE` |
| `GOOSE_TOOLSHIM_ENABLED` | `GOOSE_TOOLSHIM` |
| `GOOSE_TOOLSHIM_MODEL` | `GOOSE_TOOLSHIM_OLLAMA_MODEL` |
| `GOOSE_MODEL_TEMPERATURE` | `GOOSE_TEMPERATURE` |
| `GOOSE_SESSION_MAX_TURNS` | `GOOSE_MAX_TURNS` |
| `GOOSE_SECURITY_ALLOWLIST_URL` | `GOOSE_ALLOWLIST` |

**Migration Note:** When updating code or configuration, prefer using canonical names over legacy aliases.

## Precedence

Configuration values are resolved in the following order (highest to lowest):

1. **CLI overlay**: `goose --set KEY=VALUE` (ephemeral, non-persistent)
2. **Environment variables**: canonical `GOOSE_*` first, then legacy aliases (e.g., `OPENAI_API_KEY`)
3. **Config file values**: `config.yaml` and `secrets.yaml`
4. **Defaults**: defined in Goose registry

This means a CLI overlay always wins, environment variables override config files, and config files override defaults.

## CLI Commands: Show, Get, Set, Unset

- **Inspect effective configuration** (redacts secrets by default):
  ```bash
  goose configure --show [--format table|json|yaml] [--filter PREFIX] [--only-changed] [--sources]
  ```

- **Get one value**:
  ```bash
  goose configure --get llm.provider [--raw] [--show-secret]
  ```

- **Persist values**:
  ```bash
  goose configure --set llm.model=gpt-4o
  goose configure --set providers.openai.api_key="sk-..." --secret
  ```

- **Remove values**:
  ```bash
  goose configure --unset llm.model
  goose configure --unset providers.openai.api_key --secret
  ```

## Global Overlay Flag

The `--set` flag can be used globally (before the subcommand) for non-persistent configuration:

```bash
# Override provider for a single invocation
goose --set llm.provider=anthropic configure --get llm.provider --raw

# Use a different model for one session
goose --set llm.model=gpt-4o-mini session --name demo

# Multiple overrides
goose --set llm.provider=openai --set llm.model=gpt-4 session
```

## Secrets Management

- Keys marked as secret are always redacted when displayed unless you pass `--show-secret` to `configure --get`
- Secrets are stored in the system keyring by default
- If the keyring is disabled (`GOOSE_DISABLE_KEYRING` set), they are stored in `secrets.yaml`

## Config File Format

The config file supports canonical keys (preferred) and legacy uppercase keys. Prefer canonical dot-notation going forward:

```yaml
# ~/.config/goose/config.yaml (macOS/Linux)
# %APPDATA%/Block/goose/config/config.yaml (Windows)

# Canonical format (preferred)
llm:
  provider: "openai"
  model: "gpt-4o"

server:
  port: 3000

cli:
  theme: "dark"
  show_cost: true

tracing:
  langfuse:
    url: "https://cloud.langfuse.com"

# Legacy uppercase keys are still accepted but will be phased out
# GOOSE_PROVIDER: "openai"
# GOOSE_MODEL: "gpt-4o"
```

## Programmatic API (for Developers)

The resolver lives at `goose::config::unified` with these main functions:

### Reading Configuration

```rust
use goose::config::unified;

// Get a required value (returns Result)
let provider: String = unified::get("llm.provider")?;

// Get with a default fallback
let port: u16 = unified::get_or("server.port", 3000);

// Get with source metadata
let resolved = unified::resolve::<String>("llm.model")?;
println!("Value: {}, Source: {:?}", resolved.value, resolved.source);
```

### Writing Configuration

```rust
use goose::config::unified;

// Set a regular value
unified::set("llm.model", "gpt-4o")?;

// Set a secret value
unified::set_secret("providers.openai.api_key", "sk-...")?;

// Remove a value
unified::unset("llm.model")?;
```

### Inspecting Configuration

```rust
use goose::config::unified;

// Get effective configuration for UI/debugging
let config = unified::effective_config(
    Some("llm."),     // filter by prefix
    true,              // only_changed
    true               // include_sources
)?;
```

## Migration Examples

### Migrating Environment Variable Reads

```rust
// Before: Direct environment variable
let model = std::env::var("GOOSE_MODEL")
    .unwrap_or_else(|_| "gpt-4o".to_string());

// After: Unified config with canonical key
let model: String = unified::get_or("llm.model", "gpt-4o".to_string());
```

### Migrating Config File Access

```rust
// Before: Direct config file parsing
let config = load_config_file()?;
let provider = config.get("GOOSE_PROVIDER")
    .unwrap_or("openai");

// After: Unified config
let provider: String = unified::get_or("llm.provider", "openai".to_string());
```

### Migrating API Keys

```rust
// Before: Multiple possible sources
let api_key = std::env::var("OPENAI_API_KEY")
    .or_else(|_| std::env::var("GOOSE_OPENAI_API_KEY"))
    .or_else(|_| read_from_keyring("openai_api_key"))?;

// After: Unified config handles all sources
let api_key: String = unified::get("providers.openai.api_key")?;
```

## Desktop/Server API

The server exposes effective config for the UI at:

```
GET /config/effective?filter=llm.&only_changed=true&include_sources=true
```

## Migration Notes

The codebase is migrating from legacy names to canonical keys. Legacy envs/keys will continue to work, but new code should:

- Read via `goose::config::unified::get`/`get_or`
- Prefer canonical keys in `config.yaml`
- Use `GOOSE_*` canonical env names (e.g., `GOOSE_LLM_MODEL`) over aliases

### Deprecation Timeline

1. **Current**: Both canonical and legacy names work
2. **Future**: Legacy names will log deprecation warnings
3. **Later**: Legacy support may be removed in a major version

## See Also

- [Goose CLI Commands](/docs/guides/goose-cli-commands) - configure subcommands and overlay examples
- [Configuration File](/docs/guides/config-file) - paths and examples
- [Environment Variables](/docs/guides/environment-variables) - canonical mapping and aliases
