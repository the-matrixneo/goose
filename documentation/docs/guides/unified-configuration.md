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

Key benefits
- One canonical key per setting (dot notation): llm.provider, llm.model, server.port, etc.
- Automatic environment mapping with GOOSE_ prefix: llm.model → GOOSE_LLM_MODEL
- Clear precedence and observability
- CLI experience that works for all keys (show/get/set/unset) and a global non-persistent overlay

Canonical keys and mapping
- Use dot-separated, snake_case keys: e.g., llm.provider, llm.model, server.port, tracing.langfuse.url, providers.openai.api_key
- Environment variables map automatically by converting dots to underscores, uppercasing, and prefixing with GOOSE_:
  - llm.model → GOOSE_LLM_MODEL
  - providers.openai.api_key → GOOSE_PROVIDERS_OPENAI_API_KEY
- Legacy aliases continue to work (e.g., OPENAI_API_KEY, GOOSE_MODEL) with lower precedence than the canonical GOOSE_* form. Over time prefer switching to canonical names.

Precedence (highest → lowest)
1) CLI overlay: goose --set KEY=VALUE (ephemeral, non-persistent)
2) Environment variables: canonical GOOSE_* first, then legacy aliases (e.g., OPENAI_API_KEY)
3) Config file values (config.yaml and secrets.yaml)
4) Defaults (defined in Goose registry)

Show, get, set, unset
- Inspect effective configuration (redacts secrets by default):
  - goose configure --show [--format table|json|yaml] [--filter PREFIX] [--only-changed] [--sources]
- Get one value:
  - goose configure --get llm.provider [--raw] [--show-secret]
- Persist values:
  - goose configure --set llm.model=gpt-4o
  - goose configure --set providers.openai.api_key="sk-..." --secret
- Remove values:
  - goose configure --unset llm.model
  - goose configure --unset providers.openai.api_key --secret

Global overlay flag (non-persistent)
- Highest precedence for a single invocation. Must be specified before the subcommand:
  - goose --set llm.provider=anthropic configure --get llm.provider --raw
  - goose --set llm.model=gpt-4o-mini session --name demo

Secrets
- Keys marked as secret are always redacted when displayed unless you pass --show-secret to configure --get.
- Secrets are stored in the system keyring by default. If the keyring is disabled (GOOSE_DISABLE_KEYRING set), they are stored in secrets.yaml.

Config file and keys
- The config file supports canonical keys (preferred) and legacy uppercase keys. Prefer canonical dot-notation going forward:

```yaml
# ~/.config/goose/config.yaml (macOS/Linux)
# %APPDATA%/Block/goose/config/config.yaml (Windows)

llm:
  provider: "openai"
  model: "gpt-4o"

server:
  port: 3000

tracing:
  langfuse:
    url: "https://cloud.langfuse.com"

# Legacy uppercase keys are still accepted but will be phased out
# GOOSE_PROVIDER: "openai"
# GOOSE_MODEL: "gpt-4o"
```

Programmatic API (for developers)
- The resolver lives at goose::config::unified with:
  - get::<T>("key"), get_or::<T>("key", default)
  - resolve::<T>("key") → value + source metadata
  - set, set_secret, unset
  - effective_config(filter, only_changed, include_sources) for UI/server

Example:
```rust
use goose::config::unified as config;
let provider: String = config::get_or("llm.provider", "openai".to_string());
let port: u16 = config::get_or("server.port", 3000);
```

Desktop/server API
- The server exposes effective config for the UI at:
  - GET /config/effective?filter=llm.&only_changed=true&include_sources=true

Migration notes
- The codebase is migrating from legacy names (e.g., GOOSE_MODEL) to canonical keys. Legacy envs/keys will continue to work, but new code should:
  - Read via goose::config::unified::get/get_or
  - Prefer canonical keys in config.yaml
  - Use GOOSE_* canonical env names (e.g., GOOSE_LLM_MODEL) over aliases

See also
- Guides → Goose CLI Commands (configure subcommands and overlay examples)
- Guides → Configuration File (paths and examples)
- Guides → Environment Variables (canonical mapping and aliases)
