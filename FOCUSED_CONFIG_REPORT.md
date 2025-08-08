# Focused Goose Configuration Summary

**Total Unique Configuration Items:** 139

## Config File Parameters (69 items)

### `ANTHROPIC_HOST`
**Method:** get_param
**Locations:** 1 usage(s)
- crates/goose/src/providers/anthropic.rs:54

### `AZURE_OPENAI_API_VERSION`
**Method:** get_param
**Locations:** 1 usage(s)
- crates/goose/src/providers/azure.rs:79

### `AZURE_OPENAI_DEPLOYMENT_NAME`
**Method:** get_param
**Locations:** 1 usage(s)
- crates/goose/src/providers/azure.rs:77

### `AZURE_OPENAI_ENDPOINT`
**Method:** get_param
**Locations:** 1 usage(s)
- crates/goose/src/providers/azure.rs:76

### `CLAUDE_CODE_COMMAND`
**Method:** get_param
**Locations:** 1 usage(s)
- crates/goose/src/providers/claude_code.rs:36

### `DATABRICKS_BACKOFF_MULTIPLIER`
**Method:** get_param
**Locations:** 1 usage(s)
- crates/goose/src/providers/databricks.rs:163

### `DATABRICKS_HOST`
**Method:** get_param
**Locations:** 2 usage(s)
- crates/goose/src/providers/databricks.rs:113
- crates/goose/src/providers/databricks.rs:115

### `DATABRICKS_INITIAL_RETRY_INTERVAL_MS`
**Method:** get_param
**Locations:** 1 usage(s)
- crates/goose/src/providers/databricks.rs:157

### `DATABRICKS_MAX_RETRIES`
**Method:** get_param
**Locations:** 1 usage(s)
- crates/goose/src/providers/databricks.rs:151

### `DATABRICKS_MAX_RETRY_INTERVAL_MS`
**Method:** get_param
**Locations:** 1 usage(s)
- crates/goose/src/providers/databricks.rs:169

### `GCP_BACKOFF_MULTIPLIER`
**Method:** get_param
**Locations:** 1 usage(s)
- crates/goose/src/providers/gcpvertexai.rs:148

### `GCP_INITIAL_RETRY_INTERVAL_MS`
**Method:** get_param
**Locations:** 1 usage(s)
- crates/goose/src/providers/gcpvertexai.rs:142

### `GCP_LOCATION`
**Method:** get_param
**Locations:** 1 usage(s)
- crates/goose/src/providers/gcpvertexai.rs:174

### `GCP_MAX_RETRIES`
**Method:** get_param
**Locations:** 1 usage(s)
- crates/goose/src/providers/gcpvertexai.rs:136

### `GCP_MAX_RETRY_INTERVAL_MS`
**Method:** get_param
**Locations:** 1 usage(s)
- crates/goose/src/providers/gcpvertexai.rs:154

### `GCP_PROJECT_ID`
**Method:** get_param
**Locations:** 1 usage(s)
- crates/goose/src/providers/gcpvertexai.rs:108

### `GEMINI_CLI_COMMAND`
**Method:** get_param
**Locations:** 1 usage(s)
- crates/goose/src/providers/gemini_cli.rs:35

### `GOOGLE_HOST`
**Method:** get_param
**Locations:** 1 usage(s)
- crates/goose/src/providers/google.rs:61

### `GOOSE_AUTO_COMPACT_THRESHOLD`
**Method:** set_param
**Locations:** 2 usage(s)
- crates/goose/src/context_mgmt/auto_compact.rs:546
- crates/goose/src/context_mgmt/auto_compact.rs:576

### `GOOSE_CLI_MIN_PRIORITY`
**Method:** set_param
**Locations:** 4 usage(s)
- crates/goose-cli/src/commands/configure.rs:1227
- crates/goose-cli/src/commands/configure.rs:1231
- crates/goose-cli/src/commands/configure.rs:1235
- ... and 1 more

### `GOOSE_CLI_THEME`
**Method:** set_param
**Locations:** 3 usage(s)
- crates/goose-cli/src/session/output.rs:71
- crates/goose-cli/src/session/output.rs:82
- crates/goose-cli/src/session/output.rs:58

### `GOOSE_MAX_TURNS`
**Method:** get_param
**Locations:** 3 usage(s)
- crates/goose/src/agents/agent.rs:884
- crates/goose-cli/src/commands/configure.rs:1522
- crates/goose-cli/src/commands/configure.rs:1541

### `GOOSE_MODE`
**Method:** get_param
**Locations:** 15 usage(s)
- crates/goose/src/providers/claude_code.rs:538
- crates/goose/src/providers/claude_code.rs:535
- crates/goose/src/providers/ollama.rs:134
- ... and 12 more

### `GOOSE_MODEL`
**Method:** get_param
**Locations:** 10 usage(s)
- crates/goose/src/scheduler.rs:1135
- crates/goose/src/scheduler.rs:1428
- crates/goose/src/config/signup_openrouter/mod.rs:170
- ... and 7 more

### `GOOSE_PROVIDER`
**Method:** get_param
**Locations:** 9 usage(s)
- crates/goose/src/scheduler.rs:1125
- crates/goose/src/scheduler.rs:1427
- crates/goose/src/agents/agent.rs:1370
- ... and 6 more

### `GOOSE_ROUTER_TOOL_SELECTION_STRATEGY`
**Method:** get_param
**Locations:** 4 usage(s)
- crates/goose/src/agents/tool_route_manager.rs:78
- crates/goose-cli/src/commands/configure.rs:1193
- crates/goose-cli/src/commands/configure.rs:1202
- ... and 1 more

### `GOOSE_SCHEDULER_TYPE`
**Method:** get_param
**Locations:** 6 usage(s)
- crates/goose-cli/src/commands/configure.rs:1475
- crates/goose-cli/src/commands/configure.rs:1492
- crates/goose-cli/src/commands/configure.rs:1498
- ... and 3 more

### `GOOSE_SYSTEM_PROMPT_FILE_PATH`
**Method:** get_param
**Locations:** 1 usage(s)
- crates/goose-cli/src/session/builder.rs:563

### `GROQ_HOST`
**Method:** get_param
**Locations:** 1 usage(s)
- crates/goose/src/providers/groq.rs:40

### `LITELLM_BASE_PATH`
**Method:** get_param
**Locations:** 1 usage(s)
- crates/goose/src/providers/litellm.rs:40

### `LITELLM_CUSTOM_HEADERS`
**Method:** get_param
**Locations:** 2 usage(s)
- crates/goose/src/providers/litellm.rs:44
- crates/goose/src/providers/litellm.rs:43

### `LITELLM_HOST`
**Method:** get_param
**Locations:** 1 usage(s)
- crates/goose/src/providers/litellm.rs:37

### `LITELLM_TIMEOUT`
**Method:** get_param
**Locations:** 1 usage(s)
- crates/goose/src/providers/litellm.rs:47

### `OLLAMA_HOST`
**Method:** get_param
**Locations:** 2 usage(s)
- crates/goose/src/providers/toolshim.rs:88
- crates/goose/src/providers/ollama.rs:41

### `OLLAMA_TIMEOUT`
**Method:** get_param
**Locations:** 1 usage(s)
- crates/goose/src/providers/ollama.rs:45

### `OPENAI_API_KEY`
**Method:** get_param
**Locations:** 3 usage(s)
- crates/goose/src/config/base.rs:88
- crates/goose/src/providers/openai.rs:61
- crates/goose-server/src/routes/audio.rs:100

### `OPENAI_BASE_PATH`
**Method:** get_param
**Locations:** 1 usage(s)
- crates/goose/src/providers/openai.rs:66

### `OPENAI_CUSTOM_HEADERS`
**Method:** get_param
**Locations:** 2 usage(s)
- crates/goose/src/providers/openai.rs:72
- crates/goose/src/providers/openai.rs:71

### `OPENAI_HOST`
**Method:** get_param
**Locations:** 1 usage(s)
- crates/goose/src/providers/openai.rs:63

### `OPENAI_ORGANIZATION`
**Method:** get_param
**Locations:** 1 usage(s)
- crates/goose/src/providers/openai.rs:68

### `OPENAI_PROJECT`
**Method:** get_param
**Locations:** 1 usage(s)
- crates/goose/src/providers/openai.rs:69

### `OPENAI_TIMEOUT`
**Method:** get_param
**Locations:** 1 usage(s)
- crates/goose/src/providers/openai.rs:75

### `OPENROUTER_HOST`
**Method:** get_param
**Locations:** 1 usage(s)
- crates/goose/src/providers/openrouter.rs:48

### `RANDOM_THINKING_MESSAGES`
**Method:** get_param
**Locations:** 1 usage(s)
- crates/goose-cli/src/session/output.rs:101

### `SAGEMAKER_ENDPOINT_NAME`
**Method:** get_param
**Locations:** 1 usage(s)
- crates/goose/src/providers/sagemaker_tgi.rs:40

### `SNOWFLAKE_HOST`
**Method:** get_param
**Locations:** 2 usage(s)
- crates/goose/src/providers/snowflake.rs:48
- crates/goose/src/providers/snowflake.rs:50

### `SNOWFLAKE_TOKEN`
**Method:** get_param
**Locations:** 2 usage(s)
- crates/goose/src/providers/snowflake.rs:69
- crates/goose/src/providers/snowflake.rs:72

### `VENICE_BASE_PATH`
**Method:** get_param
**Locations:** 1 usage(s)
- crates/goose/src/providers/venice.rs:93

### `VENICE_HOST`
**Method:** get_param
**Locations:** 1 usage(s)
- crates/goose/src/providers/venice.rs:90

### `VENICE_MODELS_PATH`
**Method:** get_param
**Locations:** 1 usage(s)
- crates/goose/src/providers/venice.rs:96

### `XAI_HOST`
**Method:** get_param
**Locations:** 1 usage(s)
- crates/goose/src/providers/xai.rs:53

### `another_key`
**Method:** set_param
**Locations:** 1 usage(s)
- crates/goose/src/config/base.rs:1149

### `complex_key`
**Method:** get_param
**Locations:** 2 usage(s)
- crates/goose/src/config/base.rs:841
- crates/goose/src/config/base.rs:833

### `config`
**Method:** get_param
**Locations:** 1 usage(s)
- crates/goose/src/config/base.rs:1429

### `enabled`
**Method:** get_param
**Locations:** 1 usage(s)
- crates/goose/src/config/base.rs:1419

### `experiments`
**Method:** get_param
**Locations:** 3 usage(s)
- crates/goose/src/config/experiments.rs:23
- crates/goose/src/config/experiments.rs:33
- crates/goose/src/config/experiments.rs:38

### `extensions`
**Method:** get_param
**Locations:** 12 usage(s)
- crates/goose/src/config/extensions.rs:36
- crates/goose/src/config/extensions.rs:72
- crates/goose/src/config/extensions.rs:130
- ... and 9 more

### `key`
**Method:** get_param
**Locations:** 7 usage(s)
- crates/goose/src/config/base.rs:880
- crates/goose/src/config/base.rs:885
- crates/goose/src/config/base.rs:878
- ... and 4 more

### `key1`
**Method:** set_param
**Locations:** 7 usage(s)
- crates/goose/src/config/base.rs:862
- crates/goose/src/config/base.rs:1044
- crates/goose/src/config/base.rs:1192
- ... and 4 more

### `key2`
**Method:** set_param
**Locations:** 5 usage(s)
- crates/goose/src/config/base.rs:863
- crates/goose/src/config/base.rs:1054
- crates/goose/src/config/base.rs:950
- ... and 2 more

### `nonexistent_key`
**Method:** get_param
**Locations:** 1 usage(s)
- crates/goose/src/config/base.rs:853

### `port`
**Method:** get_param
**Locations:** 1 usage(s)
- crates/goose/src/config/base.rs:1414

### `provider`
**Method:** get_param
**Locations:** 1 usage(s)
- crates/goose/src/config/base.rs:1409

### `server`
**Method:** get_param
**Locations:** 1 usage(s)
- crates/goose/src/config/base.rs:97

### `test_key`
**Method:** get_param
**Locations:** 3 usage(s)
- crates/goose/src/config/base.rs:810
- crates/goose/src/config/base.rs:815
- crates/goose/src/config/base.rs:807

### `test_key_backup`
**Method:** set_param
**Locations:** 1 usage(s)
- crates/goose/src/config/base.rs:1148

### `test_precedence`
**Method:** get_param
**Locations:** 3 usage(s)
- crates/goose/src/config/base.rs:1451
- crates/goose/src/config/base.rs:1458
- crates/goose/src/config/base.rs:1448

### `third_key`
**Method:** set_param
**Locations:** 1 usage(s)
- crates/goose/src/config/base.rs:1156

### `version`
**Method:** set_param
**Locations:** 1 usage(s)
- crates/goose/src/config/base.rs:1213

## Environment Variables (47 items)

### `API_KEY`
**Method:** std::env::set_var
**Locations:** 1 usage(s)
- crates/goose/src/config/base.rs:923

### `CARGO_MANIFEST_DIR`
**Method:** env::var
**Locations:** 1 usage(s)
- crates/goose-server/src/bin/generate_schema.rs:9

### `CLAUDE_THINKING_BUDGET`
**Method:** std::env::var
**Locations:** 2 usage(s)
- crates/goose/src/providers/formats/databricks.rs:563
- crates/goose/src/providers/formats/anthropic.rs:419

### `CLAUDE_THINKING_ENABLED`
**Method:** std::env::var
**Locations:** 6 usage(s)
- crates/goose/src/providers/anthropic.rs:71
- crates/goose/src/providers/formats/databricks.rs:560
- crates/goose/src/providers/formats/anthropic.rs:416
- ... and 3 more

### `CONTEXT_FILE_NAMES`
**Method:** std::env::var
**Locations:** 3 usage(s)
- crates/goose-mcp/src/developer/mod.rs:406
- crates/goose-mcp/src/developer/mod.rs:1714
- crates/goose-mcp/src/developer/mod.rs:1731

### `GITHUB_ACTIONS`
**Method:** std::env::var
**Locations:** 1 usage(s)
- crates/goose-cli/src/scenario_tests/scenario_runner.rs:170

### `GOOGLE_DRIVE_CREDENTIALS_PATH`
**Method:** env::var
**Locations:** 1 usage(s)
- crates/goose-mcp/src/google_drive/mod.rs:104

### `GOOGLE_DRIVE_OAUTH_CONFIG`
**Method:** env::var
**Locations:** 1 usage(s)
- crates/goose-mcp/src/google_drive/mod.rs:119

### `GOOGLE_DRIVE_OAUTH_PATH`
**Method:** env::var
**Locations:** 1 usage(s)
- crates/goose-mcp/src/google_drive/mod.rs:102

### `GOOSE_ALLOWLIST`
**Method:** env::var
**Locations:** 2 usage(s)
- crates/goose-server/src/routes/extension.rs:351
- crates/goose-server/src/routes/extension.rs:1057

### `GOOSE_ALLOWLIST_BYPASS`
**Method:** env::var
**Locations:** 5 usage(s)
- crates/goose-server/src/routes/extension.rs:392
- crates/goose-server/src/routes/extension.rs:1123
- crates/goose-server/src/routes/extension.rs:1096
- ... and 2 more

### `GOOSE_CACHE_DIR`
**Method:** std::env::var
**Locations:** 1 usage(s)
- crates/goose/src/providers/pricing.rs:16

### `GOOSE_CLAUDE_CODE_DEBUG`
**Method:** std::env::var
**Locations:** 2 usage(s)
- crates/goose/src/providers/claude_code.rs:310
- crates/goose/src/providers/claude_code.rs:429

### `GOOSE_CLI_SHOW_THINKING`
**Method:** std::env::var
**Locations:** 1 usage(s)
- crates/goose-cli/src/session/output.rs:176

### `GOOSE_CONTEXT_LIMIT`
**Method:** std::env::var
**Locations:** 3 usage(s)
- crates/goose/src/model.rs:115
- crates/goose/src/providers/factory.rs:399
- crates/goose/src/providers/factory.rs:423

### `GOOSE_DISABLE_KEYRING`
**Method:** env::var
**Locations:** 1 usage(s)
- crates/goose/src/config/base.rs:132

### `GOOSE_EDITOR_API_KEY`
**Method:** std::env::var
**Locations:** 1 usage(s)
- crates/goose-mcp/src/developer/editor_models/mod.rs:78

### `GOOSE_EDITOR_HOST`
**Method:** std::env::var
**Locations:** 1 usage(s)
- crates/goose-mcp/src/developer/editor_models/mod.rs:79

### `GOOSE_EDITOR_MODEL`
**Method:** std::env::var
**Locations:** 1 usage(s)
- crates/goose-mcp/src/developer/editor_models/mod.rs:80

### `GOOSE_EMBEDDING_MODEL`
**Method:** std::env::var
**Locations:** 3 usage(s)
- crates/goose/src/providers/litellm.rs:229
- crates/goose/src/providers/openai.rs:268
- crates/goose/src/agents/router_tool_selector.rs:48

### `GOOSE_EMBEDDING_MODEL_PROVIDER`
**Method:** env::var
**Locations:** 2 usage(s)
- crates/goose/src/agents/router_tool_selector.rs:45
- crates/goose/src/agents/router_tool_selector.rs:51

### `GOOSE_GEMINI_CLI_DEBUG`
**Method:** std::env::var
**Locations:** 2 usage(s)
- crates/goose/src/providers/gemini_cli.rs:161
- crates/goose/src/providers/gemini_cli.rs:280

### `GOOSE_LEAD_FAILURE_THRESHOLD`
**Method:** env::var
**Locations:** 4 usage(s)
- crates/goose/src/providers/factory.rs:291
- crates/goose/src/providers/factory.rs:345
- crates/goose/src/providers/factory.rs:324
- ... and 1 more

### `GOOSE_LEAD_FALLBACK_TURNS`
**Method:** env::var
**Locations:** 4 usage(s)
- crates/goose/src/providers/factory.rs:295
- crates/goose/src/providers/factory.rs:346
- crates/goose/src/providers/factory.rs:325
- ... and 1 more

### `GOOSE_LEAD_MODEL`
**Method:** env::var
**Locations:** 9 usage(s)
- crates/goose/src/providers/factory.rs:236
- crates/goose/src/providers/factory.rs:286
- crates/goose/src/providers/factory.rs:342
- ... and 6 more

### `GOOSE_LEAD_PROVIDER`
**Method:** env::var
**Locations:** 6 usage(s)
- crates/goose/src/providers/factory.rs:237
- crates/goose/src/providers/factory.rs:287
- crates/goose/src/providers/factory.rs:343
- ... and 3 more

### `GOOSE_LEAD_TURNS`
**Method:** env::var
**Locations:** 7 usage(s)
- crates/goose/src/providers/factory.rs:238
- crates/goose/src/providers/factory.rs:288
- crates/goose/src/providers/factory.rs:344
- ... and 4 more

### `GOOSE_SERVER__SECRET_KEY`
**Method:** std::env::var
**Locations:** 1 usage(s)
- crates/goose-server/src/commands/agent.rs:31

### `GOOSE_TEMPERATURE`
**Method:** std::env::var
**Locations:** 1 usage(s)
- crates/goose/src/model.rs:141

### `GOOSE_TEMPORAL_BIN`
**Method:** std::env::var
**Locations:** 1 usage(s)
- crates/goose/src/temporal_scheduler.rs:458

### `GOOSE_TEST_PROVIDER`
**Method:** std::env::var
**Locations:** 1 usage(s)
- crates/goose-cli/src/scenario_tests/scenario_runner.rs:52

### `GOOSE_TOOLSHIM`
**Method:** std::env::var
**Locations:** 2 usage(s)
- crates/goose/src/model.rs:162
- crates/goose-cli/src/commands/configure.rs:454

### `GOOSE_TOOLSHIM_OLLAMA_MODEL`
**Method:** std::env::var
**Locations:** 3 usage(s)
- crates/goose/src/model.rs:178
- crates/goose/src/providers/toolshim.rs:282
- crates/goose-cli/src/commands/configure.rs:461

### `GOOSE_VECTOR_DB_PATH`
**Method:** env::set_var
**Locations:** 2 usage(s)
- crates/goose/src/agents/tool_vectordb.rs:554
- crates/goose/src/agents/tool_vectordb.rs:568

### `GOOSE_WORKER_CONTEXT_LIMIT`
**Method:** env::var
**Locations:** 2 usage(s)
- crates/goose/src/providers/factory.rs:397
- crates/goose/src/providers/factory.rs:418

### `GOOSE_WORKING_DIR`
**Method:** std::env::var
**Locations:** 1 usage(s)
- crates/goose-mcp/src/memory/mod.rs:228

### `HOME`
**Method:** std::env::var
**Locations:** 7 usage(s)
- crates/goose/src/providers/claude_code.rs:53
- crates/goose/src/providers/gemini_cli.rs:52
- crates/goose-cli/src/logging.rs:206
- ... and 4 more

### `LANGFUSE_INIT_PROJECT_PUBLIC_KEY`
**Method:** env::var
**Locations:** 5 usage(s)
- crates/goose/src/tracing/langfuse_layer.rs:157
- crates/goose/src/tracing/langfuse_layer.rs:431
- crates/goose/src/tracing/langfuse_layer.rs:463
- ... and 2 more

### `LANGFUSE_INIT_PROJECT_SECRET_KEY`
**Method:** env::var
**Locations:** 5 usage(s)
- crates/goose/src/tracing/langfuse_layer.rs:161
- crates/goose/src/tracing/langfuse_layer.rs:440
- crates/goose/src/tracing/langfuse_layer.rs:464
- ... and 2 more

### `LANGFUSE_PUBLIC_KEY`
**Method:** env::var
**Locations:** 5 usage(s)
- crates/goose/src/tracing/langfuse_layer.rs:156
- crates/goose/src/tracing/langfuse_layer.rs:413
- crates/goose/src/tracing/langfuse_layer.rs:449
- ... and 2 more

### `LANGFUSE_SECRET_KEY`
**Method:** env::var
**Locations:** 5 usage(s)
- crates/goose/src/tracing/langfuse_layer.rs:160
- crates/goose/src/tracing/langfuse_layer.rs:422
- crates/goose/src/tracing/langfuse_layer.rs:450
- ... and 2 more

### `LANGFUSE_URL`
**Method:** env::var
**Locations:** 3 usage(s)
- crates/goose/src/tracing/langfuse_layer.rs:169
- crates/goose/src/tracing/langfuse_layer.rs:451
- crates/goose-cli/src/logging.rs:463

### `OTEL_EXPORTER_OTLP_ENDPOINT`
**Method:** env::var
**Locations:** 5 usage(s)
- crates/goose/src/tracing/otlp_layer.rs:35
- crates/goose/src/tracing/otlp_layer.rs:249
- crates/goose/src/tracing/otlp_layer.rs:255
- ... and 2 more

### `OTEL_EXPORTER_OTLP_TIMEOUT`
**Method:** env::var
**Locations:** 4 usage(s)
- crates/goose/src/tracing/otlp_layer.rs:41
- crates/goose/src/tracing/otlp_layer.rs:250
- crates/goose/src/tracing/otlp_layer.rs:256
- ... and 1 more

### `PATH`
**Method:** std::env::var
**Locations:** 2 usage(s)
- crates/goose/src/providers/claude_code.rs:86
- crates/goose/src/providers/gemini_cli.rs:90

### `USER`
**Method:** std::env::var
**Locations:** 1 usage(s)
- crates/goose/src/agents/agent.rs:1360

### `WAYLAND_DISPLAY`
**Method:** std::env::var
**Locations:** 1 usage(s)
- crates/goose-mcp/src/computercontroller/platform/linux.rs:44

## Secret Storage (12 items)

### `ANTHROPIC_API_KEY`
**Method:** get_secret
**Locations:** 1 usage(s)
- crates/goose/src/providers/anthropic.rs:52

### `AZURE_OPENAI_API_KEY`
**Method:** get_secret
**Locations:** 1 usage(s)
- crates/goose/src/providers/azure.rs:83

### `DATABRICKS_TOKEN`
**Method:** get_secret
**Locations:** 1 usage(s)
- crates/goose/src/providers/databricks.rs:128

### `ELEVENLABS_API_KEY`
**Method:** get_secret
**Locations:** 1 usage(s)
- crates/goose-server/src/routes/audio.rs:212

### `GITHUB_COPILOT_TOKEN`
**Method:** set_secret
**Locations:** 2 usage(s)
- crates/goose/src/providers/githubcopilot.rs:239
- crates/goose/src/providers/githubcopilot.rs:498

### `GOOGLE_API_KEY`
**Method:** get_secret
**Locations:** 1 usage(s)
- crates/goose/src/providers/google.rs:59

### `GROQ_API_KEY`
**Method:** get_secret
**Locations:** 1 usage(s)
- crates/goose/src/providers/groq.rs:38

### `LITELLM_API_KEY`
**Method:** get_secret
**Locations:** 1 usage(s)
- crates/goose/src/providers/litellm.rs:34

### `OPENROUTER_API_KEY`
**Method:** get_secret
**Locations:** 2 usage(s)
- crates/goose/src/providers/openrouter.rs:46
- crates/goose/src/config/signup_openrouter/mod.rs:168

### `VENICE_API_KEY`
**Method:** get_secret
**Locations:** 1 usage(s)
- crates/goose/src/providers/venice.rs:88

### `XAI_API_KEY`
**Method:** get_secret
**Locations:** 1 usage(s)
- crates/goose/src/providers/xai.rs:51

### `api_key`
**Method:** get_secret
**Locations:** 5 usage(s)
- crates/goose/src/config/base.rs:919
- crates/goose/src/config/base.rs:924
- crates/goose/src/config/base.rs:930
- ... and 2 more

## CLI Flags (11 items)

### `--explain`
**Description:** Show the recipe title, description, and parameters
**Method:** clap_long
**Locations:** 1 usage(s)
- crates/goose-cli/src/cli.rs:469

### `--interactive`
**Description:** Continue in interactive mode after processing initial input
**Method:** clap_long
**Locations:** 1 usage(s)
- crates/goose-cli/src/cli.rs:452

### `--max-tool-repetitions`
**Description:** Maximum number of consecutive identical tool calls allowed
**Method:** clap_long
**Locations:** 2 usage(s)
- crates/goose-cli/src/cli.rs:327
- crates/goose-cli/src/cli.rs:483

### `--no-session`
**Description:** Run without storing a session file
**Method:** clap_long
**Locations:** 1 usage(s)
- crates/goose-cli/src/cli.rs:460

### `--quiet`
**Description:** Quiet mode. Suppress non-response output, printing only the model response to stdout
**Method:** clap_long
**Locations:** 1 usage(s)
- crates/goose-cli/src/cli.rs:563

### `--render-recipe`
**Description:** Print the rendered recipe instead of running it.
**Method:** clap_long
**Locations:** 1 usage(s)
- crates/goose-cli/src/cli.rs:476

### `--system`
**Description:** Additional system prompt to customize agent behavior
**Method:** clap_long
**Locations:** 1 usage(s)
- crates/goose-cli/src/cli.rs:420

### `--text`
**Description:** Input text to provide to Goose directly
**Method:** clap_long
**Locations:** 1 usage(s)
- crates/goose-cli/src/cli.rs:408

### `-q`
**Method:** clap_short
**Locations:** 1 usage(s)
- crates/goose-cli/src/cli.rs:563

### `-s`
**Method:** clap_short
**Locations:** 1 usage(s)
- crates/goose-cli/src/cli.rs:452

### `-t`
**Method:** clap_short
**Locations:** 1 usage(s)
- crates/goose-cli/src/cli.rs:408

