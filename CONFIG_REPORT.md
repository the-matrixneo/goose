# Goose Configuration Report

This report documents all configuration options in the Goose codebase, organized by source type (config file, environment variables, and CLI flags).

## Configuration System Overview

Goose uses a three-tiered configuration system with the following precedence (highest to lowest):
1. **Environment Variables** - Direct environment variable lookups (exact key match, converted to uppercase)
2. **Config File** - YAML-based configuration file (`~/.config/goose/config.yaml` by default)
3. **Secrets Storage** - System keyring or secrets file for sensitive values

The main configuration interface is provided by `Config::global()` in `crates/goose/src/config/base.rs`.

## Configuration Sources

### 1. Config File Parameters (via `get_param`/`set_param`)

These are stored in the config YAML file and can be overridden by environment variables:

#### Core Provider/Model Configuration
- **`GOOSE_PROVIDER`** - AI provider name (e.g., "openai", "anthropic", "ollama")
  - Used in: `session/builder.rs`, `commands/configure.rs`, `scheduler.rs`, `routes/agent.rs`
- **`GOOSE_MODEL`** - Model name (e.g., "gpt-4o", "claude-3.5-sonnet")  
  - Used in: `session/builder.rs`, `commands/configure.rs`, `scheduler.rs`, `routes/agent.rs`

#### Model Behavior Configuration
- **`GOOSE_CONTEXT_LIMIT`** - Context window size limit
  - Used in: `model.rs`
- **`GOOSE_TEMPERATURE`** - Model temperature setting
  - Used in: `model.rs`
- **`GOOSE_TOOLSHIM`** - Enable/disable toolshim functionality
  - Used in: `model.rs`
- **`GOOSE_TOOLSHIM_OLLAMA_MODEL`** - Specific model for toolshim with Ollama
  - Used in: `model.rs`, `commands/configure.rs`

#### Agent Behavior Configuration
- **`GOOSE_MODE`** - Agent operation mode ("auto", "approve", "smart_approve", "chat")
  - Used in: `session/mod.rs`, `commands/configure.rs`, `agents/agent.rs`, `routes/agent.rs`
- **`GOOSE_MAX_TURNS`** - Maximum consecutive agent turns without user input
  - Used in: `agents/agent.rs`, `commands/configure.rs`
- **`GOOSE_CONTEXT_STRATEGY`** - Context management strategy
  - Used in: `session/mod.rs`

#### Tool and Extension Configuration
- **`GOOSE_ROUTER_TOOL_SELECTION_STRATEGY`** - Tool selection strategy ("vector", "default")
  - Used in: `commands/configure.rs`
- **`GOOSE_SYSTEM_PROMPT_FILE_PATH`** - Path to custom system prompt file
  - Used in: `session/builder.rs`

#### UI/CLI Configuration
- **`GOOSE_CLI_THEME`** - CLI theme setting
  - Used in: `session/output.rs`
- **`GOOSE_CLI_MIN_PRIORITY`** - Minimum priority for tool output display
  - Used in: `session/output.rs`, `session/mod.rs`, `commands/configure.rs`
- **`GOOSE_CLI_TOOL_PARAMS_TRUNCATION_MAX_LENGTH`** - Max length for tool parameter display
  - Used in: `session/output.rs`
- **`GOOSE_CLI_SHOW_COST`** - Show token cost information
  - Used in: `session/mod.rs`

#### Scheduler Configuration
- **`GOOSE_SCHEDULER_TYPE`** - Scheduler type ("legacy", "temporal")
  - Used in: `scheduler_factory.rs`, `commands/configure.rs`, `commands/schedule.rs`

#### Recipe Configuration
- **`GOOSE_RECIPE_GITHUB_REPO`** - GitHub repository for recipes
  - Used in: `commands/configure.rs`, `recipes/search_recipe.rs`

#### Retry Configuration
- **`GOOSE_RECIPE_RETRY_TIMEOUT_SECONDS`** - Recipe retry timeout
  - Used in: `agents/retry.rs`
- **`GOOSE_RECIPE_ON_FAILURE_TIMEOUT_SECONDS`** - Recipe failure timeout
  - Used in: `agents/retry.rs`

#### Provider-Specific Configuration
- **`OPENAI_HOST`** - OpenAI API host
  - Used in: `providers/openai.rs`
- **`OPENAI_BASE_PATH`** - OpenAI API base path
  - Used in: `providers/openai.rs`
- **`OPENAI_ORGANIZATION`** - OpenAI organization ID
  - Used in: `providers/openai.rs`
- **`OPENAI_PROJECT`** - OpenAI project ID
  - Used in: `providers/openai.rs`
- **`OPENAI_CUSTOM_HEADERS`** - Custom headers for OpenAI requests
  - Used in: `providers/openai.rs`
- **`OPENAI_TIMEOUT`** - Request timeout for OpenAI
  - Used in: `providers/openai.rs`

- **`LITELLM_HOST`** - LiteLLM API host
  - Used in: `providers/litellm.rs`
- **`LITELLM_BASE_PATH`** - LiteLLM API base path
  - Used in: `providers/litellm.rs`
- **`LITELLM_CUSTOM_HEADERS`** - Custom headers for LiteLLM requests
  - Used in: `providers/litellm.rs`
- **`LITELLM_TIMEOUT`** - Request timeout for LiteLLM
  - Used in: `providers/litellm.rs`

- **`VENICE_HOST`** - Venice API host
  - Used in: `providers/venice.rs`
- **`VENICE_BASE_PATH`** - Venice API base path
  - Used in: `providers/venice.rs`
- **`VENICE_MODELS_PATH`** - Venice models endpoint path
  - Used in: `providers/venice.rs`

#### Embedding Configuration
- **`GOOSE_EMBEDDING_MODEL`** - Embedding model name
  - Used in: `providers/openai.rs`, `providers/litellm.rs`, `agents/router_tool_selector.rs`
- **`GOOSE_EMBEDDING_MODEL_PROVIDER`** - Embedding model provider
  - Used in: `agents/router_tool_selector.rs`

#### Context Management
- **`GOOSE_AUTO_COMPACT_THRESHOLD`** - Threshold for automatic context compaction
  - Used in: `context_mgmt/auto_compact.rs`

#### Planner Configuration
- **`GOOSE_PLANNER_PROVIDER`** - Provider for planning functionality
  - Used in: `session/mod.rs`
- **`GOOSE_PLANNER_MODEL`** - Model for planning functionality
  - Used in: `session/mod.rs`
- **`GOOSE_PLANNER_CONTEXT_LIMIT`** - Context limit for planner
  - Used in: `session/mod.rs`

#### Lead Worker Configuration
- **`GOOSE_LEAD_MODEL`** - Lead worker model
  - Used in: `providers/factory.rs`
- **`GOOSE_LEAD_PROVIDER`** - Lead worker provider
  - Used in: `providers/factory.rs`
- **`GOOSE_LEAD_TURNS`** - Lead worker turns
  - Used in: `providers/factory.rs`
- **`GOOSE_LEAD_FAILURE_THRESHOLD`** - Lead worker failure threshold
  - Used in: `providers/factory.rs`
- **`GOOSE_LEAD_FALLBACK_TURNS`** - Lead worker fallback turns
  - Used in: `providers/factory.rs`
- **`GOOSE_WORKER_CONTEXT_LIMIT`** - Worker context limit
  - Used in: `providers/factory.rs`

#### Experiment Configuration
- **`experiments`** - Experiment toggle settings (JSON object)
  - Used in: `config/experiments.rs`

#### Random/Testing Configuration
- **`RANDOM_THINKING_MESSAGES`** - Enable random thinking messages
  - Used in: `session/output.rs`
- **`EDIT_MODE`** - Editor mode setting
  - Used in: `session/builder.rs`

### 2. Environment Variables (Direct Access)

These environment variables are accessed directly via `std::env::var()`:

#### System Environment
- **`HOME`** - User home directory
  - Used in: `providers/claude_code.rs`, `providers/gemini_cli.rs`, `session/output.rs`
- **`USER`** / **`USERNAME`** - Current user name
  - Used in: `agents/agent.rs`
- **`PATH`** - System PATH variable
  - Used in: `providers/claude_code.rs`, `providers/gemini_cli.rs`
- **`USERPROFILE`** - Windows user profile path
  - Used in: `developer/shell.rs`
- **`APPDATA`** - Windows app data path
  - Used in: `developer/shell.rs`
- **`TEMP`** / **`TMPDIR`** - Temporary directory
  - Used in: `computercontroller/platform/windows.rs`

#### Display/Graphics Environment
- **`WAYLAND_DISPLAY`** - Wayland display identifier
  - Used in: `computercontroller/platform/linux.rs`
- **`DISPLAY`** - X11 display identifier
  - Used in: `computercontroller/platform/linux.rs`

#### Development/Testing Environment
- **`GOOSE_TEST_PROVIDER`** - Provider for testing
  - Used in: `scenario_tests/scenario_runner.rs`
- **`GITHUB_ACTIONS`** - GitHub Actions environment indicator
  - Used in: `scenario_tests/scenario_runner.rs`
- **`CARGO_MANIFEST_DIR`** - Cargo manifest directory
  - Used in: `routes/generate_schema.rs`

#### Cache and Storage
- **`GOOSE_CACHE_DIR`** - Cache directory override
  - Used in: `providers/pricing.rs`, various test files
- **`GOOSE_WORKING_DIR`** - Working directory for memory extension
  - Used in: `memory/mod.rs`
- **`GOOSE_VECTOR_DB_PATH`** - Vector database path
  - Used in: `agents/tool_vectordb.rs`

#### Security and Permissions
- **`GOOSE_DISABLE_KEYRING`** - Disable keyring usage, fall back to file storage
  - Used in: `config/base.rs`
- **`GOOSE_ALLOWLIST`** - Extension allowlist URL
  - Used in: `routes/extension.rs`
- **`GOOSE_ALLOWLIST_BYPASS`** - Bypass allowlist checks
  - Used in: `routes/extension.rs`

#### Debug and Development
- **`GOOSE_CLAUDE_CODE_DEBUG`** - Enable Claude Code debug output
  - Used in: `providers/claude_code.rs`
- **`GOOSE_GEMINI_CLI_DEBUG`** - Enable Gemini CLI debug output
  - Used in: `providers/gemini_cli.rs`
- **`GOOSE_CLI_SHOW_THINKING`** - Show thinking process in CLI
  - Used in: `session/output.rs`
- **`NO_COLOR`** - Disable colored output
  - Used in: `session/output.rs`

#### Tracing and Observability
- **`OTEL_EXPORTER_OTLP_ENDPOINT`** - OpenTelemetry endpoint
  - Used in: `main.rs`, `tracing/otlp_layer.rs`
- **`OTEL_EXPORTER_OTLP_TIMEOUT`** - OpenTelemetry timeout
  - Used in: `tracing/otlp_layer.rs`
- **`LANGFUSE_PUBLIC_KEY`** - Langfuse public key
  - Used in: `tracing/langfuse_layer.rs`, `logging.rs`
- **`LANGFUSE_SECRET_KEY`** - Langfuse secret key
  - Used in: `tracing/langfuse_layer.rs`, `logging.rs`
- **`LANGFUSE_URL`** - Langfuse URL
  - Used in: `tracing/langfuse_layer.rs`, `logging.rs`
- **`LANGFUSE_INIT_PROJECT_PUBLIC_KEY`** - Langfuse init project public key
  - Used in: `tracing/langfuse_layer.rs`, `logging.rs`
- **`LANGFUSE_INIT_PROJECT_SECRET_KEY`** - Langfuse init project secret key
  - Used in: `tracing/langfuse_layer.rs`, `logging.rs`

#### Temporal Scheduler
- **`PORT`** - Port for Temporal service
  - Used in: `temporal_scheduler.rs`
- **`GOOSE_TEMPORAL_BIN`** - Path to Temporal binary
  - Used in: `temporal_scheduler.rs`

#### Recipe System
- **`GOOSE_RECIPE_PATH`** - Path for recipe search
  - Used in: `recipes/search_recipe.rs`

#### Sub-agent Configuration
- **`GOOSE_SUBAGENT_MAX_TURNS`** - Maximum turns for sub-agents
  - Used in: `agents/subagent_task_config.rs`

#### Provider-Specific Secrets and Configuration
- **`CLAUDE_THINKING_ENABLED`** - Enable Claude thinking mode
  - Used in: `providers/anthropic.rs`, `providers/formats/anthropic.rs`, `providers/formats/databricks.rs`
- **`CLAUDE_THINKING_BUDGET`** - Claude thinking token budget
  - Used in: `providers/formats/anthropic.rs`, `providers/formats/databricks.rs`

#### Google Drive Extension
- **`GOOGLE_DRIVE_OAUTH_PATH`** - OAuth credentials path
  - Used in: `google_drive/mod.rs`
- **`GOOGLE_DRIVE_CREDENTIALS_PATH`** - Service account credentials path
  - Used in: `google_drive/mod.rs`
- **`GOOGLE_DRIVE_OAUTH_CONFIG`** - OAuth configuration
  - Used in: `google_drive/mod.rs`
- **`KEYCHAIN_DISK_FALLBACK`** - Keychain disk fallback setting
  - Used in: `google_drive/mod.rs`

#### Editor Models
- **`GOOSE_EDITOR_API_KEY`** - API key for editor models
  - Used in: `developer/editor_models/mod.rs`
- **`GOOSE_EDITOR_HOST`** - Host for editor models
  - Used in: `developer/editor_models/mod.rs`
- **`GOOSE_EDITOR_MODEL`** - Model for editor functionality
  - Used in: `developer/editor_models/mod.rs`

#### Server Configuration
- **`GOOSE_SERVER__SECRET_KEY`** - Server secret key
  - Used in: `commands/agent.rs`

#### Context Files
- **`CONTEXT_FILE_NAMES`** - JSON array of context file names
  - Used in: `developer/mod.rs`

### 3. CLI Flags (via clap)

CLI flags are defined in `crates/goose-cli/src/cli.rs` using the clap library:

#### Session Management
- **`--name` / `-n`** - Session name
- **`--path` / `-p`** - Session path
- **`--resume` / `-r`** - Resume previous session
- **`--history`** - Show message history when resuming
- **`--no-session`** - Run without storing session file

#### Execution Control
- **`--interactive` / `-s`** - Continue in interactive mode
- **`--debug`** - Enable debug output mode
- **`--quiet` / `-q`** - Quiet mode, suppress non-response output
- **`--max-tool-repetitions`** - Maximum consecutive identical tool calls
- **`--max-turns`** - Maximum turns without user input

#### Input Sources
- **`--instructions` / `-i`** - Path to instruction file (or "-" for stdin)
- **`--text` / `-t`** - Input text directly
- **`--recipe`** - Recipe name or path
- **`--params`** - Key-value parameters for recipes
- **`--system`** - Additional system prompt

#### Extensions
- **`--with-extension`** - Add stdio extensions (can be repeated)
- **`--with-remote-extension`** - Add remote extensions (can be repeated)
- **`--with-streamable-http-extension`** - Add streamable HTTP extensions (can be repeated)
- **`--with-builtin`** - Add builtin extensions by name (comma-separated)

#### Provider Override
- **`--provider`** - Override GOOSE_PROVIDER for this run
- **`--model`** - Override GOOSE_MODEL for this run

#### Recipe Management
- **`--explain`** - Show recipe details instead of running
- **`--render-recipe`** - Print rendered recipe instead of running
- **`--sub-recipe`** - Additional sub-recipe paths (can be repeated)

#### Session Commands
- **`session list --verbose` / `-v`** - Verbose session listing
- **`session list --format`** - Output format (text, json)
- **`session list --ascending`** - Sort sessions by date ascending
- **`session remove --id`** - Session ID to remove
- **`session remove --regex`** - Regex pattern for session removal
- **`session export --output` / `-o`** - Export output file path

#### Scheduler Commands
- **`schedule add --id`** - Unique job ID
- **`schedule add --cron`** - Cron expression
- **`schedule add --recipe-source`** - Recipe source path or base64
- **`schedule remove --id`** - Job ID to remove
- **`schedule sessions --id`** - Schedule ID for session listing
- **`schedule sessions --limit`** - Maximum sessions to return
- **`schedule run-now --id`** - Schedule ID to run immediately

#### Benchmark Commands
- **`bench init-config --name` / `-n`** - Config filename
- **`bench run --config` / `-c`** - Config file path
- **`bench selectors --config` / `-c`** - Config file for selectors
- **`bench eval-model --config` / `-c`** - Serialized model config
- **`bench exec-eval --config` / `-c`** - Serialized eval config
- **`bench generate-leaderboard --benchmark-dir` / `-b`** - Benchmark directory

#### Recipe Commands
- **`recipe validate <recipe_name>`** - Recipe to validate
- **`recipe deeplink <recipe_name>`** - Recipe for deeplink generation
- **`recipe list --format`** - Output format
- **`recipe list --verbose` / `-v`** - Show verbose information

#### Update Commands
- **`update --canary` / `-c`** - Update to canary version
- **`update --reconfigure` / `-r`** - Force reconfiguration during update

#### Info Commands
- **`info --verbose` / `-v`** - Show verbose configuration information

#### Web Server Commands
- **`web --port` / `-p`** - Server port (default: 3000)
- **`web --host`** - Server host (default: 127.0.0.1)
- **`web --open`** - Open browser automatically

#### Internal/Hidden Flags
- **`--scheduled-job-id`** - Internal flag for scheduled job tracking

## Configuration File Location

The default configuration file location varies by platform:
- **macOS/Linux**: `~/.config/goose/config.yaml`
- **Windows**: `~\AppData\Roaming\Block\goose\config\config.yaml`

## Secrets Storage

Secrets are stored in:
1. **System keyring** (default) - macOS Keychain, Windows Credential Manager, Linux secret service
2. **Secrets file** (fallback) - `~/.config/goose/secrets.yaml` when `GOOSE_DISABLE_KEYRING` is set

## Configuration Precedence

For any given configuration key, the system checks in this order:
1. Environment variable (exact key name, converted to uppercase)
2. Config file parameter (via `get_param`)
3. Default value (if specified in code)

This allows for flexible configuration management where environment variables can override file-based settings, and CLI flags can override both.
