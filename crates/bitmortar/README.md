# BitMortar

BitMortar is a unified API server that provides Databricks-compatible endpoints while routing requests to various LLM providers behind the scenes. It acts as a proxy/gateway that allows the Databricks provider in Goose to connect to any supported provider through a consistent API.

## Features

- **Databricks-Compatible API**: Provides endpoints that match the Databricks serving endpoints format
- **Multi-Provider Support**: Routes to OpenAI, Anthropic, Databricks, Groq, XAI, and other Goose-supported providers
- **Intelligent Routing**: Route specific models to specific providers with fallback support
- **Load Balancing**: Multiple load balancing strategies (Priority, Round Robin, Random, First Available)
- **Health Checking**: Automatic health monitoring of backend providers
- **Configuration-Driven**: Easy TOML-based configuration

## Architecture

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Databricks    │    │                 │    │     OpenAI      │
│   Provider      │───▶│   BitMortar     │───▶│   Provider      │
│   (in Goose)    │    │   Server        │    │                 │
└─────────────────┘    │                 │    └─────────────────┘
                       │                 │    ┌─────────────────┐
                       │                 │───▶│   Anthropic     │
                       │                 │    │   Provider      │
                       │                 │    └─────────────────┘
                       │                 │    ┌─────────────────┐
                       │                 │───▶│   Other Goose   │
                       │                 │    │   Providers     │
                       └─────────────────┘    └─────────────────┘
```

## Quick Start

1. **Build BitMortar**:
   ```bash
   cd crates/bitmortar
   cargo build --release
   ```

2. **Create Configuration**:
   ```bash
   cp bitmortar.toml.example bitmortar.toml
   # Edit bitmortar.toml with your API keys and settings
   ```

3. **Run BitMortar**:
   ```bash
   cargo run -- --config bitmortar.toml --port 8080
   ```

4. **Configure Databricks Provider in Goose**:
   ```bash
   # Set environment variables to point to BitMortar
   export DATABRICKS_HOST="http://localhost:8080"
   export DATABRICKS_TOKEN="any-token"  # BitMortar doesn't validate this
   ```

## API Endpoints

BitMortar provides the following Databricks-compatible endpoints:

### Chat Completions
```
POST /serving-endpoints/{model}/invocations
```

Example:
```bash
curl -X POST "http://localhost:8080/serving-endpoints/gpt-4o/invocations" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer any-token" \
  -d '{
    "messages": [
      {"role": "user", "content": "Hello, world!"}
    ],
    "max_tokens": 100
  }'
```

### Embeddings
```
POST /v1/embeddings
```

Example:
```bash
curl -X POST "http://localhost:8080/v1/embeddings" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer any-token" \
  -d '{
    "input": ["Hello, world!", "How are you?"],
    "model": "text-embedding-3-small"
  }'
```

### List Models
```
GET /v1/models
```

### List Endpoints (Databricks-compatible)
```
GET /serving-endpoints
```

### Health Check
```
GET /health
```

## Configuration

The configuration file (`bitmortar.toml`) has several sections:

### Server Configuration
```toml
[server]
host = "0.0.0.0"
port = 8080
timeout = 600
```

### Routing Configuration
```toml
[routing]
default_provider = "openai"
load_balancing = "Priority"  # Priority, RoundRobin, Random, FirstAvailable

# Route specific models to specific providers
[routing.model_routes.gpt-4o]
provider = "openai"
fallbacks = []

[routing.model_routes."claude-3-5-sonnet-latest"]
provider = "anthropic"
fallbacks = ["openai"]  # Fallback to OpenAI if Anthropic fails
```

### Provider Configuration
```toml
[providers.openai]
provider_type = "openai"
enabled = true
priority = 100

[providers.openai.config]
OPENAI_API_KEY = "your-api-key"
OPENAI_HOST = "https://api.openai.com"
```

## Supported Providers

BitMortar supports all providers that Goose supports:

- **OpenAI**: GPT-4o, GPT-4o-mini, O1, O3, etc.
- **Anthropic**: Claude 3.5 Sonnet, Claude 3.5 Haiku, Claude 4, etc.
- **Databricks**: Any model served through Databricks Model Serving
- **Groq**: Fast inference for Llama, Mixtral, etc.
- **XAI**: Grok models
- **Azure OpenAI**: Azure-hosted OpenAI models
- **Google**: Gemini models
- **And more**: Any provider supported by Goose

## Use Cases

### 1. Databricks Provider Proxy
Use BitMortar as a proxy for the Databricks provider in Goose, allowing it to route to any backend provider:

```bash
# Start BitMortar
cargo run -- --port 8080

# Configure Goose to use BitMortar as Databricks endpoint
export DATABRICKS_HOST="http://localhost:8080"
export DATABRICKS_TOKEN="any-token"

# Now Goose's Databricks provider will route through BitMortar
goose session start --provider databricks --model gpt-4o
```

### 2. Multi-Provider Load Balancing
Configure multiple providers with different priorities and let BitMortar balance the load:

```toml
[providers.openai]
provider_type = "openai"
enabled = true
priority = 100

[providers.anthropic]
provider_type = "anthropic"
enabled = true
priority = 90

[routing]
load_balancing = "Priority"
```

### 3. Model-Specific Routing
Route different models to their optimal providers:

```toml
[routing.model_routes.gpt-4o]
provider = "openai"

[routing.model_routes."claude-3-5-sonnet-latest"]
provider = "anthropic"

[routing.model_routes."llama-3-70b"]
provider = "groq"
```

## Development

### Building
```bash
cargo build
```

### Running Tests
```bash
cargo test
```

### Running with Debug Logging
```bash
RUST_LOG=debug cargo run -- --config bitmortar.toml
```

## Integration with Goose

BitMortar is designed to work seamlessly with Goose's Databricks provider. Here's how to set it up:

1. **Start BitMortar** with your desired provider configuration
2. **Configure Goose** to use BitMortar as the Databricks endpoint:
   ```bash
   export DATABRICKS_HOST="http://localhost:8080"
   export DATABRICKS_TOKEN="any-value"  # BitMortar doesn't validate this
   ```
3. **Use Goose** with the Databricks provider:
   ```bash
   goose session start --provider databricks --model gpt-4o
   ```

Now Goose will send requests to BitMortar, which will route them to the appropriate backend provider based on your configuration.

## License

This project is part of the Goose project and follows the same license terms.
