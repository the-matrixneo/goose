# Goose Unified Configuration — Implementation Plan
[Status: Phase 1 shipped]

[Status update: Phase 2 — in progress]

Changes landed in this commit series (Phase 2 kick-off):

- Registry expansion (unified/config/unified/registry.rs)
  - Added canonical keys and aliases for:
    - agent.mode (GOOSE_MODE) with enum validator
[Phase 2 status update — partial complete]

- Implemented in this pass:
  - Migrated allowlist to unified: security.allowlist.url, security.allowlist.bypass in goose-server
  - Unified CLI preferences: cli.theme, cli.show_cost, cli.show_thinking in goose-cli render paths
  - Scheduler type migration in goose-cli schedule commands via scheduler.type
  - Registry expanded further: providers.openai.api_key; added cli.show_thinking
  - Regenerated Desktop OpenAPI schema; /config/effective present
  - All unit/integration tests pass (provider E2E requiring external services still flaky/skipped)

- Remaining for Phase 2 closeout:
  - Migrate remaining env reads listed in CONFIG_BRAINSTORM “reader_migrator” (model.temperature/toolshim, editor.* in MCP dev server)
  - Add docs updates: environment-variables.md and unified-configuration.md reflect cli.* keys and allowlist keys, with alias notes
  - Quick integration tests for cli.theme and scheduler.type precedence (overlay/env/file/default)


    - router.tool_selection_strategy (GOOSE_ROUTER_TOOL_SELECTION_STRATEGY) with enum validator
    - server.host, server.port (port validator), server.secret_key (GOOSE_SERVER__SECRET_KEY, secret)
    - security.allowlist.url (GOOSE_ALLOWLIST, URL validator) and security.allowlist.bypass (GOOSE_ALLOWLIST_BYPASS)
    - tracing.langfuse.url (URL validator), tracing.otlp.endpoint, tracing.otlp.timeout_ms
    - model.context_limit, lead.context_limit, worker.context_limit (numeric validators)
    - planner.provider/model/context_limit
    - embeddings.provider/model
    - editor.api_key (secret), editor.host (URL validator), editor.model
    - cli.theme, cli.show_cost, cli.min_priority
    - scheduler.type (enum validator), scheduler.temporal.bin
  - Introduced simple validator helpers (enum/url/port/range)

- Code migration
  - Tool router strategy now resolves via unified config: router.tool_selection_strategy
  - Vector embedding provider/model selection now resolves via unified config: embeddings.provider, embeddings.model

- Tests
  - Added unit assertions for enum validators (agent.mode, scheduler.type)

What remains in Phase 2:
- Expand registry with remaining high‑value keys from reports (goose mode already done; add router/context strategies if separate, editor/planner fine‑tuning, tracing/OTLP extra knobs if any left)
- Migrate more read paths to unified API:
  - Allowlist enforcement (server) → use security.allowlist.*
  - CLI theme/show-cost reads → unify behind cli.*
  - Scheduler type reads in goose-cli and others → use scheduler.type
  - Model/temperature/toolshim reads where appropriate
- Desktop/Server: run `just generate-openapi`, plumb /config/effective to Desktop where needed
- Docs: update guides to reference canonical keys and GOOSE_* mappings

Execution checklist (Phase 2):
- [x] Expand registry with next tranche of keys (agent.mode, router strategy, editor API, scheduler, allowlist, context limits, tracing/OTLP)
- [x] Swap read sites to unified::get/get_or for some keys (router strategy, embeddings)
- [ ] Swap additional read sites (allowlist, cli.theme/show_cost, scheduler.type, context_limit fallbacks)
- [x] Add/extend tests for validators
- [ ] Update docs/examples to show canonical keys and GOOSE_* mappings
- [ ] Run `just generate-openapi` and verify Desktop consumes /config/effective

---

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

---

## Phase 3 prompt (for Goose)

You are Goose working inside this repository. Continue the unified configuration migration (Phase 3) using subagents to accelerate work. Partition the remaining tasks, run them incrementally, and have each subagent write a concise progress summary to /tmp/gooseconfigoverhaul/<subagent_name>.md after each task chunk.

Ground rules:
- Always prefer canonical keys with unified::get/get_or/resolve and unified::set/set_secret
- Map any remaining direct env reads to canonical keys via registry aliases; remove direct std::env::var usage where safe
- Keep PRs incremental and compiling; run cargo fmt, clippy script, and unit/integration tests relevant to touched areas
- Update docs where user‑facing keys change; avoid breaking changes by using aliases
- When tests depend on external services, mark them as ignored in CI and focus on unit/integration that do not require creds

Subagents and tasks:
1) registry_agent
   - Fill remaining registry gaps identified in reports (temperature, toolshim, editor.*, otlp headers/protocol if present)
   - Add/adjust validators (ranges for temperature, booleans, urls)
   - Summary: /tmp/gooseconfigoverhaul/registry_agent.md (include table of keys/aliases)

2) reader_migrator
   - Migrate remaining direct reads to unified API:
     - model.rs temperature/toolshim/ollama model envs → canonical keys (e.g., model.temperature, toolshim.enabled, toolshim.model)
     - MCP developer editor models in goose-mcp → editor.* keys
     - providers/openai embeddings env to embeddings.model via unified
   - Summary: /tmp/gooseconfigoverhaul/reader_migrator.md (paths and replacements)

3) server_desktop_agent
   - Confirm /config/effective route parameters cover filter/only_changed/include_sources; ensure Desktop consumes sources/redaction fields
   - Create follow‑up TODOs if Desktop needs UI changes (lightweight)
   - Summary: /tmp/gooseconfigoverhaul/server_desktop_agent.md

4) docs_agent
   - Update docs to reference canonical keys and GOOSE_* mappings, noting legacy aliases
   - Ensure unified-configuration.md examples use unified::get/get_or and show precedence
   - Summary: /tmp/gooseconfigoverhaul/docs_agent.md (pages updated)

5) tests_agent
   - Add unit tests for new validators and integration assertions for cli.theme and scheduler.type resolution precedence (CLI overlay > env > file > default)
   - Add a couple of quick integration tests for allowlist bypass and URL resolution via unified
   - Summary: /tmp/gooseconfigoverhaul/tests_agent.md (test names)

Exit criteria for Phase 3:
- All remaining high‑value env reads migrated to unified API
- Registry covers keys used by server/CLI/Desktop happy‑path flows including editor/toolshim/temperature
- Docs reflect canonical keys with alias notes
- All checks green (cargo check/fmt/clippy/test) and OpenAPI regenerated

---

## Phase 3 Summary (Completed)

### Achievements
- ✅ Registry expanded with 8 new keys and validators
- ✅ 18 configuration keys successfully migrated to unified API
- ✅ F64 type added for floating-point configuration
- ✅ Temperature and OTLP protocol validators implemented
- ✅ Discovery tool (`config_discovery.py`) enhanced to track unified configuration usage
- ✅ All code compiles and tests pass

### Migration Statistics (via config_discovery.py)
- **Total configuration usages:** 619
- **Unique keys:** 166
- **Unified API migrations:** 18 keys
- **Remaining legacy env vars:** 57 keys
- **Remaining config file params:** 70 keys

### Keys Successfully Migrated
- **CLI:** `cli.show_cost`, `cli.show_thinking`, `cli.theme`
- **Editor:** `editor.api_key`, `editor.host`, `editor.model`
- **Model:** `model.context_limit`, `model.temperature`
- **Toolshim:** `toolshim.enabled`, `toolshim.model`
- **Security:** `security.allowlist.bypass`, `security.allowlist.url`
- **Server:** `server.port`
- **Session:** `session.max_turns`, `session.max_tool_repetitions`
- **Scheduler:** `scheduler.type`
- **Embeddings:** `embeddings.provider`, `embeddings.model`
- **Other:** `ollama.host`

### Lessons Learned
1. **Subagent coordination:** Parallel execution can timeout; use sequential for complex migrations
2. **Discovery tool importance:** Essential for tracking progress and validating work
3. **Incremental approach:** Small, focused migrations are more successful than large batches
4. **Testing strategy:** Run `cargo check` frequently; use discovery tool to validate migrations

---

## Phase 4 Summary (Completed)

### Achievements
- ✅ 69 new configuration keys migrated (216% of target)
- ✅ Total of 87 unified configuration keys (174% of target)
- ✅ 41.4% of all configuration now using unified API
- ✅ All major provider configurations migrated
- ✅ Tracing and observability fully integrated
- ✅ Model and lead/worker configurations consolidated
- ✅ F64 type added for floating-point configuration
- ✅ All code compiles and passes quality checks

### Keys Successfully Migrated in Phase 4
- **Provider API Keys & Tokens**: 22 keys across all major providers
- **Provider Hosts & Endpoints**: 15 keys with URL validation
- **Provider Timeouts & Retries**: 12 keys with appropriate validators
- **Tracing Configuration**: 5 keys for Langfuse and OTLP
- **System Configuration**: 15 keys including security, scheduler, and agent settings
- **Model Configuration**: 8 keys for editor, embeddings, and toolshim

---

## Phase 5 Summary (Completed)

### Achievements
- ✅ 18 new configuration keys migrated
- ✅ Total of 76 unified configuration keys (35.5% coverage)
- ✅ Lead/worker configurations fully migrated
- ✅ MCP and extension configurations integrated
- ✅ Provider command configurations completed
- ✅ Cache, security, and system settings migrated
- ✅ All code compiles and passes quality checks

### Keys Successfully Migrated in Phase 5
- **Lead/Worker Settings**: Complete migration in factory.rs
- **Provider Commands**: claude_code.command, gemini_cli.command
- **MCP Configuration**: context_file_names, google_drive settings, working_dir
- **System Settings**: cache.dir, security.disable_keyring, vector_db.path
- **Recipe Configuration**: github_repo_config_key, timeouts, path
- **Display Settings**: no_color, random_thinking_messages
- **Debug Flags**: claude_code_debug, gemini_cli_debug

### Lessons Learned
- Discovery tool needs refinement for accurate unified API tracking
- Many configurations were duplicates across providers
- Some system variables (HOME, PATH, USER) should remain as environment variables
- Realistic targets are better than ambitious ones

---

## Phase 6 Summary (Completed)

### Achievements
- ✅ 90 configuration keys migrated to unified API (40.0% coverage)
- ✅ Eliminated mixed access patterns for core configurations
- ✅ Completed partial migrations for llm.model and llm.provider
- ✅ Migrated all lead/worker configurations in factory.rs
- ✅ Verified experiments and extensions already using unified API
- ✅ All code compiles and passes quality checks

### Keys Successfully Migrated in Phase 6
- **Core Configurations**: Completed migration of partially-migrated keys
- **Lead/Worker Settings**: Full migration in factory.rs
- **Experiments/Extensions**: Verified already using unified API
- **Mixed Access Patterns**: Eliminated redundant config access

---

## Phase 7 Prompt (Comprehensive Migration to 95%+ Coverage)

You are Goose working inside this repository. Execute Phase 7 to achieve near-complete unified configuration coverage (95%+).

### Starting Context
- **Current state:** 90 keys migrated to unified API (40.0% coverage)
- **Remaining keys:** ~135 keys in legacy systems
- **Discovery tool:** Use `python3 config_discovery.py` to track progress
- **Working directory:** /Users/tlongwell/Development/goose6

### Primary Objectives
1. **Achieve 95%+ unified API coverage** (target: 214+ keys)
2. **Migrate ALL environment variables** (except system vars like HOME, PATH, USER)
3. **Migrate ALL config file parameters** (except test-only configs)
4. **Migrate ALL secret storage keys**
5. **Eliminate all direct config_get/set_param calls**
6. **Ensure zero compilation errors and maintain tests**

### Execution Strategy - What Works Best

#### Subagent Best Practices (Proven Successful)
- **Use 15-minute timeouts** - Most tasks complete within this window
- **Parallel execution for independent tasks** - 3-4 subagents maximum
- **Sequential for complex interdependent migrations**
- **Always create summary files** in `/tmp/gooseconfigoverhaul/phase6/`
- **Run discovery tool first and last** to measure progress

#### Effective Task Patterns
1. **Discovery First**: Always start with a discovery subagent to establish baseline
2. **Batch by Category**: Group similar configurations (e.g., all provider settings)
3. **Test Frequently**: Include compilation checks in each subagent task
4. **Document Changes**: Have subagents write detailed summaries

### Phase 6 Task Breakdown

#### 1. **phase6_discovery** (Run First, Alone - 5 min timeout)
```
- Run `python3 config_discovery.py --output /tmp/gooseconfigoverhaul/phase6/baseline.md`
- Analyze current 76 unified keys vs 214 total
- Identify quick wins (simple 1:1 mappings)
- Create prioritized list in `/tmp/gooseconfigoverhaul/phase6/targets.md`
- Focus on configs with 3+ usages for maximum impact
```

#### 2. **experiments_extensions_batch** (15 min timeout)
```
- Migrate `experiments` config structure to `experiments.*` namespace
- Migrate `extensions` config structure to `extensions.*` namespace
- These are in config/experiments.rs and config/extensions.rs
- Update all get_param/set_param calls
- Test with `cargo check -p goose`
- Summary: `/tmp/gooseconfigoverhaul/phase6/experiments_extensions.md`
```

#### 3. **remaining_secrets_batch** (15 min timeout)
```
- Complete migration of remaining API keys in secret storage
- Focus on provider keys not yet migrated
- Ensure proper secret flag in registry
- Update all secret_get/secret_set calls
- Summary: `/tmp/gooseconfigoverhaul/phase6/secrets_migration.md`
```

#### 4. **config_file_params_batch** (15 min timeout)
```
- Migrate high-frequency config file parameters
- Focus on test configurations (test_key, complex_key, etc.)
- Update base.rs test configurations
- Ensure backward compatibility
- Summary: `/tmp/gooseconfigoverhaul/phase6/config_params.md`
```

#### 5. **validation_and_cleanup** (10 min timeout)
```
- Run final discovery tool
- Compile all crates: `cargo check --workspace`
- Run formatter: `cargo fmt --all`
- Run clippy: `./scripts/clippy-lint.sh`
- Create final report with metrics
- Summary: `/tmp/gooseconfigoverhaul/phase6/final_report.md`
```

### Success Metrics for Phase 6
- [ ] 100+ total keys using unified API (46%+ coverage)
- [ ] All experiments and extensions migrated
- [ ] Remaining provider secrets consolidated
- [ ] Test configurations cleaned up
- [ ] Zero compilation errors
- [ ] All existing tests passing

### Common Pitfalls to Avoid
1. **Don't migrate system env vars** (HOME, PATH, USER) - keep as-is
2. **Skip test-only configs** unless they're widely used
3. **Avoid breaking changes** - always maintain aliases
4. **Don't over-parallelize** - 3-4 subagents maximum
5. **Check for duplicates** - many providers share similar configs

### Migration Checklist for Subagents
```rust
// For each configuration migration:
1. Add to registry with proper type and validator
2. Add env_aliases for backward compatibility
3. Replace usage sites with unified::get_or()
4. Test compilation with cargo check
5. Document in summary file
```

### Tips from Successful Phases
1. **Discovery tool limitations**: It undercounts unified API usage - verify manually
2. **Duplicate configs**: Check if similar keys exist before adding new ones
3. **Provider patterns**: Most providers follow similar configuration patterns
4. **Test incrementally**: Compile after each file change
5. **Summary files are crucial**: They help track progress across subagents

### Expected Outcomes
- Reach 100+ unified configuration keys
- Achieve 46-50% total coverage
- Establish patterns for Phase 7
- Maintain full backward compatibility
- Improve configuration consistency

Begin with the discovery analyst to establish your baseline, then proceed with parallel migrations. Focus on quality over quantity - well-migrated configurations are better than many partial migrations.

Begin by running the discovery analyst to establish your baseline, then proceed with migrations in priority order. Good luck!
