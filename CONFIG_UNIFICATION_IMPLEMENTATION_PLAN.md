# Goose Unified Configuration — Implementation Plan
[Status: Phase 1 shipped]

- Implemented unified resolver + static registry + CLI overlay
- Added non-interactive `goose configure` subcommands: show/get/set/unset
- Wired call sites (session + web + server.port) to unified reads
- Tests landed: module unit tests for registry/env precedence/parsing/errors; CLI integration tests for show/get/set/unset, redaction, sources, overlay precedence
- Docs updated (brainstorm reflects shipped state); module-level docs in unified/mod.rs



This plan describes how to technically implement a drop‑in, unified configuration system with minimal disruption to the codebase. It reflects these choices:

Note: The global CLI overlay flag `--set KEY=VALUE` must be placed before the subcommand (e.g., `goose --set llm.model=gpt-4o configure --get llm.model`). This avoids long option name conflicts with `configure --set` which persists values.


- Make `GOOSE_` prefix the standard for environment variables
- Start with `--set KEY=VALUE` to keep CLI scope tight
- Implement the registry as a Rust const/static with macros for ergonomics

No timelines are included. This is a technical plan only. Do not implement yet.

---

## High-level architecture

Introduce a new top-level module/crate (e.g., `goose_config`) that orchestrates configuration resolution across:

- CLI overlays (from `--set KEY=VALUE`)
- Environment variables (`GOOSE_*` first, then alias envs)
- Config file (via existing `goose::config::base::Config`)
- Registry defaults

It provides a minimal, drop‑in read/write API and a provenance-aware inspection API. Secrets leverage existing keyring/file management. The `goose configure` command is extended with `show`, `get`, `set`, and `unset` subcommands that use the same APIs.

---

## Components

### 1) The Registry

A compile-time registry (Rust const/static) describing all canonical keys and their metadata.

- Structure per key (conceptual):
  ```rust
  struct KeySpec {
      key: &'static str,                // e.g., "llm.model"
      ty: ValueType,                    // String, Bool, U32, F64, Map, List, Any
      default: Option<serde_json::Value>,
      help_short: &'static str,
      help_long: &'static str,
      secret: bool,
      env_aliases: &'static [&'static str],   // e.g., ["OPENAI_API_KEY", "MODEL"]
      cli_aliases: &'static [&'static str],   // e.g., ["--model", "--provider"] (optional)
      deprecated_aliases: &'static [&'static str],
      validator: Option<fn(&serde_json::Value) -> Result<(), String>>, // optional
      tags: &'static [&'static str],     // e.g., ["providers/openai", "llm"]
  }
  ```
- Helper macro(s) to define keys ergonomically:
  ```rust
  key_spec! {
    key: "llm.model",
    ty: String,
    default: "gpt-4o",
    help: "Model to use",
    env_aliases: ["GOOSE_MODEL", "MODEL"],
    tags: ["llm"],
  }
  ```
- Organization: group by namespace (`llm.*`, `server.*`, `providers.*`, `tracing.*`).
- Scope for v1: cover high‑value keys first (provider/model/server/tracing/secrets) and include aliases discovered by `config_discovery.py`.

### 2) Key normalization and mapping

- Canonical → Env: `llm.model` → `GOOSE_LLM_MODEL` (dots → underscores, uppercased, prefixed with `GOOSE_`).
- Accept alias envs from the registry (e.g., `OPENAI_API_KEY`).
- Accept legacy CLI aliases if present (but v1 focuses on `--set`).

### 3) Resolver and precedence

- Inputs:
  - `cli_overrides: HashMap<String, serde_json::Value>` (from `--set`)
  - `env`: `std::env` snapshot filtered to known aliases and canonical forms
  - `file`: values provided by `goose::config::base::Config::load_values()` and `load_secrets()`
  - `defaults`: from registry
- Precedence (highest → lowest): CLI → env → file → default
- Output: a merged, typed view for each key:
  ```rust
  struct Resolved<T> {
      key: &'static str,
      value: T,
      source: Source,  // Cli | Env { name } | File { path } | Default
      is_secret: bool,
      used_alias: Option<&'static str>,
  }
  ```
- Cache the merged map for fast reads; expose an invalidation hook.

### 4) Public API (drop‑in)

- Reads:
  - `get::<T>(key: &str) -> Result<T, Error>` — typed, uses registry when available; errors if not set and no default
  - `get_or::<T>(key: &str, default: T) -> T` — typed with caller default
  - `resolve::<T>(key: &str) -> Result<Resolved<T>, Error>` — value with provenance
- Secrets:
  - `get_secret::<T>(key: &str) -> Result<T, Error>` — same precedence
- Writes:
  - `set(key: &str, value: serde_json::Value) -> Result<()>` — writes to config file (non-secret)
  - `set_secret(key: &str, value: serde_json::Value) -> Result<()>` — writes to keyring/file
  - `unset(key: &str) -> Result<()>` and `unset_secret(key: &str) -> Result<()>`

Note: The API sits atop the existing `Config` in `base.rs`, which already provides robust persistence and secret management.

### 5) CLI integration — extend `goose configure`

Add subcommands under the existing `goose configure`:

- `goose configure show [--format json|yaml|table] [--filter PREFIX] [--only-changed] [--sources]`
  - Displays effective configuration including source and alias usage; redacts secrets by default
- `goose configure get KEY [--raw] [--show-secret]`
  - Prints the value; secrets redacted unless `--show-secret`
- `goose configure set KEY=VALUE [--secret]`
  - Persists value; if `--secret` or registry marks as secret → keyring/file
  - For convenience, `set` can accept multiple pairs: `set a=b c=d`
- `goose configure unset KEY [--secret]`
  - Removes from config store / secrets store

Global CLI overlay for all commands:

- `--set KEY=VALUE` (repeatable) to apply runtime overrides for that invocation (not persisted)

Parsing rules for KEY=VALUE:

- Use the same parsing rules as `base.rs` env parsing: try JSON first, then bool/number, else string.

### 6) Observability data model

- Provide programmatic access to the same data model used by `configure show`:
  ```rust
  struct EffectiveEntry {
      key: String,
      value: serde_json::Value,  // redacted if secret
      secret: bool,
      source: Source,
      alias_used: Option<String>,
      has_default: bool,
      description: Option<String>,
  }
  ```
- `fn effective_config(filter: Option<&str>, only_changed: bool) -> Vec<EffectiveEntry>`

### 7) Validation and error handling

- Registry-driven validators enforce:
  - Enum value sets, numeric ranges, URL/path formats
- Error messages include source and key name with suggestions:
  - If alias used, log a one-time deprecation warning and suggest the canonical name

### 8) Backward compatibility

- Support legacy env vars and CLI flags via registry aliases.
- Emit deprecation warnings when aliases are used.
- Document canonical names in help/errors; keep aliases indefinitely or deprecate on a timeline later.

### 9) Performance and caching

- Cache resolved map on first use; invalidate on any `set`/`unset` or when `--set` overlays change.
- No file watcher in v1; rely on explicit invalidation points (e.g., after `configure set`).

### 10) Secrets handling

- Keys marked `secret: true` default to `set_secret`/`get_secret` semantics.
- `configure get` redacts by default unless `--show-secret` is explicitly passed.

---

## Minimal disruption migration strategy

1. Introduce `goose_config` module/crate without modifying call sites.
2. Add `--set KEY=VALUE` global overlay and extend `goose configure` to support `show|get|set|unset` using the new module.
3. Migrate high-value reads to one-liners:
   - Provider/model/server/observability-related keys where current code uses env or config directly.
   - Replace with `goose_config::get_or::<T>("key", default)` or `get::<T>("key")?`.
4. Expand registry coverage incrementally using `config_discovery.py` as a guide.
5. Keep legacy envs/flags through aliases; start logging deprecation messages where appropriate.

Where to swap first (low risk, high value):

- LLM selection: `llm.provider`, `llm.model`, related provider endpoints/timeouts
- Server configuration: `server.host`, `server.port`
- Telemetry/tracing: `tracing.langfuse.url`, `tracing.otlp.endpoint`, timeouts
- Session behavior: `session.max_turns`, `session.max_tool_repetitions`

Each swap should be a one-liner, e.g.:

```rust
let provider = goose_config::get_or::<String>("llm.provider", "openai".into());
```

---

## Testing and coverage plan

Ensure the new system works end-to-end and does not break existing behavior.

### Unit tests (module-level)

1) Registry tests
- Validate that key specs compile, defaults are well-formed, and type metadata matches.
- For keys with validators, feed valid/invalid values and assert outcomes.

2) Resolver precedence tests
- For a given key, set different values in CLI overlay, env, file, and default; assert effective value and source follow precedence.
- Include alias env tests (e.g., `OPENAI_API_KEY`) and verify deprecation notice.

3) Parsing tests for `--set KEY=VALUE`
- JSON object/array strings → parsed correctly
- Booleans/numbers/null → parsed correctly
- Edge cases: whitespace, strings that look like numbers

4) Secret handling tests
- Ensure secrets resolve from env/keyring/file in the right order.
- `configure get` redacts by default; `--show-secret` reveals.

5) Error reporting tests
- Unknown key handling: helpful error with suggestions.
- Type mismatch: clear message indicating expected type and source.

### Integration tests (CLI)

1) `goose configure show`
- Verify formats (table/json/yaml), filtering, `--only-changed`, and `--sources` behavior.

2) `goose configure set/unset/get`
- `set` non-secret → persists in YAML; `unset` removes it; `get` returns persisted value.
- `set --secret` or secret key → persisted in keyring/file; `get` redacts; `--show-secret` reveals.

3) Global `--set KEY=VALUE` overlays
- Run a representative command with `--set` and verify runtime value takes precedence without persisting.

4) Backward compatibility
- Set legacy env vars (e.g., `OPENAI_API_KEY`) and confirm effective canonical key resolves.
- If both legacy and canonical are set, confirm precedence (canonical env via `GOOSE_*` should win if both are present; otherwise highest precedence wins as defined).

### Non-regression tests

- Ensure that existing flows (sessions, providers, server startup) behave unchanged when no new config is supplied.
- Compare before/after effective values for a small matrix of scenarios.

### Property tests (optional)

- Fuzz key names and values for parser robustness.
- Round-trip tests for set → show → get consistency.

---

## Developer ergonomics and safeguards

- Lints or code review checklist items to discourage direct `std::env::var` for configuration.
- Helper macros (optional) for declaring and using canonical keys in code.
- Docs: add a section describing canonical keys, `GOOSE_` env mapping, and examples of `--set` usage.

---

## Future enhancements (out of scope for v1)

- Project/workspace overlay config files
- Auto-generate dedicated CLI flags from the registry (beyond `--set`)
- Strict mode (disallow aliases entirely)
- Live reload via file watching
- OpenAPI exposure for desktop/server to show effective config in UI

---

## Summary

This plan introduces a unified, registry-driven configuration system with a simple, drop‑in API, consistent precedence, and strong observability. It builds directly on the existing `base.rs` storage semantics, minimizes disruption by supporting aliases and one‑liner replacements, and provides a comprehensive testing strategy to ensure reliability and backward compatibility.

## Phase 1 deliverables (landed)

- Unified resolver and typed API: resolve/get/get_or/get_secret/set/set_secret/unset
- Registry keys with defaults and secret flags
- Precedence: CLI overlay > Env (canonical then aliases) > File > Default
- CLI: configure show/get/set/unset with redaction and provenance
- Call sites: session + web + server.port use unified reads
- Tests: unit (unified) + integration (goose-cli) + non-regression for aliases

## Next steps (Phase 1(b)/2 scope)

- Documentation: finalize module docs; expand CLI examples in README and help text
- Validators: scaffold registry-driven validators (enum/range/URL) returning Ok(()) to start
- Alias telemetry: optional one-time notice when alias env is used (deferred behind a feature flag)
- Expand registry coverage: add more keys from CONFIG_REPORT.md incrementally (server.host, tracing/OTLP, planner, embeddings)
- Desktop/server API: expose effective config over OpenAPI for UI (deferred)


---

## Phase 2 migration prompt (for Goose)

You are Goose working inside the goose repository. Your goal is to migrate the codebase to the unified configuration system so every relevant setting can be set consistently via config file, environment variables, and CLI flags.

Constraints and principles:
- Canonical keys: use dot-notation (e.g., llm.provider, llm.model, server.port, tracing.langfuse.url)
- Env mapping: prefer canonical GOOSE_* variables first (dots→underscores, uppercase), accept legacy aliases as defined in the registry
- Precedence: CLI overlay (--set) > Env (GOOSE_* then aliases) > File > Default
- Read paths should use goose::config::unified::{get,get_or,resolve}
- Writes should use goose::config::unified::{set,set_secret,unset}
- Secrets: mark/read/write using registry; redact by default in displays
- Backward compatibility: ensure existing flows keep working; keep legacy names via aliases; add deprecation notes in docs (not hard errors)

High-level tasks:
1) Registry coverage expansion
   - Add canonical keys for remaining settings discovered in the reports (CONFIG_REPORT.md, FINAL_*_CONFIG_REPORT.md)
   - Include reasonable defaults and env aliases for legacy variables
   - Add validators for high-risk keys (ports, timeouts, URLs, enums)

2) Migrate reads
   - Search for std::env::var("GOOSE_") and direct Config::get_param usages across crates
   - Replace with unified::get or unified::get_or using canonical keys
   - Keep fallback to legacy env/config if a canonical key is not yet in the registry (temporary)

3) Migrate writes and CLI flags
   - Prefer configure --set/--unset for persistence where possible
   - For ad-hoc flag setters, route to unified::set/set_secret
   - If dedicated flags exist (e.g., --model), map them to canonical keys but favor the universal --set in docs

4) Server/Desktop
   - Ensure /config/effective is in OpenAPI; run `just generate-openapi`
   - Update Desktop UI to display effective config with sources and filtering

5) Tests
   - Add unit tests for new registry entries (defaults, validators)
   - Add integration tests for configure flows covering new keys and alias envs
   - Add non-regression tests for critical flows (sessions, server startup)

6) Docs
   - Keep Guides → Unified Configuration in sync with registry
   - Document new canonical keys and common aliases
   - Note deprecation of legacy variable names in examples

Execution checklist (iterate):
- [ ] Expand registry with next tranche of keys (server.host, tracing.otlp.*, goose_mode, router strategy, lead/worker, editor API settings)
- [ ] Swap read sites to unified::get/get_or for those keys
- [ ] Add/extend tests for those keys (env, file, overlay precedence)
- [ ] Update docs/examples to show canonical keys and GOOSE_* mappings
- [ ] Repeat until coverage is “most keys used in practice”

Exit criteria for Phase 2:
- Majority of configuration reads use the unified API
- Registry covers all frequently used keys with aliases
- configure --show provides a comprehensive overview for users; desktop shows the same
- Documentation clearly points to canonical keys and the unified flow
