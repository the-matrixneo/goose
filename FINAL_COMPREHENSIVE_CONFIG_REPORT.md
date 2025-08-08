# Comprehensive Goose Configuration Analysis

## Summary

- **Total Configuration Usages:** 1045
- **Unique Configuration Keys:** 346
- **Files with Configuration:** 101
- **Test-related Usages:** 5

### By Category

- **Environment Variables:** 63 unique keys
- **Config File Parameters:** 269 unique keys
- **Secret Storage:** 21 unique keys
- **CLI Flags:** 11 unique keys

## Config File Parameters

### `$ref`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-server/src/openapi.rs:72`

### `1=1`

**Method(s):** config_delete

**Usage Locations (1):**

- `crates/goose/src/agents/tool_vectordb.rs:165`

### `ANTHROPIC_HOST`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/anthropic.rs:54`

### `AZURE_OPENAI_API_VERSION`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/azure.rs:79`

### `AZURE_OPENAI_DEPLOYMENT_NAME`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose/src/providers/azure.rs:77`
- `crates/goose/src/providers/azure.rs:77`

### `AZURE_OPENAI_ENDPOINT`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose/src/providers/azure.rs:76`
- `crates/goose/src/providers/azure.rs:76`

### `CLAUDE_CODE_COMMAND`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/claude_code.rs:36`

### `DATABRICKS_BACKOFF_MULTIPLIER`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/databricks.rs:163`

### `DATABRICKS_HOST`

**Method(s):** secret_get, config_get

**Usage Locations (4):**

- `crates/goose/src/providers/databricks.rs:113`
- `crates/goose/src/providers/databricks.rs:113`
- `crates/goose/src/providers/databricks.rs:115`
- `crates/goose/src/providers/databricks.rs:115`

### `DATABRICKS_INITIAL_RETRY_INTERVAL_MS`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/databricks.rs:157`

### `DATABRICKS_MAX_RETRIES`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/databricks.rs:151`

### `DATABRICKS_MAX_RETRY_INTERVAL_MS`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/databricks.rs:169`

### `GCP_BACKOFF_MULTIPLIER`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/gcpvertexai.rs:148`

### `GCP_INITIAL_RETRY_INTERVAL_MS`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/gcpvertexai.rs:142`

### `GCP_LOCATION`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/gcpvertexai.rs:174`

### `GCP_MAX_RETRIES`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/gcpvertexai.rs:136`

### `GCP_MAX_RETRY_INTERVAL_MS`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/gcpvertexai.rs:154`

### `GCP_PROJECT_ID`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose/src/providers/gcpvertexai.rs:108`
- `crates/goose/src/providers/gcpvertexai.rs:108`

### `GEMINI_CLI_COMMAND`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/gemini_cli.rs:35`

### `GOOGLE_HOST`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/google.rs:61`

### `GOOSE_AUTO_COMPACT_THRESHOLD`

**Method(s):** config_set

**Usage Locations (4):**

- `crates/goose/src/context_mgmt/auto_compact.rs:546`
- `crates/goose/src/context_mgmt/auto_compact.rs:546`
- `crates/goose/src/context_mgmt/auto_compact.rs:576`
- `crates/goose/src/context_mgmt/auto_compact.rs:576`

### `GOOSE_MAX_TURNS`

**Method(s):** config_set, config_get

**Usage Locations (7):**

- `crates/goose-cli/src/commands/configure.rs:1522`
- `crates/goose-cli/src/commands/configure.rs:1522`
- `crates/goose-cli/src/commands/configure.rs:1541`
- `crates/goose-cli/src/commands/configure.rs:1541`
- `crates/goose-cli/src/commands/configure.rs:1541`
- `crates/goose/src/agents/agent.rs:884`
- `crates/goose/src/agents/agent.rs:884`

### `GOOSE_PROVIDER`

**Method(s):** config_set, env_remove, env_set, config_get

**Usage Locations (18):**

- `crates/goose-cli/src/commands/configure.rs:297`
- `crates/goose-cli/src/commands/configure.rs:297`
- `crates/goose-cli/src/commands/configure.rs:502`
- `crates/goose-cli/src/commands/configure.rs:502`
- `crates/goose-cli/src/commands/configure.rs:502`
- `crates/goose-cli/src/commands/configure.rs:1310`
- `crates/goose-cli/src/commands/web.rs:87`
- `crates/goose-cli/src/commands/web.rs:87`
- `crates/goose-cli/src/session/builder.rs:187`
- `crates/goose-cli/src/session/builder.rs:187`
- ... and 8 more locations

### `GOOSE_SYSTEM_PROMPT_FILE_PATH`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose-cli/src/session/builder.rs:563`
- `crates/goose-cli/src/session/builder.rs:563`

### `GROQ_HOST`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/groq.rs:40`

### `LITELLM_BASE_PATH`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/litellm.rs:40`

### `LITELLM_HOST`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/litellm.rs:37`

### `LITELLM_TIMEOUT`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose/src/providers/litellm.rs:47`
- `crates/goose/src/providers/litellm.rs:47`

### `OLLAMA_HOST`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose/src/providers/ollama.rs:41`
- `crates/goose/src/providers/toolshim.rs:88`

### `OLLAMA_TIMEOUT`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose/src/providers/ollama.rs:45`
- `crates/goose/src/providers/ollama.rs:45`

### `OPENAI_BASE_PATH`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/openai.rs:66`

### `OPENAI_HOST`

**Method(s):** config_get

**Usage Locations (3):**

- `crates/goose-server/src/routes/audio.rs:104`
- `crates/goose-server/src/routes/audio.rs:104`
- `crates/goose/src/providers/openai.rs:63`

### `OPENAI_ORGANIZATION`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose/src/providers/openai.rs:68`
- `crates/goose/src/providers/openai.rs:68`

### `OPENAI_PROJECT`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose/src/providers/openai.rs:69`
- `crates/goose/src/providers/openai.rs:69`

### `OPENAI_TIMEOUT`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose/src/providers/openai.rs:75`
- `crates/goose/src/providers/openai.rs:75`

### `OPENROUTER_HOST`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/openrouter.rs:48`

### `RANDOM_THINKING_MESSAGES`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-cli/src/session/output.rs:101`

### `SAGEMAKER_ENDPOINT_NAME`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose/src/providers/sagemaker_tgi.rs:40`
- `crates/goose/src/providers/sagemaker_tgi.rs:40`

### `SNOWFLAKE_HOST`

**Method(s):** secret_get, config_get

**Usage Locations (4):**

- `crates/goose/src/providers/snowflake.rs:48`
- `crates/goose/src/providers/snowflake.rs:48`
- `crates/goose/src/providers/snowflake.rs:50`
- `crates/goose/src/providers/snowflake.rs:50`

### `SNOWFLAKE_TOKEN`

**Method(s):** secret_get, config_get

**Usage Locations (4):**

- `crates/goose/src/providers/snowflake.rs:69`
- `crates/goose/src/providers/snowflake.rs:69`
- `crates/goose/src/providers/snowflake.rs:72`
- `crates/goose/src/providers/snowflake.rs:72`

### `VENICE_BASE_PATH`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/venice.rs:93`

### `VENICE_HOST`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/venice.rs:90`

### `VENICE_MODELS_PATH`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/venice.rs:96`

### `X-Secret-Key`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-server/src/routes/utils.rs:29`

### `XAI_HOST`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/xai.rs:53`

### `_errors`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/venice.rs:140`

### `access_token`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/oauth.rs:180`

### `action`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose/src/agents/agent.rs:396`
- `crates/goose/src/agents/schedule_tool.rs:34`

### `active`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-cli/src/session/export.rs:865`

### `activities`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/agents/agent.rs:1306`

### `additionalProperties`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-server/src/openapi.rs:155`

### `alignment`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/computercontroller/docx_tool.rs:47`

### `allOf`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-server/src/openapi.rs:87`

### `allowSharedDrives`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose-mcp/src/google_drive/mod.rs:1991`
- `crates/goose-mcp/src/google_drive/mod.rs:2156`

### `another_key`

**Method(s):** config_set

**Usage Locations (3):**

- `crates/goose/src/config/base.rs:1149`
- `crates/goose/src/config/base.rs:1149`
- `crates/goose/src/config/base.rs:1149`

### `anthropic`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/pricing.rs:426`

### `anyOf`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-server/src/openapi.rs:97`

### `arg1`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose-cli/src/session/input.rs:393`
- `crates/goose-cli/src/session/input.rs:445`

### `arg2`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose-cli/src/session/input.rs:394`
- `crates/goose-cli/src/session/input.rs:449`

### `args`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/formats/google.rs:263`

### `arguments`

**Method(s):** config_get

**Usage Locations (3):**

- `crates/goose/src/providers/toolshim.rs:220`
- `crates/mcp-server/src/router.rs:182`
- `crates/mcp-server/src/router.rs:277`

### `authorization_endpoint`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/oauth.rs:114`

### `body`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose-mcp/src/google_drive/mod.rs:1987`
- `crates/goose-mcp/src/google_drive/mod.rs:2161`

### `bold`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/computercontroller/docx_tool.rs:38`

### `cache_control`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose/src/providers/formats/anthropic.rs:897`
- `crates/goose/src/providers/formats/anthropic.rs:910`

### `cache_creation_input_tokens`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose/src/providers/formats/anthropic.rs:277`
- `crates/goose/src/providers/formats/anthropic.rs:314`

### `cache_read_input_tokens`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose/src/providers/formats/anthropic.rs:282`
- `crates/goose/src/providers/formats/anthropic.rs:319`

### `candidates`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/formats/google.rs:227`

### `candidatesTokenCount`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/formats/google.rs:284`

### `case_sensitive`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/computercontroller/mod.rs:894`

### `category`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-bench/src/eval_suites/core/memory/save_fact.rs:56`

### `cell`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:1654`

### `choiceValue`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:2254`

### `choices`

**Method(s):** config_get

**Usage Locations (5):**

- `crates/goose-mcp/src/developer/editor_models/morphllm_editor.rs:122`
- `crates/goose-mcp/src/developer/editor_models/openai_compatible_editor.rs:88`
- `crates/goose-mcp/src/developer/editor_models/relace_editor.rs:88`
- `crates/goose/src/providers/formats/snowflake.rs:151`
- `crates/goose/src/providers/snowflake.rs:144`

### `claude-3-5-sonnet-latest`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/base.rs:525`

### `code`

**Method(s):** config_get

**Usage Locations (5):**

- `crates/goose/src/providers/oauth.rs:306`
- `crates/goose/src/providers/openrouter.rs:91`
- `crates/goose/src/providers/snowflake.rs:110`
- `crates/goose/src/providers/snowflake.rs:112`
- `crates/goose/src/providers/utils.rs:188`

### `code_edit`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-cli/src/session/export.rs:160`

### `col`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/computercontroller/mod.rs:949`

### `color`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/computercontroller/docx_tool.rs:45`

### `command`

**Method(s):** config_get

**Usage Locations (12):**

- `crates/goose-bench/src/eval_suites/core/developer/create_file.rs:56`
- `crates/goose-bench/src/eval_suites/core/developer/create_file.rs:87`
- `crates/goose-bench/src/eval_suites/core/developer/list_files.rs:49`
- `crates/goose-bench/src/eval_suites/vibes/flappy_bird.rs:65`
- `crates/goose-bench/src/eval_suites/vibes/goose_wiki.rs:75`
- `crates/goose-bench/src/eval_suites/vibes/squirrel_census.rs:86`
- `crates/goose-bench/src/eval_suites/vibes/squirrel_census.rs:116`
- `crates/goose-cli/src/session/export.rs:132`
- `crates/goose-cli/src/session/output.rs:416`
- `crates/goose-mcp/src/computercontroller/mod.rs:1012`
- ... and 2 more locations

### `command_parameters`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/agents/subagent_execution_tool/task_types.rs:34`

### `commentId`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:2520`

### `completion_tokens`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/formats/openai.rs:342`

### `complex_key`

**Method(s):** config_set, config_get

**Usage Locations (5):**

- `crates/goose/src/config/base.rs:833`
- `crates/goose/src/config/base.rs:833`
- `crates/goose/src/config/base.rs:833`
- `crates/goose/src/config/base.rs:841`
- `crates/goose/src/config/base.rs:841`

### `config`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose/src/config/base.rs:1429`
- `crates/goose/src/config/base.rs:1429`

### `content`

**Method(s):** config_get

**Usage Locations (16):**

- `crates/goose-cli/src/recipes/github_recipe.rs:323`
- `crates/goose-mcp/src/computercontroller/mod.rs:990`
- `crates/goose-mcp/src/developer/editor_models/morphllm_editor.rs:125`
- `crates/goose-mcp/src/developer/editor_models/openai_compatible_editor.rs:91`
- `crates/goose-mcp/src/developer/editor_models/relace_editor.rs:91`
- `crates/goose-mcp/src/google_drive/mod.rs:2482`
- `crates/goose/src/providers/claude_code.rs:213`
- `crates/goose/src/providers/formats/databricks.rs:291`
- `crates/goose/src/providers/formats/databricks.rs:330`
- `crates/goose/src/providers/formats/google.rs:238`
- ... and 6 more locations

### `content_block`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/formats/anthropic.rs:522`

### `content_list`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose/src/providers/formats/snowflake.rs:204`
- `crates/goose/src/providers/snowflake.rs:169`

### `context`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/venice.rs:152`

### `corpora`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:1014`

### `cron_expression`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/agents/schedule_tool.rs:92`

### `currentFolderId`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:2111`

### `cursor`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:1848`

### `data`

**Method(s):** config_get

**Usage Locations (8):**

- `crates/goose-bench/src/eval_suites/core/memory/save_fact.rs:57`
- `crates/goose-mcp/src/memory/mod.rs:592`
- `crates/goose/src/providers/formats/databricks.rs:317`
- `crates/goose/src/providers/formats/snowflake.rs:264`
- `crates/goose/src/providers/githubcopilot.rs:455`
- `crates/goose/src/providers/groq.rs:120`
- `crates/goose/src/providers/openai.rs:187`
- `crates/goose/src/providers/openrouter.rs:297`

### `dateValue`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:2229`

### `delta`

**Method(s):** config_get

**Usage Locations (3):**

- `crates/goose/src/providers/formats/anthropic.rs:535`
- `crates/goose/src/providers/formats/snowflake.rs:153`
- `crates/goose/src/providers/snowflake.rs:158`

### `details`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/venice.rs:137`

### `display`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose-bench/src/eval_suites/core/developer_image/image.rs:53`
- `crates/goose-mcp/src/developer/mod.rs:1459`

### `documentId`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:2571`

### `driveId`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:1009`

### `driveType`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:993`

### `emailMessage`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:3096`

### `enabled`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose/src/config/base.rs:1419`
- `crates/goose/src/config/base.rs:1419`

### `endPosition`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:2894`

### `endpoints`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/databricks.rs:367`

### `error`

**Method(s):** config_get

**Usage Locations (7):**

- `crates/goose/src/providers/anthropic.rs:102`
- `crates/goose/src/providers/openai.rs:179`
- `crates/goose/src/providers/openrouter.rs:84`
- `crates/goose/src/providers/openrouter.rs:288`
- `crates/goose/src/providers/utils.rs:88`
- `crates/goose/src/providers/utils.rs:187`
- `crates/goose/src/providers/utils.rs:227`

### `exclusiveMaximum`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose-server/src/openapi.rs:258`
- `crates/goose-server/src/openapi.rs:289`

### `exclusiveMinimum`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose-server/src/openapi.rs:253`
- `crates/goose-server/src/openapi.rs:284`

### `execution_mode`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose/src/agents/schedule_tool.rs:100`
- `crates/goose/src/agents/subagent_execution_tool/subagent_execute_task_tool.rs:75`

### `experiments`

**Method(s):** config_set, config_get

**Usage Locations (6):**

- `crates/goose/src/config/experiments.rs:23`
- `crates/goose/src/config/experiments.rs:23`
- `crates/goose/src/config/experiments.rs:33`
- `crates/goose/src/config/experiments.rs:38`
- `crates/goose/src/config/experiments.rs:38`
- `crates/goose/src/config/experiments.rs:38`

### `expires_in`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/oauth.rs:194`

### `extension`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/agents/extension_manager.rs:672`

### `extension_name`

**Method(s):** config_get

**Usage Locations (4):**

- `crates/goose/src/agents/agent.rs:390`
- `crates/goose/src/agents/extension_manager.rs:551`
- `crates/goose/src/agents/router_tool_selector.rs:84`
- `crates/goose/src/agents/router_tool_selector.rs:265`

### `extensions`

**Method(s):** config_set, config_get

**Usage Locations (23):**

- `crates/goose/src/config/extensions.rs:36`
- `crates/goose/src/config/extensions.rs:36`
- `crates/goose/src/config/extensions.rs:53`
- `crates/goose/src/config/extensions.rs:53`
- `crates/goose/src/config/extensions.rs:53`
- `crates/goose/src/config/extensions.rs:72`
- `crates/goose/src/config/extensions.rs:72`
- `crates/goose/src/config/extensions.rs:89`
- `crates/goose/src/config/extensions.rs:95`
- `crates/goose/src/config/extensions.rs:95`
- ... and 13 more locations

### `fieldId`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose-mcp/src/google_drive/mod.rs:2205`
- `crates/goose-mcp/src/google_drive/mod.rs:2225`

### `fileId`

**Method(s):** config_get

**Usage Locations (6):**

- `crates/goose-mcp/src/google_drive/mod.rs:2105`
- `crates/goose-mcp/src/google_drive/mod.rs:2149`
- `crates/goose-mcp/src/google_drive/mod.rs:2407`
- `crates/goose-mcp/src/google_drive/mod.rs:2472`
- `crates/goose-mcp/src/google_drive/mod.rs:3017`
- `crates/goose-mcp/src/google_drive/mod.rs:3068`

### `file_text`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-bench/src/eval_suites/core/developer/create_file.rs:58`

### `format`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-server/src/openapi.rs:234`

### `functionCall`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/formats/google.rs:246`

### `generated_text`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/sagemaker_tgi.rs:197`

### `gpt-4o`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/base.rs:521`

### `height`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/computercontroller/docx_tool.rs:229`

### `host`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/config/base.rs:1329`

### `https://openrouter.ai/api/v1/models`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/pricing.rs:279`

### `id`

**Method(s):** config_get

**Usage Locations (7):**

- `crates/goose/src/providers/anthropic.rs:209`
- `crates/goose/src/providers/formats/anthropic.rs:501`
- `crates/goose/src/providers/formats/anthropic.rs:524`
- `crates/goose/src/providers/githubcopilot.rs:465`
- `crates/goose/src/providers/groq.rs:128`
- `crates/goose/src/providers/openai.rs:192`
- `crates/goose/src/providers/openrouter.rs:305`

### `image_path`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/computercontroller/docx_tool.rs:214`

### `includeImages`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:1419`

### `includeLabels`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:1049`

### `input`

**Method(s):** config_get

**Usage Locations (6):**

- `crates/goose/src/providers/databricks.rs:207`
- `crates/goose/src/providers/formats/snowflake.rs:167`
- `crates/goose/src/providers/formats/snowflake.rs:244`
- `crates/goose/src/providers/snowflake.rs:192`
- `crates/goose/src/providers/snowflake.rs:210`
- `crates/goose/src/tracing/observation_layer.rs:202`

### `input_tokens`

**Method(s):** config_get

**Usage Locations (5):**

- `crates/goose/src/providers/claude_code.rs:234`
- `crates/goose/src/providers/claude_code.rs:258`
- `crates/goose/src/providers/formats/anthropic.rs:272`
- `crates/goose/src/providers/formats/anthropic.rs:309`
- `crates/goose/src/providers/formats/snowflake.rs:283`

### `insert_line`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/developer/mod.rs:820`

### `instructions`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/agents/agent.rs:1299`

### `integerValue`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:2263`

### `is_global`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose-bench/src/eval_suites/core/memory/save_fact.rs:58`
- `crates/goose-mcp/src/memory/mod.rs:600`

### `issues`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/venice.rs:153`

### `italic`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/computercontroller/docx_tool.rs:39`

### `items`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-server/src/openapi.rs:181`

### `job_id`

**Method(s):** config_get

**Usage Locations (7):**

- `crates/goose/src/agents/schedule_tool.rs:175`
- `crates/goose/src/agents/schedule_tool.rs:198`
- `crates/goose/src/agents/schedule_tool.rs:221`
- `crates/goose/src/agents/schedule_tool.rs:244`
- `crates/goose/src/agents/schedule_tool.rs:267`
- `crates/goose/src/agents/schedule_tool.rs:290`
- `crates/goose/src/agents/schedule_tool.rs:320`

### `key`

**Method(s):** secret_set, secret_get, config_delete, config_set, config_get, secret_delete

**Usage Locations (19):**

- `crates/goose/src/config/base.rs:878`
- `crates/goose/src/config/base.rs:878`
- `crates/goose/src/config/base.rs:878`
- `crates/goose/src/config/base.rs:880`
- `crates/goose/src/config/base.rs:880`
- `crates/goose/src/config/base.rs:883`
- `crates/goose/src/config/base.rs:883`
- `crates/goose/src/config/base.rs:885`
- `crates/goose/src/config/base.rs:885`
- `crates/goose/src/config/base.rs:897`
- ... and 9 more locations

### `key1`

**Method(s):** config_set, secret_get, secret_set, secret_delete

**Usage Locations (19):**

- `crates/goose/src/config/base.rs:862`
- `crates/goose/src/config/base.rs:862`
- `crates/goose/src/config/base.rs:862`
- `crates/goose/src/config/base.rs:945`
- `crates/goose/src/config/base.rs:945`
- `crates/goose/src/config/base.rs:945`
- `crates/goose/src/config/base.rs:949`
- `crates/goose/src/config/base.rs:949`
- `crates/goose/src/config/base.rs:955`
- `crates/goose/src/config/base.rs:955`
- ... and 9 more locations

### `key2`

**Method(s):** config_set, secret_get, secret_set

**Usage Locations (13):**

- `crates/goose/src/config/base.rs:863`
- `crates/goose/src/config/base.rs:863`
- `crates/goose/src/config/base.rs:863`
- `crates/goose/src/config/base.rs:946`
- `crates/goose/src/config/base.rs:946`
- `crates/goose/src/config/base.rs:946`
- `crates/goose/src/config/base.rs:950`
- `crates/goose/src/config/base.rs:950`
- `crates/goose/src/config/base.rs:959`
- `crates/goose/src/config/base.rs:959`
- ... and 3 more locations

### `labelId`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:2190`

### `language`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/computercontroller/mod.rs:670`

### `level`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/computercontroller/docx_tool.rs:204`

### `limit`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/agents/schedule_tool.rs:325`

### `location`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-cli/src/scenario_tests/mock_client.rs:155` (test)

### `maxItems`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-server/src/openapi.rs:210`

### `maxLength`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-server/src/openapi.rs:226`

### `maximum`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose-server/src/openapi.rs:248`
- `crates/goose-server/src/openapi.rs:279`

### `message`

**Method(s):** config_get

**Usage Locations (16):**

- `crates/goose-bench/src/error_capture.rs:47`
- `crates/goose-cli/src/session/mod.rs:1104`
- `crates/goose-mcp/src/developer/editor_models/morphllm_editor.rs:124`
- `crates/goose-mcp/src/developer/editor_models/openai_compatible_editor.rs:90`
- `crates/goose-mcp/src/developer/editor_models/relace_editor.rs:90`
- `crates/goose/src/providers/anthropic.rs:103`
- `crates/goose/src/providers/claude_code.rs:211`
- `crates/goose/src/providers/formats/anthropic.rs:499`
- `crates/goose/src/providers/openai.rs:181`
- `crates/goose/src/providers/openrouter.rs:87`
- ... and 6 more locations

### `messages`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/databricks.rs:207`

### `metadata`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/tracing/observation_layer.rs:223`

### `mimeType`

**Method(s):** config_get

**Usage Locations (3):**

- `crates/goose-mcp/src/google_drive/mod.rs:1008`
- `crates/goose-mcp/src/google_drive/mod.rs:1979`
- `crates/goose-mcp/src/google_drive/mod.rs:2160`

### `minItems`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-server/src/openapi.rs:205`

### `minLength`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-server/src/openapi.rs:221`

### `minimum`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose-server/src/openapi.rs:243`
- `crates/goose-server/src/openapi.rs:274`

### `mode`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/computercontroller/docx_tool.rs:181`

### `model`

**Method(s):** config_get

**Usage Locations (6):**

- `crates/goose/src/providers/formats/anthropic.rs:509`
- `crates/goose/src/providers/formats/anthropic.rs:629`
- `crates/goose/src/providers/formats/anthropic.rs:648`
- `crates/goose/src/providers/githubcopilot.rs:141`
- `crates/goose/src/providers/utils.rs:172`
- `crates/goose/src/providers/utils.rs:267`

### `modelVersion`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/google.rs:126`

### `model_config`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/tracing/observation_layer.rs:210`

### `models`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose/src/providers/anthropic.rs:198`
- `crates/goose/src/providers/google.rs:139`

### `multipleOf`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose-server/src/openapi.rs:263`
- `crates/goose-server/src/openapi.rs:294`

### `name`

**Method(s):** config_get

**Usage Locations (16):**

- `crates/goose-cli/src/recipes/github_recipe.rs:249`
- `crates/goose-cli/src/recipes/github_recipe.rs:284`
- `crates/goose-mcp/src/google_drive/mod.rs:1007`
- `crates/goose-mcp/src/google_drive/mod.rs:1971`
- `crates/goose-mcp/src/tutorial/mod.rs:137`
- `crates/goose/src/agents/subagent_execution_tool/task_types.rs:46`
- `crates/goose/src/providers/databricks.rs:381`
- `crates/goose/src/providers/formats/anthropic.rs:526`
- `crates/goose/src/providers/formats/snowflake.rs:164`
- `crates/goose/src/providers/formats/snowflake.rs:239`
- ... and 6 more locations

### `name_contains`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:2947`

### `newFolderId`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:2116`

### `new_str`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose-mcp/src/developer/mod.rs:810`
- `crates/goose-mcp/src/developer/mod.rs:826`

### `nonexistent_key`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose/src/config/base.rs:853`
- `crates/goose/src/config/base.rs:853`

### `old_str`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/developer/mod.rs:804`

### `old_text`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/computercontroller/docx_tool.rs:191`

### `oneOf`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-server/src/openapi.rs:77`

### `operation`

**Method(s):** config_get

**Usage Locations (9):**

- `crates/goose-mcp/src/computercontroller/mod.rs:833`
- `crates/goose-mcp/src/computercontroller/mod.rs:983`
- `crates/goose-mcp/src/computercontroller/mod.rs:1003`
- `crates/goose-mcp/src/google_drive/mod.rs:1447`
- `crates/goose-mcp/src/google_drive/mod.rs:2195`
- `crates/goose-mcp/src/google_drive/mod.rs:2292`
- `crates/goose-mcp/src/google_drive/mod.rs:2477`
- `crates/goose-mcp/src/google_drive/mod.rs:2575`
- `crates/goose-mcp/src/google_drive/mod.rs:3073`

### `output`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose-cli/src/session/mod.rs:1146`
- `crates/goose/src/tracing/observation_layer.rs:206`

### `output_tokens`

**Method(s):** config_get

**Usage Locations (5):**

- `crates/goose/src/providers/claude_code.rs:238`
- `crates/goose/src/providers/claude_code.rs:264`
- `crates/goose/src/providers/formats/anthropic.rs:287`
- `crates/goose/src/providers/formats/anthropic.rs:324`
- `crates/goose/src/providers/formats/snowflake.rs:288`

### `pageSize`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:1030`

### `params`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/computercontroller/mod.rs:991`

### `parent`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:1010`

### `parentId`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:1985`

### `partial_json`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/formats/anthropic.rs:553`

### `partial_output`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose-cli/src/session/task_execution_display/mod.rs:24`
- `crates/goose/src/agents/subagent_execution_tool/lib/mod.rs:79`

### `parts`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/formats/google.rs:239`

### `path`

**Method(s):** config_get

**Usage Locations (16):**

- `crates/goose-bench/src/eval_suites/core/developer/create_file.rs:57`
- `crates/goose-bench/src/eval_suites/core/developer/create_file.rs:88`
- `crates/goose-bench/src/eval_suites/vibes/flappy_bird.rs:67`
- `crates/goose-bench/src/eval_suites/vibes/goose_wiki.rs:77`
- `crates/goose-bench/src/eval_suites/vibes/squirrel_census.rs:88`
- `crates/goose-cli/src/session/export.rs:157`
- `crates/goose-cli/src/session/output.rs:392`
- `crates/goose-mcp/src/computercontroller/mod.rs:828`
- `crates/goose-mcp/src/computercontroller/mod.rs:978`
- `crates/goose-mcp/src/computercontroller/mod.rs:998`
- ... and 6 more locations

### `pattern`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-server/src/openapi.rs:231`

### `permissionId`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:3076`

### `port`

**Method(s):** config_get

**Usage Locations (3):**

- `crates/goose/src/config/base.rs:1332`
- `crates/goose/src/config/base.rs:1414`
- `crates/goose/src/config/base.rs:1414`

### `position`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:2631`

### `promptTokenCount`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/formats/google.rs:280`

### `prompt_tokens`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/formats/openai.rs:337`

### `properties`

**Method(s):** config_get

**Usage Locations (4):**

- `crates/goose-server/src/openapi.rs:136`
- `crates/goose/src/agents/extension_manager.rs:96`
- `crates/goose/src/providers/formats/google.rs:139`
- `crates/goose/src/providers/formats/google.rs:694`

### `provider`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose/src/config/base.rs:1409`
- `crates/goose/src/config/base.rs:1409`

### `query`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose/src/agents/router_tool_selector.rs:77`
- `crates/goose/src/agents/router_tool_selector.rs:260`

### `quoted`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-cli/src/session/input.rs:464`

### `range`

**Method(s):** config_get

**Usage Locations (4):**

- `crates/goose-mcp/src/computercontroller/mod.rs:864`
- `crates/goose-mcp/src/google_drive/mod.rs:1537`
- `crates/goose-mcp/src/google_drive/mod.rs:1582`
- `crates/goose-mcp/src/google_drive/mod.rs:1787`

### `read_only_tools`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/permission/permission_judge.rs:108`

### `recipe`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose/src/recipe/mod.rs:291`
- `crates/goose/src/recipe/mod.rs:297`

### `recipe_path`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose/src/agents/schedule_tool.rs:85`
- `crates/goose/src/agents/subagent_execution_tool/task_types.rs:52`

### `refresh_token`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/oauth.rs:187`

### `replaceText`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:2757`

### `required`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-server/src/openapi.rs:146`

### `resolveComment`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:2527`

### `role`

**Method(s):** config_get

**Usage Locations (5):**

- `crates/goose-mcp/src/google_drive/mod.rs:3077`
- `crates/goose/src/providers/litellm.rs:276`
- `crates/goose/src/providers/litellm.rs:295`
- `crates/goose/src/providers/openrouter.rs:131`
- `crates/goose/src/providers/openrouter.rs:151`

### `row`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/computercontroller/mod.rs:945`

### `save_as`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/computercontroller/mod.rs:602`

### `save_output`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose-mcp/src/computercontroller/mod.rs:680`
- `crates/goose-mcp/src/computercontroller/mod.rs:800`

### `script`

**Method(s):** config_get

**Usage Locations (3):**

- `crates/goose-bench/src/eval_suites/core/computercontroller/script.rs:53`
- `crates/goose-mcp/src/computercontroller/mod.rs:675`
- `crates/goose-mcp/src/computercontroller/mod.rs:795`

### `search_text`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/computercontroller/mod.rs:887`

### `sequential_when_repeated`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/agents/subagent_execution_tool/task_types.rs:40`

### `server`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose/src/config/base.rs:97`
- `crates/goose/src/config/base.rs:97`

### `session_id`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/agents/schedule_tool.rs:369`

### `sheetName`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:1496`

### `signature`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose/src/providers/formats/databricks.rs:311`
- `crates/goose/src/providers/formats/snowflake.rs:257`

### `simple`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-cli/src/session/input.rs:462`

### `size`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/computercontroller/docx_tool.rs:44`

### `spreadsheetId`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:1443`

### `startPosition`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:2890`

### `state`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/oauth.rs:307`

### `status`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/utils.rs:229`

### `style`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/computercontroller/docx_tool.rs:184`

### `sub_recipe`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/agents/subagent_execution_tool/task_types.rs:28`

### `subagent_id`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-cli/src/session/mod.rs:1106`

### `summary`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/formats/databricks.rs:301`

### `supported_parameters`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/openrouter.rs:309`

### `target`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:3095`

### `targetId`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:1986`

### `task_ids`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/agents/subagent_execution_tool/lib/mod.rs:24`

### `task_parameters`

**Method(s):** config_get

**Usage Locations (3):**

- `crates/goose-cli/src/session/output.rs:428`
- `crates/goose/src/agents/recipe_tools/dynamic_task_tools.rs:72`
- `crates/goose/src/agents/recipe_tools/sub_recipe_tools.rs:47`

### `temperature`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/formats/anthropic.rs:937`

### `test_key`

**Method(s):** config_set, config_get

**Usage Locations (7):**

- `crates/goose/src/config/base.rs:807`
- `crates/goose/src/config/base.rs:807`
- `crates/goose/src/config/base.rs:807`
- `crates/goose/src/config/base.rs:810`
- `crates/goose/src/config/base.rs:810`
- `crates/goose/src/config/base.rs:815`
- `crates/goose/src/config/base.rs:815`

### `test_key_backup`

**Method(s):** config_set

**Usage Locations (3):**

- `crates/goose/src/config/base.rs:1148`
- `crates/goose/src/config/base.rs:1148`
- `crates/goose/src/config/base.rs:1148`

### `test_precedence`

**Method(s):** config_set, config_get

**Usage Locations (7):**

- `crates/goose/src/config/base.rs:1448`
- `crates/goose/src/config/base.rs:1448`
- `crates/goose/src/config/base.rs:1448`
- `crates/goose/src/config/base.rs:1451`
- `crates/goose/src/config/base.rs:1451`
- `crates/goose/src/config/base.rs:1458`
- `crates/goose/src/config/base.rs:1458`

### `text`

**Method(s):** config_get

**Usage Locations (11):**

- `crates/goose-mcp/src/google_drive/mod.rs:2627`
- `crates/goose-mcp/src/google_drive/mod.rs:2678`
- `crates/goose-mcp/src/google_drive/mod.rs:2753`
- `crates/goose-mcp/src/google_drive/mod.rs:2815`
- `crates/goose/src/providers/claude_code.rs:221`
- `crates/goose/src/providers/formats/anthropic.rs:538`
- `crates/goose/src/providers/formats/databricks.rs:295`
- `crates/goose/src/providers/formats/databricks.rs:307`
- `crates/goose/src/providers/formats/google.rs:244`
- `crates/goose/src/providers/formats/snowflake.rs:227`
- ... and 1 more locations

### `textValue`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:2245`

### `text_instruction`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose/src/agents/recipe_tools/dynamic_task_tools.rs:83`
- `crates/goose/src/agents/subagent_execution_tool/task_types.rs:59`

### `thinking`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose/src/providers/formats/anthropic.rs:932`
- `crates/goose/src/providers/formats/snowflake.rs:253`

### `third_key`

**Method(s):** config_set

**Usage Locations (3):**

- `crates/goose/src/config/base.rs:1156`
- `crates/goose/src/config/base.rs:1156`
- `crates/goose/src/config/base.rs:1156`

### `title`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:1709`

### `token_endpoint`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/oauth.rs:120`

### `tool_calls`

**Method(s):** config_get

**Usage Locations (4):**

- `crates/goose/src/providers/formats/databricks.rs:336`
- `crates/goose/src/providers/formats/openai.rs:235`
- `crates/goose/src/providers/formats/openai.rs:279`
- `crates/goose/src/providers/toolshim.rs:214`

### `tool_use_id`

**Method(s):** config_get

**Usage Locations (4):**

- `crates/goose/src/providers/formats/snowflake.rs:161`
- `crates/goose/src/providers/formats/snowflake.rs:235`
- `crates/goose/src/providers/snowflake.rs:182`
- `crates/goose/src/providers/snowflake.rs:205`

### `tools`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose/src/providers/formats/snowflake.rs:674`
- `crates/goose/src/providers/venice.rs:139`

### `totalTokenCount`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/formats/google.rs:288`

### `total_tokens`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/formats/openai.rs:347`

### `type`

**Method(s):** config_get

**Usage Locations (19):**

- `crates/goose-cli/src/recipes/github_recipe.rs:250`
- `crates/goose-cli/src/session/mod.rs:1108`
- `crates/goose-mcp/src/google_drive/mod.rs:3086`
- `crates/goose-server/src/openapi.rs:108`
- `crates/goose/src/agents/extension_manager.rs:163`
- `crates/goose/src/providers/claude_code.rs:208`
- `crates/goose/src/providers/claude_code.rs:217`
- `crates/goose/src/providers/formats/anthropic.rs:523`
- `crates/goose/src/providers/formats/anthropic.rs:536`
- `crates/goose/src/providers/formats/anthropic.rs:550`
- ... and 9 more locations

### `underline`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/computercontroller/docx_tool.rs:41`

### `unknown-model`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/base.rs:530`

### `updateLabels`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:2173`

### `uri`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose-mcp/src/google_drive/mod.rs:1378`
- `crates/mcp-server/src/router.rs:226`

### `url`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose-bench/src/eval_suites/core/computercontroller/web_scrape.rs:56`
- `crates/goose-mcp/src/google_drive/mod.rs:1379`

### `usage`

**Method(s):** config_get

**Usage Locations (15):**

- `crates/goose/src/providers/azure.rs:156`
- `crates/goose/src/providers/claude_code.rs:232`
- `crates/goose/src/providers/claude_code.rs:255`
- `crates/goose/src/providers/databricks.rs:259`
- `crates/goose/src/providers/formats/anthropic.rs:269`
- `crates/goose/src/providers/formats/anthropic.rs:505`
- `crates/goose/src/providers/formats/anthropic.rs:605`
- `crates/goose/src/providers/formats/anthropic.rs:643`
- `crates/goose/src/providers/formats/snowflake.rs:281`
- `crates/goose/src/providers/githubcopilot.rs:425`
- ... and 5 more locations

### `usageMetadata`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose/src/providers/formats/google.rs:278`

### `userValue`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:2272`

### `valid`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-cli/src/session/input.rs:481`

### `value`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:1661`

### `valueInputOption`

**Method(s):** config_get

**Usage Locations (2):**

- `crates/goose-mcp/src/google_drive/mod.rs:1609`
- `crates/goose-mcp/src/google_drive/mod.rs:1669`

### `values`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:1589`

### `version`

**Method(s):** config_set

**Usage Locations (3):**

- `crates/goose/src/config/base.rs:1213`
- `crates/goose/src/config/base.rs:1213`
- `crates/goose/src/config/base.rs:1213`

### `view_range`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/developer/mod.rs:784`

### `width`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/computercontroller/docx_tool.rs:224`

### `window_title`

**Method(s):** config_get

**Usage Locations (1):**

- `crates/goose-mcp/src/developer/mod.rs:1435`

### `worksheet`

**Method(s):** config_get

**Usage Locations (5):**

- `crates/goose-mcp/src/computercontroller/mod.rs:849`
- `crates/goose-mcp/src/computercontroller/mod.rs:872`
- `crates/goose-mcp/src/computercontroller/mod.rs:900`
- `crates/goose-mcp/src/computercontroller/mod.rs:922`
- `crates/goose-mcp/src/computercontroller/mod.rs:955`

## Secret Storage

### `ANTHROPIC_API_KEY`

**Method(s):** secret_get

**Usage Locations (2):**

- `crates/goose/src/providers/anthropic.rs:52`
- `crates/goose/src/providers/anthropic.rs:52`

### `AZURE_OPENAI_API_KEY`

**Method(s):** secret_get

**Usage Locations (1):**

- `crates/goose/src/providers/azure.rs:83`

### `ELEVENLABS_API_KEY`

**Method(s):** config_set, secret_get, config_get, config_delete

**Usage Locations (9):**

- `crates/goose-server/src/routes/audio.rs:212`
- `crates/goose-server/src/routes/audio.rs:212`
- `crates/goose-server/src/routes/audio.rs:216`
- `crates/goose-server/src/routes/audio.rs:216`
- `crates/goose-server/src/routes/audio.rs:223`
- `crates/goose-server/src/routes/audio.rs:223`
- `crates/goose-server/src/routes/audio.rs:231`
- `crates/goose-server/src/routes/audio.rs:231`
- `crates/goose-server/src/routes/audio.rs:339`

### `GITHUB_COPILOT_TOKEN`

**Method(s):** secret_set

**Usage Locations (5):**

- `crates/goose/src/providers/githubcopilot.rs:239`
- `crates/goose/src/providers/githubcopilot.rs:239`
- `crates/goose/src/providers/githubcopilot.rs:239`
- `crates/goose/src/providers/githubcopilot.rs:498`
- `crates/goose/src/providers/githubcopilot.rs:498`

### `GOOGLE_API_KEY`

**Method(s):** secret_get

**Usage Locations (2):**

- `crates/goose/src/providers/google.rs:59`
- `crates/goose/src/providers/google.rs:59`

### `GROQ_API_KEY`

**Method(s):** secret_get

**Usage Locations (2):**

- `crates/goose/src/providers/groq.rs:38`
- `crates/goose/src/providers/groq.rs:38`

### `LITELLM_API_KEY`

**Method(s):** secret_get

**Usage Locations (1):**

- `crates/goose/src/providers/litellm.rs:34`

### `LITELLM_CUSTOM_HEADERS`

**Method(s):** secret_get, config_get

**Usage Locations (3):**

- `crates/goose/src/providers/litellm.rs:43`
- `crates/goose/src/providers/litellm.rs:44`
- `crates/goose/src/providers/litellm.rs:44`

### `OPENAI_API_KEY`

**Method(s):** secret_get, config_get

**Usage Locations (5):**

- `crates/goose-server/src/routes/audio.rs:100`
- `crates/goose/src/config/base.rs:88`
- `crates/goose/src/config/base.rs:88`
- `crates/goose/src/providers/openai.rs:61`
- `crates/goose/src/providers/openai.rs:61`

### `OPENAI_CUSTOM_HEADERS`

**Method(s):** secret_get, config_get

**Usage Locations (3):**

- `crates/goose/src/providers/openai.rs:71`
- `crates/goose/src/providers/openai.rs:72`
- `crates/goose/src/providers/openai.rs:72`

### `OPENROUTER_API_KEY`

**Method(s):** secret_get, secret_set

**Usage Locations (5):**

- `crates/goose/src/config/signup_openrouter/mod.rs:168`
- `crates/goose/src/config/signup_openrouter/mod.rs:168`
- `crates/goose/src/config/signup_openrouter/mod.rs:168`
- `crates/goose/src/providers/openrouter.rs:46`
- `crates/goose/src/providers/openrouter.rs:46`

### `VENICE_API_KEY`

**Method(s):** secret_get

**Usage Locations (2):**

- `crates/goose/src/providers/venice.rs:88`
- `crates/goose/src/providers/venice.rs:88`

### `XAI_API_KEY`

**Method(s):** secret_get

**Usage Locations (2):**

- `crates/goose/src/providers/xai.rs:51`
- `crates/goose/src/providers/xai.rs:51`

### `api_key`

**Method(s):** secret_delete, secret_get, secret_set

**Usage Locations (12):**

- `crates/goose/src/config/base.rs:918`
- `crates/goose/src/config/base.rs:918`
- `crates/goose/src/config/base.rs:918`
- `crates/goose/src/config/base.rs:919`
- `crates/goose/src/config/base.rs:919`
- `crates/goose/src/config/base.rs:924`
- `crates/goose/src/config/base.rs:924`
- `crates/goose/src/config/base.rs:929`
- `crates/goose/src/config/base.rs:929`
- `crates/goose/src/config/base.rs:929`
- ... and 2 more locations

## Environment Variables

### `API_KEY`

**Method(s):** env_remove, env_set

**Usage Locations (4):**

- `crates/goose/src/config/base.rs:923`
- `crates/goose/src/config/base.rs:923`
- `crates/goose/src/config/base.rs:926`
- `crates/goose/src/config/base.rs:926`

### `CARGO_MANIFEST_DIR`

**Method(s):** env_var

**Usage Locations (1):**

- `crates/goose-server/src/bin/generate_schema.rs:9`

### `CLAUDE_THINKING_BUDGET`

**Method(s):** env_var

**Usage Locations (4):**

- `crates/goose/src/providers/formats/anthropic.rs:419`
- `crates/goose/src/providers/formats/anthropic.rs:419`
- `crates/goose/src/providers/formats/databricks.rs:563`
- `crates/goose/src/providers/formats/databricks.rs:563`

### `CLAUDE_THINKING_ENABLED`

**Method(s):** env_remove, env_set, env_var

**Usage Locations (14):**

- `crates/goose/src/providers/anthropic.rs:71`
- `crates/goose/src/providers/anthropic.rs:71`
- `crates/goose/src/providers/formats/anthropic.rs:416`
- `crates/goose/src/providers/formats/anthropic.rs:416`
- `crates/goose/src/providers/formats/anthropic.rs:915`
- `crates/goose/src/providers/formats/anthropic.rs:915`
- `crates/goose/src/providers/formats/anthropic.rs:916`
- `crates/goose/src/providers/formats/anthropic.rs:916`
- `crates/goose/src/providers/formats/anthropic.rs:944`
- `crates/goose/src/providers/formats/anthropic.rs:944`
- ... and 4 more locations

### `CONTEXT_FILE_NAMES`

**Method(s):** env_remove, env_set, env_var

**Usage Locations (10):**

- `crates/goose-mcp/src/developer/mod.rs:406`
- `crates/goose-mcp/src/developer/mod.rs:406`
- `crates/goose-mcp/src/developer/mod.rs:1714`
- `crates/goose-mcp/src/developer/mod.rs:1714`
- `crates/goose-mcp/src/developer/mod.rs:1723`
- `crates/goose-mcp/src/developer/mod.rs:1723`
- `crates/goose-mcp/src/developer/mod.rs:1731`
- `crates/goose-mcp/src/developer/mod.rs:1731`
- `crates/goose-mcp/src/developer/mod.rs:1739`
- `crates/goose-mcp/src/developer/mod.rs:1739`

### `DATABRICKS_TOKEN`

**Method(s):** secret_get, env_remove

**Usage Locations (4):**

- `crates/goose/examples/databricks_oauth.rs:16`
- `crates/goose/examples/databricks_oauth.rs:16`
- `crates/goose/src/providers/databricks.rs:128`
- `crates/goose/src/providers/databricks.rs:128`

### `GITHUB_ACTIONS`

**Method(s):** env_var

**Usage Locations (2):**

- `crates/goose-cli/src/scenario_tests/scenario_runner.rs:170` (test)
- `crates/goose-cli/src/scenario_tests/scenario_runner.rs:170` (test)

### `GOOGLE_DRIVE_CREDENTIALS_PATH`

**Method(s):** env_var

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:104`

### `GOOGLE_DRIVE_DISK_FALLBACK`

**Method(s):** env_const

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:47`

### `GOOGLE_DRIVE_OAUTH_CONFIG`

**Method(s):** env_var

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:119`

### `GOOGLE_DRIVE_OAUTH_PATH`

**Method(s):** env_var

**Usage Locations (1):**

- `crates/goose-mcp/src/google_drive/mod.rs:102`

### `GOOSE_ALLOWLIST`

**Method(s):** env_remove, env_set, env_var

**Usage Locations (3):**

- `crates/goose-server/src/routes/extension.rs:351`
- `crates/goose-server/src/routes/extension.rs:1057`
- `crates/goose-server/src/routes/extension.rs:1075`

### `GOOSE_ALLOWLIST_BYPASS`

**Method(s):** env_remove, env_set, env_var

**Usage Locations (8):**

- `crates/goose-server/src/routes/extension.rs:392`
- `crates/goose-server/src/routes/extension.rs:1096`
- `crates/goose-server/src/routes/extension.rs:1105`
- `crates/goose-server/src/routes/extension.rs:1112`
- `crates/goose-server/src/routes/extension.rs:1117`
- `crates/goose-server/src/routes/extension.rs:1119`
- `crates/goose-server/src/routes/extension.rs:1123`
- `crates/goose-server/src/routes/extension.rs:1155`

### `GOOSE_CACHE_DIR`

**Method(s):** env_var

**Usage Locations (2):**

- `crates/goose/src/providers/pricing.rs:16`
- `crates/goose/src/providers/pricing.rs:16`

### `GOOSE_CLAUDE_CODE_DEBUG`

**Method(s):** env_var

**Usage Locations (4):**

- `crates/goose/src/providers/claude_code.rs:310`
- `crates/goose/src/providers/claude_code.rs:310`
- `crates/goose/src/providers/claude_code.rs:429`
- `crates/goose/src/providers/claude_code.rs:429`

### `GOOSE_CLI_MIN_PRIORITY`

**Method(s):** config_set, env_var

**Usage Locations (11):**

- `crates/goose-cli/src/commands/configure.rs:1216`
- `crates/goose-cli/src/commands/configure.rs:1216`
- `crates/goose-cli/src/commands/configure.rs:1227`
- `crates/goose-cli/src/commands/configure.rs:1227`
- `crates/goose-cli/src/commands/configure.rs:1227`
- `crates/goose-cli/src/commands/configure.rs:1231`
- `crates/goose-cli/src/commands/configure.rs:1231`
- `crates/goose-cli/src/commands/configure.rs:1231`
- `crates/goose-cli/src/commands/configure.rs:1235`
- `crates/goose-cli/src/commands/configure.rs:1235`
- ... and 1 more locations

### `GOOSE_CLI_SHOW_THINKING`

**Method(s):** env_var

**Usage Locations (2):**

- `crates/goose-cli/src/session/output.rs:176`
- `crates/goose-cli/src/session/output.rs:176`

### `GOOSE_CLI_THEME`

**Method(s):** config_set, env_var

**Usage Locations (7):**

- `crates/goose-cli/src/session/output.rs:58`
- `crates/goose-cli/src/session/output.rs:58`
- `crates/goose-cli/src/session/output.rs:71`
- `crates/goose-cli/src/session/output.rs:71`
- `crates/goose-cli/src/session/output.rs:82`
- `crates/goose-cli/src/session/output.rs:82`
- `crates/goose-cli/src/session/output.rs:82`

### `GOOSE_CONTEXT_LIMIT`

**Method(s):** env_remove, env_set, env_var

**Usage Locations (5):**

- `crates/goose/src/model.rs:115`
- `crates/goose/src/model.rs:115`
- `crates/goose/src/providers/factory.rs:399`
- `crates/goose/src/providers/factory.rs:423`
- `crates/goose/src/providers/factory.rs:425`

### `GOOSE_DISABLE_KEYRING`

**Method(s):** env_var

**Usage Locations (1):**

- `crates/goose/src/config/base.rs:132`

### `GOOSE_EDITOR_API_KEY`

**Method(s):** env_var

**Usage Locations (2):**

- `crates/goose-mcp/src/developer/editor_models/mod.rs:78`
- `crates/goose-mcp/src/developer/editor_models/mod.rs:78`

### `GOOSE_EDITOR_HOST`

**Method(s):** env_var

**Usage Locations (2):**

- `crates/goose-mcp/src/developer/editor_models/mod.rs:79`
- `crates/goose-mcp/src/developer/editor_models/mod.rs:79`

### `GOOSE_EDITOR_MODEL`

**Method(s):** env_var

**Usage Locations (2):**

- `crates/goose-mcp/src/developer/editor_models/mod.rs:80`
- `crates/goose-mcp/src/developer/editor_models/mod.rs:80`

### `GOOSE_EMBEDDING_MODEL`

**Method(s):** env_var

**Usage Locations (5):**

- `crates/goose/src/agents/router_tool_selector.rs:48`
- `crates/goose/src/providers/litellm.rs:229`
- `crates/goose/src/providers/litellm.rs:229`
- `crates/goose/src/providers/openai.rs:268`
- `crates/goose/src/providers/openai.rs:268`

### `GOOSE_EMBEDDING_MODEL_PROVIDER`

**Method(s):** env_var

**Usage Locations (2):**

- `crates/goose/src/agents/router_tool_selector.rs:45`
- `crates/goose/src/agents/router_tool_selector.rs:51`

### `GOOSE_GEMINI_CLI_DEBUG`

**Method(s):** env_var

**Usage Locations (4):**

- `crates/goose/src/providers/gemini_cli.rs:161`
- `crates/goose/src/providers/gemini_cli.rs:161`
- `crates/goose/src/providers/gemini_cli.rs:280`
- `crates/goose/src/providers/gemini_cli.rs:280`

### `GOOSE_LEAD_FAILURE_THRESHOLD`

**Method(s):** env_remove, env_set, env_var

**Usage Locations (5):**

- `crates/goose/src/providers/factory.rs:291`
- `crates/goose/src/providers/factory.rs:324`
- `crates/goose/src/providers/factory.rs:345`
- `crates/goose/src/providers/factory.rs:352`
- `crates/goose/src/providers/factory.rs:381`

### `GOOSE_LEAD_FALLBACK_TURNS`

**Method(s):** env_remove, env_set, env_var

**Usage Locations (5):**

- `crates/goose/src/providers/factory.rs:295`
- `crates/goose/src/providers/factory.rs:325`
- `crates/goose/src/providers/factory.rs:346`
- `crates/goose/src/providers/factory.rs:353`
- `crates/goose/src/providers/factory.rs:384`

### `GOOSE_LEAD_MODEL`

**Method(s):** env_remove, env_set, env_var

**Usage Locations (11):**

- `crates/goose/src/providers/factory.rs:236`
- `crates/goose/src/providers/factory.rs:241`
- `crates/goose/src/providers/factory.rs:269`
- `crates/goose/src/providers/factory.rs:270`
- `crates/goose/src/providers/factory.rs:286`
- `crates/goose/src/providers/factory.rs:305`
- `crates/goose/src/providers/factory.rs:342`
- `crates/goose/src/providers/factory.rs:349`
- `crates/goose/src/providers/factory.rs:372`
- `crates/goose/src/providers/factory.rs:394`
- ... and 1 more locations

### `GOOSE_LEAD_PROVIDER`

**Method(s):** env_remove, env_set, env_var

**Usage Locations (8):**

- `crates/goose/src/providers/factory.rs:237`
- `crates/goose/src/providers/factory.rs:261`
- `crates/goose/src/providers/factory.rs:273`
- `crates/goose/src/providers/factory.rs:274`
- `crates/goose/src/providers/factory.rs:287`
- `crates/goose/src/providers/factory.rs:343`
- `crates/goose/src/providers/factory.rs:350`
- `crates/goose/src/providers/factory.rs:375`

### `GOOSE_LEAD_TURNS`

**Method(s):** env_remove, env_set, env_var

**Usage Locations (9):**

- `crates/goose/src/providers/factory.rs:238`
- `crates/goose/src/providers/factory.rs:262`
- `crates/goose/src/providers/factory.rs:277`
- `crates/goose/src/providers/factory.rs:278`
- `crates/goose/src/providers/factory.rs:288`
- `crates/goose/src/providers/factory.rs:323`
- `crates/goose/src/providers/factory.rs:344`
- `crates/goose/src/providers/factory.rs:351`
- `crates/goose/src/providers/factory.rs:378`

### `GOOSE_MODE`

**Method(s):** env_remove, env_var, env_set, config_set, config_get

**Usage Locations (35):**

- `crates/goose-cli/src/commands/configure.rs:1121`
- `crates/goose-cli/src/commands/configure.rs:1121`
- `crates/goose-cli/src/commands/configure.rs:1150`
- `crates/goose-cli/src/commands/configure.rs:1150`
- `crates/goose-cli/src/commands/configure.rs:1150`
- `crates/goose-cli/src/commands/configure.rs:1154`
- `crates/goose-cli/src/commands/configure.rs:1154`
- `crates/goose-cli/src/commands/configure.rs:1154`
- `crates/goose-cli/src/commands/configure.rs:1158`
- `crates/goose-cli/src/commands/configure.rs:1158`
- ... and 25 more locations

### `GOOSE_MODEL`

**Method(s):** env_remove, env_var, env_set, config_set, config_get

**Usage Locations (21):**

- `crates/goose-cli/src/commands/configure.rs:442`
- `crates/goose-cli/src/commands/configure.rs:442`
- `crates/goose-cli/src/commands/configure.rs:503`
- `crates/goose-cli/src/commands/configure.rs:503`
- `crates/goose-cli/src/commands/configure.rs:503`
- `crates/goose-cli/src/commands/configure.rs:1314`
- `crates/goose-cli/src/commands/configure.rs:1578`
- `crates/goose-cli/src/commands/configure.rs:1578`
- `crates/goose-cli/src/commands/web.rs:95`
- `crates/goose-cli/src/commands/web.rs:95`
- ... and 11 more locations

### `GOOSE_RECIPE_GITHUB_REPO`

**Method(s):** env_const

**Usage Locations (1):**

- `crates/goose-cli/src/recipes/github_recipe.rs:31`

### `GOOSE_RECIPE_ON_FAILURE_TIMEOUT_SECONDS`

**Method(s):** env_const

**Usage Locations (1):**

- `crates/goose/src/agents/retry.rs:35`

### `GOOSE_RECIPE_PATH`

**Method(s):** env_const

**Usage Locations (1):**

- `crates/goose-cli/src/recipes/search_recipe.rs:16`

### `GOOSE_RECIPE_RETRY_TIMEOUT_SECONDS`

**Method(s):** env_const

**Usage Locations (1):**

- `crates/goose/src/agents/retry.rs:32`

### `GOOSE_ROUTER_TOOL_SELECTION_STRATEGY`

**Method(s):** config_set, config_get, env_var

**Usage Locations (9):**

- `crates/goose-cli/src/commands/configure.rs:1174`
- `crates/goose-cli/src/commands/configure.rs:1174`
- `crates/goose-cli/src/commands/configure.rs:1193`
- `crates/goose-cli/src/commands/configure.rs:1193`
- `crates/goose-cli/src/commands/configure.rs:1193`
- `crates/goose-cli/src/commands/configure.rs:1202`
- `crates/goose-cli/src/commands/configure.rs:1202`
- `crates/goose-cli/src/commands/configure.rs:1202`
- `crates/goose/src/agents/tool_route_manager.rs:78`

### `GOOSE_SCHEDULER_TYPE`

**Method(s):** config_set, config_get, env_var

**Usage Locations (13):**

- `crates/goose-cli/src/commands/configure.rs:1469`
- `crates/goose-cli/src/commands/configure.rs:1469`
- `crates/goose-cli/src/commands/configure.rs:1475`
- `crates/goose-cli/src/commands/configure.rs:1492`
- `crates/goose-cli/src/commands/configure.rs:1492`
- `crates/goose-cli/src/commands/configure.rs:1492`
- `crates/goose-cli/src/commands/configure.rs:1498`
- `crates/goose-cli/src/commands/configure.rs:1498`
- `crates/goose-cli/src/commands/configure.rs:1498`
- `crates/goose-cli/src/commands/schedule.rs:266`
- ... and 3 more locations

### `GOOSE_SERVER__SECRET_KEY`

**Method(s):** env_var

**Usage Locations (2):**

- `crates/goose-server/src/commands/agent.rs:31`
- `crates/goose-server/src/commands/agent.rs:31`

### `GOOSE_SUBAGENT_MAX_TURNS`

**Method(s):** env_const

**Usage Locations (1):**

- `crates/goose/src/agents/subagent_task_config.rs:11`

### `GOOSE_TEMPERATURE`

**Method(s):** env_var

**Usage Locations (2):**

- `crates/goose/src/model.rs:141`
- `crates/goose/src/model.rs:141`

### `GOOSE_TEMPORAL_BIN`

**Method(s):** env_var

**Usage Locations (2):**

- `crates/goose/src/temporal_scheduler.rs:458`
- `crates/goose/src/temporal_scheduler.rs:458`

### `GOOSE_TEST_PROVIDER`

**Method(s):** env_var

**Usage Locations (2):**

- `crates/goose-cli/src/scenario_tests/scenario_runner.rs:52` (test)
- `crates/goose-cli/src/scenario_tests/scenario_runner.rs:52` (test)

### `GOOSE_TOOLSHIM`

**Method(s):** env_var

**Usage Locations (4):**

- `crates/goose-cli/src/commands/configure.rs:454`
- `crates/goose-cli/src/commands/configure.rs:454`
- `crates/goose/src/model.rs:162`
- `crates/goose/src/model.rs:162`

### `GOOSE_TOOLSHIM_OLLAMA_MODEL`

**Method(s):** env_var

**Usage Locations (6):**

- `crates/goose-cli/src/commands/configure.rs:461`
- `crates/goose-cli/src/commands/configure.rs:461`
- `crates/goose/src/model.rs:178`
- `crates/goose/src/model.rs:178`
- `crates/goose/src/providers/toolshim.rs:282`
- `crates/goose/src/providers/toolshim.rs:282`

### `GOOSE_VECTOR_DB_PATH`

**Method(s):** env_remove, env_set

**Usage Locations (5):**

- `crates/goose/src/agents/tool_vectordb.rs:554`
- `crates/goose/src/agents/tool_vectordb.rs:559`
- `crates/goose/src/agents/tool_vectordb.rs:568`
- `crates/goose/src/agents/tool_vectordb.rs:581`
- `crates/goose/src/agents/tool_vectordb.rs:589`

### `GOOSE_WORKER_CONTEXT_LIMIT`

**Method(s):** env_remove, env_set, env_var

**Usage Locations (3):**

- `crates/goose/src/providers/factory.rs:397`
- `crates/goose/src/providers/factory.rs:418`
- `crates/goose/src/providers/factory.rs:420`

### `GOOSE_WORKING_DIR`

**Method(s):** env_var

**Usage Locations (2):**

- `crates/goose-mcp/src/memory/mod.rs:228`
- `crates/goose-mcp/src/memory/mod.rs:228`

### `HOME`

**Method(s):** env_remove, env_set, env_var

**Usage Locations (10):**

- `crates/goose-cli/src/logging.rs:206`
- `crates/goose-cli/src/logging.rs:258`
- `crates/goose-cli/src/session/output.rs:885`
- `crates/goose-cli/src/session/output.rs:888`
- `crates/goose-cli/src/session/output.rs:903`
- `crates/goose-cli/src/session/output.rs:905`
- `crates/goose/src/providers/claude_code.rs:53`
- `crates/goose/src/providers/claude_code.rs:53`
- `crates/goose/src/providers/gemini_cli.rs:52`
- `crates/goose/src/providers/gemini_cli.rs:52`

### `LANGFUSE_INIT_PROJECT_PUBLIC_KEY`

**Method(s):** env_remove, env_set, env_var

**Usage Locations (7):**

- `crates/goose-cli/src/logging.rs:466`
- `crates/goose-cli/src/logging.rs:490`
- `crates/goose-cli/src/logging.rs:495`
- `crates/goose/src/tracing/langfuse_layer.rs:157`
- `crates/goose/src/tracing/langfuse_layer.rs:431`
- `crates/goose/src/tracing/langfuse_layer.rs:437`
- `crates/goose/src/tracing/langfuse_layer.rs:463`

### `LANGFUSE_INIT_PROJECT_SECRET_KEY`

**Method(s):** env_remove, env_set, env_var

**Usage Locations (6):**

- `crates/goose-cli/src/logging.rs:470`
- `crates/goose-cli/src/logging.rs:491`
- `crates/goose/src/tracing/langfuse_layer.rs:161`
- `crates/goose/src/tracing/langfuse_layer.rs:440`
- `crates/goose/src/tracing/langfuse_layer.rs:446`
- `crates/goose/src/tracing/langfuse_layer.rs:464`

### `LANGFUSE_PUBLIC_KEY`

**Method(s):** env_remove, env_set, env_var

**Usage Locations (8):**

- `crates/goose-cli/src/logging.rs:461`
- `crates/goose-cli/src/logging.rs:483`
- `crates/goose-cli/src/logging.rs:488`
- `crates/goose/src/tracing/langfuse_layer.rs:156`
- `crates/goose/src/tracing/langfuse_layer.rs:413`
- `crates/goose/src/tracing/langfuse_layer.rs:419`
- `crates/goose/src/tracing/langfuse_layer.rs:449`
- `crates/goose/src/tracing/langfuse_layer.rs:459`

### `LANGFUSE_SECRET_KEY`

**Method(s):** env_remove, env_set, env_var

**Usage Locations (8):**

- `crates/goose-cli/src/logging.rs:462`
- `crates/goose-cli/src/logging.rs:484`
- `crates/goose-cli/src/logging.rs:489`
- `crates/goose/src/tracing/langfuse_layer.rs:160`
- `crates/goose/src/tracing/langfuse_layer.rs:422`
- `crates/goose/src/tracing/langfuse_layer.rs:428`
- `crates/goose/src/tracing/langfuse_layer.rs:450`
- `crates/goose/src/tracing/langfuse_layer.rs:460`

### `LANGFUSE_URL`

**Method(s):** env_set, env_var

**Usage Locations (3):**

- `crates/goose-cli/src/logging.rs:463`
- `crates/goose/src/tracing/langfuse_layer.rs:169`
- `crates/goose/src/tracing/langfuse_layer.rs:451`

### `NO_COLOR`

**Method(s):** env_var_os

**Usage Locations (2):**

- `crates/goose-cli/src/session/output.rs:484`
- `crates/goose-cli/src/session/output.rs:484`

### `OTEL_EXPORTER_OTLP_ENDPOINT`

**Method(s):** env_set, env_remove, env_var

**Usage Locations (8):**

- `crates/goose-cli/src/main.rs:13`
- `crates/goose-cli/src/main.rs:13`
- `crates/goose/src/tracing/otlp_layer.rs:35`
- `crates/goose/src/tracing/otlp_layer.rs:249`
- `crates/goose/src/tracing/otlp_layer.rs:252`
- `crates/goose/src/tracing/otlp_layer.rs:255`
- `crates/goose/src/tracing/otlp_layer.rs:263`
- `crates/goose/src/tracing/otlp_layer.rs:264`

### `OTEL_EXPORTER_OTLP_TIMEOUT`

**Method(s):** env_remove, env_set, env_var

**Usage Locations (5):**

- `crates/goose/src/tracing/otlp_layer.rs:41`
- `crates/goose/src/tracing/otlp_layer.rs:250`
- `crates/goose/src/tracing/otlp_layer.rs:256`
- `crates/goose/src/tracing/otlp_layer.rs:267`
- `crates/goose/src/tracing/otlp_layer.rs:268`

### `PATH`

**Method(s):** env_var

**Usage Locations (4):**

- `crates/goose/src/providers/claude_code.rs:86`
- `crates/goose/src/providers/claude_code.rs:86`
- `crates/goose/src/providers/gemini_cli.rs:90`
- `crates/goose/src/providers/gemini_cli.rs:90`

### `PORT`

**Method(s):** env_remove, env_set, env_var

**Usage Locations (6):**

- `crates/goose/src/config/base.rs:1413`
- `crates/goose/src/config/base.rs:1413`
- `crates/goose/src/config/base.rs:1435`
- `crates/goose/src/config/base.rs:1435`
- `crates/goose/src/temporal_scheduler.rs:127`
- `crates/goose/src/temporal_scheduler.rs:127`

### `USER`

**Method(s):** env_var

**Usage Locations (2):**

- `crates/goose/src/agents/agent.rs:1360`
- `crates/goose/src/agents/agent.rs:1360`

### `WAYLAND_DISPLAY`

**Method(s):** env_var

**Usage Locations (2):**

- `crates/goose-mcp/src/computercontroller/platform/linux.rs:44`
- `crates/goose-mcp/src/computercontroller/platform/linux.rs:44`

## CLI Flags

### `--explain`

**Description:** Show the recipe title, description, and parameters

**Method(s):** clap_long

**Usage Locations (1):**

- `crates/goose-cli/src/cli.rs:469`

### `--interactive`

**Description:** Continue in interactive mode after processing initial input

**Method(s):** clap_long

**Usage Locations (1):**

- `crates/goose-cli/src/cli.rs:452`

### `--max-tool-repetitions`

**Description:** Maximum number of consecutive identical tool calls allowed

**Method(s):** clap_long

**Usage Locations (2):**

- `crates/goose-cli/src/cli.rs:327`
- `crates/goose-cli/src/cli.rs:483`

### `--no-session`

**Description:** Run without storing a session file

**Method(s):** clap_long

**Usage Locations (1):**

- `crates/goose-cli/src/cli.rs:460`

### `--quiet`

**Description:** Quiet mode. Suppress non-response output, printing only the model response to stdout

**Method(s):** clap_long

**Usage Locations (1):**

- `crates/goose-cli/src/cli.rs:563`

### `--render-recipe`

**Description:** Print the rendered recipe instead of running it.

**Method(s):** clap_long

**Usage Locations (1):**

- `crates/goose-cli/src/cli.rs:476`

### `--system`

**Description:** Additional system prompt to customize agent behavior

**Method(s):** clap_long

**Usage Locations (1):**

- `crates/goose-cli/src/cli.rs:420`

### `--text`

**Description:** Input text to provide to Goose directly

**Method(s):** clap_long

**Usage Locations (1):**

- `crates/goose-cli/src/cli.rs:408`

### `-q`

**Method(s):** clap_short

**Usage Locations (1):**

- `crates/goose-cli/src/cli.rs:563`

### `-s`

**Method(s):** clap_short

**Usage Locations (1):**

- `crates/goose-cli/src/cli.rs:452`

### `-t`

**Method(s):** clap_short

**Usage Locations (1):**

- `crates/goose-cli/src/cli.rs:408`

