# Tunnel Access - Extension Loading Issue

## Problem
When accessing goosed remotely through the tunnel, tools (like file editing, shell commands) don't work because extensions aren't being loaded into the session. The same goosed server instance is being accessed, but remote clients don't have the extension initialization logic that the Electron app performs.

## Root Cause
The goosed server uses a two-layer extension system:

1. **Config Layer** (`/config/extensions`) - Persistent storage in `~/.config/goose/config.yaml`
   - Stores what extensions are available/configured
   - Maintains enabled/disabled state
   - Persists across sessions

2. **Session Layer** (`/extensions/add`, `/extensions/remove`) - Runtime for specific agent sessions
   - Controls which extensions are actually loaded in a session
   - Must be explicitly added each time a session starts
   - Extensions in config.yaml are NOT automatically loaded

## How the Desktop App Works

### Initial Bootstrap (First-time setup)
1. **Check existing config**: `GET /config/extensions`
2. **If empty, populate from bundled-extensions.json**: 
   - For each built-in extension, call `POST /config/extensions`
   - This saves them to `~/.config/goose/config.yaml`
3. **Sync bundled extensions**: Ensures any new built-ins are added

### Session Initialization (Every new session)
1. **Start agent session**: `POST /agent/start` with working_dir and optional recipe
2. **Update provider and model**: `POST /agent/update_provider` with provider and model settings
3. **Extend system prompt**: `POST /agent/extend_prompt` with desktop-specific instructions
4. **Get enabled extensions from config**: `GET /config/extensions`
5. **Add each enabled extension to the session**:
   ```json
   POST /extensions/add
   {
     "type": "builtin",
     "name": "developer",
     "session_id": "your-session-id"
   }
   ```

## Built-in Extensions
From `ui/desktop/src/built-in-extensions.json`:
- **developer** (enabled by default) - File editing, shell commands, code analysis
- **computercontroller** (disabled) - General computer control
- **autovisualiser** (disabled) - Data visualization and UI generation
- **memory** (disabled) - User preference learning
- **tutorial** (disabled) - Interactive tutorials

## Solution for Remote Clients

Remote clients need to replicate the desktop app's initialization sequence:

### Step 1: Check/Initialize Config
```bash
# Check existing extensions
curl -X GET http://localhost:PORT/config/extensions \
  -H "X-Secret-Key: YOUR_SECRET"

# If empty, add built-in extensions (example for developer extension)
curl -X POST http://localhost:PORT/config/extensions \
  -H "Content-Type: application/json" \
  -H "X-Secret-Key: YOUR_SECRET" \
  -d '{
    "name": "developer",
    "config": {
      "type": "builtin",
      "name": "developer",
      "display_name": "Developer",
      "timeout": 300,
      "bundled": true
    },
    "enabled": true
  }'
```

### Step 2: Start Session
```bash
curl -X POST http://localhost:PORT/agent/start \
  -H "Content-Type: application/json" \
  -H "X-Secret-Key: YOUR_SECRET" \
  -d '{
    "working_dir": "/path/to/working/dir"
  }'
# Returns: {"session_id": "xxx", ...}
```

### Step 3: Configure Provider and Model
```bash
curl -X POST http://localhost:PORT/agent/update_provider \
  -H "Content-Type: application/json" \
  -H "X-Secret-Key: YOUR_SECRET" \
  -d '{
    "session_id": "SESSION_ID_FROM_STEP_2",
    "provider": "openai",
    "model": "gpt-4o"
  }'
```

### Step 4: Extend System Prompt (Optional)
```bash
curl -X POST http://localhost:PORT/agent/extend_prompt \
  -H "Content-Type: application/json" \
  -H "X-Secret-Key: YOUR_SECRET" \
  -d '{
    "session_id": "SESSION_ID_FROM_STEP_2",
    "extension": "You are being accessed through a remote client..."
  }'
```

### Step 5: Add Extensions to Session
```bash
# For each enabled extension from config
curl -X POST http://localhost:PORT/extensions/add \
  -H "Content-Type: application/json" \
  -H "X-Secret-Key: YOUR_SECRET" \
  -d '{
    "type": "builtin",
    "name": "developer",
    "session_id": "SESSION_ID_FROM_STEP_2"
  }'
```

## Key Points
- Extensions must be added to EACH new session
- The config.yaml is just storage - it doesn't automatically load extensions
- The desktop app orchestrates this flow client-side, including:
  - Setting the provider/model after session creation
  - Extending the system prompt
  - Loading all enabled extensions
- There's no server-side automatic extension loading currently
- Provider and model MUST be configured via `/agent/update_provider` after starting the session

## Potential Improvements
1. **Quick fix**: Document this process for remote clients
2. **Better fix**: Add a query parameter to `/agent/start` to auto-load enabled extensions
3. **Best fix**: Make goosed automatically load enabled extensions from config.yaml when starting sessions (unless explicitly disabled)
