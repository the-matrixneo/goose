# BitMortar: Unified LLM Provider Gateway

## Overview

BitMortar is a server that provides a Databricks-compatible API while routing requests to various LLM providers behind the scenes. It allows the Databricks provider in Goose to connect to **any** supported provider (OpenAI, Anthropic, Groq, XAI, etc.) through a uniform interface.

## Architecture

```
┌─────────────────────┐       ┌─────────────────────┐       ┌─────────────────────┐
│                     │       │                     │       │                     │
│  Goose Databricks   │──────▶│    BitMortar        │──────▶│   OpenAI Provider   │
│     Provider        │       │     Server          │       │                     │
│                     │       │                     │       └─────────────────────┘
└─────────────────────┘       │                     │       ┌─────────────────────┐
                               │  - Route by model   │──────▶│ Anthropic Provider  │
                               │  - Load balancing   │       │                     │
                               │  - Health checking  │       └─────────────────────┘
                               │  - Fallback support │       ┌─────────────────────┐
                               │                     │──────▶│   Other Providers   │
                               └─────────────────────┘       │  (Groq, XAI, etc.) │
                                                             └─────────────────────┘
```

## Key Features

### 1. **Reuses Goose Provider Code**
- BitMortar wraps existing Goose providers instead of reimplementing them
- Maintains consistency with Goose's provider implementations
- Automatically inherits all provider features (OAuth, retry logic, error handling, etc.)

### 2. **Databricks-Compatible API**
- Provides endpoints that match Databricks Model Serving format:
  - `POST /serving-endpoints/{model}/invocations` - Chat completions
  - `GET /serving-endpoints` - List available endpoints
  - `POST /v1/embeddings` - Create embeddings
  - `GET /v1/models` - List models

### 3. **Intelligent Routing**
- Route specific models to specific providers
- Fallback support if primary provider fails
- Load balancing strategies (Priority, Round Robin, Random, First Available)

### 4. **Configuration-Driven**
- TOML configuration file
- Environment variable support for API keys
- Hot-swappable provider configurations

## Implementation Details

### Core Components

1. **BitMortarProvider** (`src/providers/mod.rs`)
   - Wraps Goose providers
   - Handles request/response conversion between Databricks and Goose formats
   - Supports all Goose provider features

2. **BitMortarServer** (`src/server.rs`)
   - HTTP server with Axum
   - Request routing and load balancing
   - Provider health monitoring

3. **Configuration** (`src/config.rs`)
   - TOML-based configuration
   - Provider-specific settings
   - Routing rules

4. **Error Handling** (`src/error.rs`)
   - Comprehensive error types
   - HTTP status code mapping
   - Provider error translation

### Request Flow

1. **Goose Databricks Provider** sends request to BitMortar
2. **BitMortar** receives Databricks-format request
3. **Routing Logic** selects appropriate backend provider
4. **Request Conversion** transforms to Goose format
5. **Goose Provider** processes the request
6. **Response Conversion** transforms back to Databricks format
7. **BitMortar** returns response to Goose

### Supported Providers

BitMortar supports all providers that Goose supports:
- **OpenAI**: GPT-4o, GPT-4o-mini, O1, O3
- **Anthropic**: Claude 3.5 Sonnet, Claude 3.5 Haiku, Claude 4
- **Databricks**: Any model served through Databricks
- **Groq**: Fast Llama, Mixtral inference
- **XAI**: Grok models
- And more...

## Usage Examples

### 1. Basic Setup
```bash
# Start BitMortar
cd crates/bitmortar
cargo run -- --config bitmortar.toml --port 8080

# Configure Goose to use BitMortar
export DATABRICKS_HOST="http://localhost:8080"
export DATABRICKS_TOKEN="any-value"

# Use Goose with any model through Databricks provider
goose session start --provider databricks --model gpt-4o
```

### 2. Multi-Provider Configuration
```toml
[providers.openai]
provider_type = "openai"
enabled = true
priority = 100

[providers.anthropic]
provider_type = "anthropic"
enabled = true
priority = 90

[routing.model_routes.gpt-4o]
provider = "openai"

[routing.model_routes."claude-3-5-sonnet-latest"]
provider = "anthropic"
fallbacks = ["openai"]
```

### 3. Load Balancing
```toml
[routing]
default_provider = "openai"
load_balancing = "RoundRobin"  # Distribute load across providers
```

## Benefits

### For Goose Users
- **Unified Interface**: Use Databricks provider syntax to access any backend
- **Flexibility**: Switch providers without changing Goose configuration
- **Reliability**: Automatic fallbacks and health checking
- **Cost Optimization**: Route expensive models to cheaper providers

### For Developers
- **Code Reuse**: Leverages existing Goose provider implementations
- **Maintainability**: Single point of configuration for multiple providers
- **Extensibility**: Easy to add new providers as Goose adds them
- **Testing**: Mock different providers for testing scenarios

## Configuration Examples

### Simple OpenAI Proxy
```toml
[providers.openai]
provider_type = "openai"
enabled = true

[providers.openai.config]
OPENAI_API_KEY = "sk-..."
```

### Multi-Provider with Routing
```toml
[providers.openai]
provider_type = "openai"
enabled = true
priority = 100

[providers.anthropic]
provider_type = "anthropic"
enabled = true
priority = 90

[routing.model_routes.gpt-4o]
provider = "openai"

[routing.model_routes.claude-3-5-sonnet-latest]
provider = "anthropic"
fallbacks = ["openai"]
```

### Databricks Passthrough
```toml
[providers.databricks]
provider_type = "databricks"
enabled = true

[providers.databricks.config]
DATABRICKS_HOST = "https://your-workspace.cloud.databricks.com"
DATABRICKS_TOKEN = "dapi-..."
```

## Future Enhancements

1. **Streaming Support**: Add streaming response support
2. **Metrics**: Provider usage metrics and monitoring
3. **Caching**: Response caching for identical requests
4. **Rate Limiting**: Per-provider rate limiting
5. **Authentication**: API key validation and user management

## Conclusion

BitMortar provides a clean, efficient way to create a unified LLM provider gateway that:
- Reuses battle-tested Goose provider code
- Provides Databricks-compatible API
- Enables flexible provider routing and load balancing
- Maintains consistency with Goose's provider ecosystem

This approach allows the Databricks provider in Goose to effectively become a "universal provider" that can route to any backend while maintaining a consistent interface.
