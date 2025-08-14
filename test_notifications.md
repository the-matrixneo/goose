# Testing Thinking Indicators and System Alerts

## Test Instructions

### CLI Testing

1. Start goose CLI:
   ```bash
   cd /Users/dkatz/git/goose2/goose
   cargo run --bin goose
   ```

2. Run a command that triggers auto-compaction (e.g., with many messages)

3. You should see:
   - System alerts appearing as yellow text messages
   - Thinking indicator updating with custom messages like "Compacting conversation history..."
   - Thinking indicator returning after system alerts

### Desktop Testing

1. Start the desktop app:
   ```bash
   cd /Users/dkatz/git/goose2/goose
   just run-ui
   ```

2. Start a conversation and trigger auto-compaction

3. You should see:
   - System alerts appearing as notification cards in the top-right corner
   - Thinking message updating in the LoadingGoose component at the bottom
   - Different colored alerts based on level (info=blue, warning=yellow, error=red, success=green)

## What Was Fixed

### CLI
- Fixed `ThinkingUpdate` events to re-show the thinking indicator if it was hidden by a `SystemAlert`
- System alerts now properly display without permanently hiding the thinking indicator

### Desktop
- Added `systemAlerts` and `thinkingMessage` state management to `useMessageStream` hook
- Exposed these states through the hook interface
- Updated `BaseChat` to display system alerts as notification cards
- Updated `LoadingGoose` to use the custom thinking message from the backend

## Implementation Details

### Backend (Rust)
- `AgentEvent::SystemAlert` - For transient notifications (info, warning, error, success)
- `AgentEvent::ThinkingUpdate` - For updating the loading/thinking message

### Frontend (TypeScript)
- `useMessageStream` hook now tracks `systemAlerts` array and `thinkingMessage` string
- `BaseChat` renders system alerts as floating notification cards
- `LoadingGoose` uses the `thinkingMessage` prop when available

The notifications are transient and don't get persisted to the session history, keeping the conversation clean while still providing important feedback to users.
