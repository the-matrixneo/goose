# Unified configuration migration prompt (for Goose)
# Phase 2(b) prompt for Goose (self)

You are Goose working inside this repository. Continue Phase 2 of the unified configuration migration using subagents to accelerate work. Coordinate the following tasks and have your subagents write concise progress summaries to /tmp/gooseconfigoverhaul/<subagent_name>.md after each task chunk.

Ground rules:
- Always prefer canonical keys with unified::get/get_or/resolve and unified::set/set_secret.
- When touching env reads, map them to canonical keys via registry aliases; remove direct std::env::var usage where safe.
- Keep PRs incremental and compiling. Run cargo fmt, clippy script, and relevant unit tests.
- Update docs where user-facing keys change. Avoid breaking changes; use aliases.

Subagents and tasks:
1) registry_agent
   - Expand the registry with any missing keys referenced in CONFIG_REPORT.md and FINAL_* reports, particularly:
     - tracings.otlp.* additional knobs (headers? protocol?), editor.* refinements, planner/embeddings defaults, CLI extras (show_thinking), router context strategy if present.
   - Add validators for URL/port/enums where appropriate.
   - Write summary to /tmp/gooseconfigoverhaul/registry_agent.md with a table of keys added and aliases.

2) reader_migrator
   - Migrate remaining direct reads to unified API:
     - Allowlist in goose-server routes/extension.rs → security.allowlist.url and security.allowlist.bypass
     - CLI theme/show_cost reads in goose-cli and session output → cli.theme, cli.show_cost
     - Scheduler type in goose, goose-cli → scheduler.type
     - Context limit fallbacks in providers/factory.rs and model.rs to use unified keys (model.context_limit, lead/worker/planner.context_limit)
   - Write summary to /tmp/gooseconfigoverhaul/reader_migrator.md with file paths and replacements.

3) server_desktop_agent
   - Ensure /config/effective route remains correct; run just generate-openapi to refresh Desktop API types.
   - Add (or verify) Desktop UI consumes sources/redaction fields; prepare a follow-up note if UI changes are needed.
   - Write summary to /tmp/gooseconfigoverhaul/server_desktop_agent.md.

4) docs_agent
   - Update documentation pages to reference canonical keys and GOOSE_* mappings, noting legacy aliases.
   - Update unified-configuration.md examples to use unified::get/get_or.
   - Write summary to /tmp/gooseconfigoverhaul/docs_agent.md listing pages updated.

5) tests_agent
   - Add unit tests for new validators and a couple of integration assertions for cli/theme and scheduler.type resolution precedence (CLI overlay > env > file > default).
   - Write summary to /tmp/gooseconfigoverhaul/tests_agent.md with test names.

Exit criteria for this phase:
- Majority of env-based reads migrated to unified API for the items above.
- Registry covers keys exercised by the server/CLI/Desktop happy-path flows.
- Docs reflect canonical keys with alias notes.
- All checks green (cargo check/fmt/clippy/test) and Desktop OpenAPI is regenerated.


You are the Goose developer assistant working inside this repository. Begin Phase 2 of the unified configuration migration.

Checklist for the next commit series:

- Registry expansion
  - [ ] Add canonical keys and aliases for: goose mode, router strategy, context strategy, context limits (main/lead/worker/planner), editor API (host, api key, model), CLI theme and show-cost, scheduler type and temporal path, allowlist URL, server host/port (already present, review), planner/embeddings (already scaffolded), tracing OTLP all knobs (endpoint, timeout; already partially scaffolded)
  - [ ] Add validators (enums: goose_mode, router strategy; url: hosts/endpoints; numeric ranges: timeouts, ports)
- Code migration
  - [ ] Grep for std::env::var("GOOSE_") and Config::get_param calls; replace reads with unified::get/get_or using canonical keys
  - [ ] Where config is written, use unified::set or unified::set_secret
  - [ ] Keep temporary fallback to legacy names if key not yet in registry
- CLI
  - [ ] Ensure any bespoke flags map to canonical keys behind the scenes; prefer docs to advertise configure --set
- Server/Desktop
  - [ ] Run `just generate-openapi` to expose /config/effective for the Desktop UI
  - [ ] Add UI to display effective config with filters and sources (follow-up PR in ui/desktop)
- Tests
  - [ ] Unit tests for new keys and validators
  - [ ] Integration tests for configure flows with env, overlay, file precedence
  - [ ] Non-regression tests for server startup, sessions
- Docs
  - [ ] Keep docs/guides/unified-configuration.md in sync
  - [ ] Update environment-variables.md and config-file.md to prefer canonical keys and note aliases/deprecation

Exit criteria: most practical configuration in Goose is resolvable through unified::get/get_or with canonical keys; configure/show reflects the system comprehensively; Desktop view available.

# Goose Unified Configuration — Brainstorm

This document explores what a drop-in, unified configuration system for Goose should look like. It captures the problem framing, observations from the codebase, core design goals, and a proposed model that keeps things elegant, maintainable, reliable, and easy to reason about. It also outlines how this design lends itself to a future `goose configure` subcommand family (show/get/set/unset) and a `goose show-config`-style experience.

This brainstorm is now implemented through Phase 3. See the implementation plan file for the shipped scope and next steps.

## Implementation Status (as of Phase 6)
- ✅ Unified resolver and typed API implemented
- ✅ Static registry with 100+ configuration keys
- ✅ CLI configure subcommands (show/get/set/unset) working
- ✅ 90 keys successfully migrated to unified API (40.0% coverage)
- ✅ Validators for constrained values (ranges, enums, URLs, floats)
- ✅ Backward compatibility via environment variable aliases
- ✅ Discovery tool (`config_discovery.py`) enhanced to track migration progress
- ✅ All major provider configurations migrated
- ✅ Tracing and observability fully integrated
- ✅ Model and lead/worker configurations consolidated
- ✅ MCP and extension configurations integrated
- ✅ Cache, security, and system settings migrated
- ✅ Experiments and extensions using unified API
- ✅ Eliminated mixed access patterns for core configurations

---

## How I arrived at this

Inputs and steps I took to form this proposal:

- Ran the provided discovery utility: `python3 config_discovery.py` from the repository root to extract configuration usages across the codebase.
  - Results highlighted:
    - 1045 total configuration usages
    - 346 unique configuration keys
    - 101 files with configuration
    - By source: ~269 config-file keys, ~63 environment variables, ~21 secrets, ~11 CLI flags
- Skimmed key areas of the codebase related to configuration:
  - `crates/goose/src/config/base.rs` — an existing, robust config module supporting:
    - YAML config file (atomic writes, backup/rotation, recovery)
    - Environment variable overrides (case-mapped)
    - Secrets stored in keyring or YAML file if keyring disabled
    - Smart parsing of env strings (JSON, bools, numbers)
  - `crates/goose-cli/src/cli.rs` — current CLI flags, subcommands, and the `configure` entry point
- Correlated the discovery results with the code patterns to understand the current config surface and pain points: inconsistency of names, uneven coverage across env/config/CLI, and lack of a single-place mapping and observability.

---

## Problem summary

- Configuration is consumed through three separate channels today: config file, environment variables, and command-line flags.
- Keys and names are not consistently unified across those channels (e.g., GOOSE_MODEL vs model vs llm.model; provider-specific env vars like OPENAI_API_KEY, etc.).
- Many config values are not available uniformly in all three channels.
- Observability is limited: it’s hard to see the effective value of a config item and where it came from.

---

## Design goals

- Elegance and consistency
  - One canonical key for each config item, used across all channels.
  - A clear, uniform precedence for merging values.
- Maintainability
  - A single registry of keys (types, defaults, docs, aliases, validation).
  - Easy to add new keys, providers, and features.
- Reliability
  - Typed, validated values with clear errors.
  - Keep existing atomic writes, backups, and secret storage guarantees.
- Developer ergonomics
  - One-liner lookups to replace existing patterns.
  - Minimal changes required across the codebase.
- Observability and UX
  - Show effective values with their source (CLI/env/file/default).
  - Redact secrets by default; reveal explicitly if needed.

---

## Proposed model

### 1) Canonical keys and naming

- Canonical format: snake_case with dot-separated namespaces.
  - Examples: `llm.provider`, `llm.model`, `server.port`, `tracing.langfuse.url`, `providers.openai.host`.
- Config file organization: YAML using nested structure or flat keys with dots (whichever fits current `base.rs` best).
- Environment variables:
  - Standardize on `GOOSE_` prefix and convert dots to underscores, uppercase: `llm.model` → `GOOSE_LLM_MODEL`.
  - Keep compatibility aliases for existing env names (e.g., `OPENAI_API_KEY`, `GOOSE_MODEL`, `PROVIDER`) via a registry (with deprecation warnings).
- CLI:
  - Add a universal setter: `--set KEY=VALUE` (repeatable), so every key is settable via CLI without bespoke flags.
  - Dedicated flags for popular items may still exist or be auto-generated later, but `--set` keeps scope tight and uniform.

### 2) Central registry (the single source of truth)

A static registry (Rust const/static, with macros for ergonomics) declaring for each key:

- `key`: canonical key, e.g., `llm.model`
- `type`: string | bool | u32 | f64 | map | list | any
- `default`: optional default value
- `help`: short and long help text
- `secret`: bool — controls redaction and storage location
- `env_aliases`: legacy env names to accept (e.g., `OPENAI_API_KEY`)
- `cli_aliases`: legacy flags to accept (optional)
- `deprecated_aliases`: and migration hints
- `validator`: enum set, ranges, URL/path checks as needed
- `tags/scopes`: to organize docs and show-config filtering (e.g., `providers/openai`)

Benefits: one place to add keys, define types and defaults, deprecate legacy names, and generate help.

### 3) Sources and precedence

Uniform merge order (highest → lowest):

1. CLI `--set KEY=VALUE` (universal setter)
2. CLI dedicated flags mapped to keys (if present)
3. Environment variables (prefer `GOOSE_*`; fall back to aliases)
4. Config file values
5. Registry defaults

Secrets follow the same precedence but are redacted in displays.

### 4) Overlays and scopes (optional, later)

- Potential overlays in the future: project-local config files, etc.
- Start simple with the existing global config path from `base.rs`.

### 5) Unified API (drop-in)

Module: `goose_config` (or `goose-config` crate) providing:

- Reads:
  - `get::<T>("llm.model") -> Result<T, Error>`
  - `get_or::<T>("todo.max_chars", default: T) -> T`
  - `resolve::<T>("server.port") -> Result<ValueWithSource<T>, Error>` where `ValueWithSource` includes `{ value, source, key, was_default, ... }`
- Secrets:
  - `get_secret::<T>("openai.api_key") -> Result<T, Error>`
- Writes:
  - `set("llm.model", json!("gpt-4o"))` → updates YAML config (non-secret)
  - `set_secret("openai.api_key", json!("..."))` → updates keyring or secrets.yaml

Example of drop-in replacement:

```rust
// Before (env + parse + default):
std::env::var("GOOSE_TODO_MAX_CHARS")
    .ok()
    .and_then(|s| s.parse().ok())
    .unwrap_or(DEFAULT_TODO_MAX_CHARS)

// After (one-liner):
let max = goose_config::get_or::<usize>("todo.max_chars", DEFAULT_TODO_MAX_CHARS);
```

### 6) CLI integration

- Extend `goose configure` with:
  - `goose configure show` — print effective config with source, redact secrets
  - `goose configure get KEY` — print value (redacted if secret unless forced)
  - `goose configure set KEY=VALUE` — persist to file or secrets store depending on key
  - `goose configure unset KEY` — remove from file or secrets store
- Global option: allow `--set KEY=VALUE` on the main CLI to apply ephemeral overrides for a single invocation/session.

### 7) Observability ("show-config")

- `show` supports:
  - Formats: table, json, yaml
  - Filters: `--filter llm.`, `--only-changed`, `--sources`
  - Provenance per key (CLI/env/file/default), and alias usage/deprecation notes
- Programmatic API to return the same data structure for server/desktop UI.

### 8) Validation, typing, and reliability

- Registry-driven validation: enums, ranges, URL/path checks
- Typed parsing (strings, numbers, bools, JSON objects/arrays)
- Clear error messages with origin

### 9) Backward compatibility

- Accept legacy env vars and flags via alias rules with deprecation warnings.
- Encourage canonical names in docs and error messages.
- Optionally provide a strict mode later that disallows aliases.

### 10) Performance and caching

- Cache resolved values after the first merge, with explicit invalidation on config changes or `--set`.
- Optionally add a file watcher later.

### 11) Secrets policy

- Mark secrets in the registry; `set_secret` writes to keyring/file.
- CLI/env allowed to set ephemeral secrets; UI redacts by default.

### 12) Documentation and discovery

- Use the registry to generate docs and CLI help for keys.
- `goose configure show --describe KEY` (future) to print help, type, default, aliases.

### 13) Extensibility

- Adding a new provider or feature is a matter of adding canonical keys to the registry (with aliases and validation), and replacing reads with one-liners.

---

## Example canonical mappings

- `llm.provider` ↔ `GOOSE_LLM_PROVIDER` (aliases: `GOOSE_PROVIDER`, `PROVIDER`)
- `llm.model` ↔ `GOOSE_LLM_MODEL` (aliases: `GOOSE_MODEL`, `MODEL`)
- `providers.openai.api_key` (secret) ↔ `GOOSE_PROVIDERS_OPENAI_API_KEY` (alias: `OPENAI_API_KEY`)
- `server.port` ↔ `GOOSE_SERVER_PORT` (alias: `PORT`)
- `tracing.langfuse.url` ↔ `GOOSE_TRACING_LANGFUSE_URL` (alias: `LANGFUSE_URL`)

---

## Why this fits Goose

- Builds on the strengths already present in `base.rs` (atomic writes, recovery, secret storage) while adding:
  - One canonical key per item
  - A registry for types/defaults/aliases
  - Uniform precedence
  - One-liner fetches
  - Strong observability and UX (show-config)
- Scales to more providers and features without additional plumbing.

---

## Decisions (for v1)

- Make `GOOSE_` prefix the standard for environment variables.
- Start with a universal `--set KEY=VALUE` to keep CLI scope tight.
- Implement the registry as a Rust const/static with macros for ergonomics.

---

## Migration & minimal-change replacement

- Existing code that reads config values should become one-liners:
  - `goose_config::get::<T>("key")?` or `goose_config::get_or::<T>("key", default)`
- Legacy env vars and CLI flags still work (via aliases), but log deprecation notices and show canonical equivalents.
- Incremental adoption is possible: start with high-value keys (provider/model/server), then expand.

---

## Open questions (intentionally deferred)

- Project/workspace overlay files timing and scope
- Auto-generating dedicated flags from the registry (beyond `--set`)
- Strict mode rollout timing for removing legacy names
