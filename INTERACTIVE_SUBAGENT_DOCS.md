# Interactive Subagent Implementation

## Overview

This implementation adds interactive subagent functionality to Goose, allowing the main agent to spawn specialized subagents based on recipes for complex, multi-turn conversations.

## Key Features

### 1. **Simple Recipe Calls** (`platform__call_recipe`)
- Single-turn recipe execution
- Direct provider calls for simple tasks
- Existing functionality (already implemented)

### 2. **Interactive Subagents** (`platform__spawn_interactive_subagent`)
- Multi-turn conversations with specialized agents
- Recipe-based configuration
- Configurable maximum turns (1-10)
- Automatic termination detection

## Usage Examples

### Basic Interactive Subagent Call

```json
{
  "name": "platform__spawn_interactive_subagent",
  "arguments": {
    "recipe_name": "research_assistant_recipe.yaml",
    "message": "I need to research the latest developments in AI safety",
    "max_turns": 5
  }
}
```

### Code Review Subagent

```json
{
  "name": "platform__spawn_interactive_subagent", 
  "arguments": {
    "recipe_name": "code_review_recipe.yaml",
    "message": "Please review this Python function for security issues and best practices: [code here]",
    "max_turns": 3
  }
}
```

## Recipe Configuration

### Required Fields
- `version`: Recipe format version
- `title`: Human-readable title
- `description`: Brief description of the recipe's purpose
- `instructions`: Detailed system prompt for the subagent
- `activities`: List of activities the subagent can perform

### Optional Fields
- `extensions`: List of extensions to enable for the subagent
- `parameters`: Key-value parameters for recipe customization

### Example Recipe Structure

```yaml
version: "1.0.0"
title: "Specialized Assistant"
description: "A specialized assistant for specific tasks"
instructions: |
  You are a specialized assistant. Your role is to:
  1. Understand the user's request
  2. Provide detailed analysis
  3. Always end with "Task complete" when finished
activities:
  - "Analyze requests"
  - "Provide detailed responses"
extensions:
  - type: stdio
    name: websearch
    display_name: Web Search
    description: Search the web
    args: [mcp_websearch@latest]
    bundled: true
    cmd: uvx
    timeout: 300
```

## Implementation Details

### Message Flow
1. **User Request**: Main agent receives request to spawn subagent
2. **Subagent Creation**: New agent instance created with recipe configuration
3. **Provider Sharing**: Subagent shares the same LLM provider as main agent
4. **Conversation Loop**: Subagent processes messages until termination
5. **Response Formatting**: Conversation formatted and returned to main agent

### Termination Conditions
The subagent conversation terminates when:
- Maximum turns reached
- Termination phrases detected ("task complete", "finished", "done", etc.)
- No tool calls and substantial content (>50 chars)
- No assistant response received

### Thread Safety
- Uses direct provider calls to avoid `Send` trait issues
- Isolated agent instances prevent state conflicts
- Proper async/await handling throughout

## Architecture Benefits

### 1. **Separation of Concerns**
- Main agent handles orchestration
- Subagents focus on specialized tasks
- Clear boundaries between responsibilities

### 2. **Reusable Recipes**
- Recipe-based configuration allows reuse
- Easy to create new specialized agents
- Consistent behavior across contexts

### 3. **Scalable Design**
- Multiple subagents can be spawned
- Each operates independently
- Resource sharing through provider cloning

## Future Enhancements

### 1. **Tool Call Support**
- Enable subagents to make tool calls
- Handle tool responses in conversation flow
- Support for complex multi-step workflows

### 2. **Streaming Integration**
- Real-time subagent responses
- Progressive conversation updates
- Better user experience for long tasks

### 3. **State Management**
- Persistent subagent sessions
- Cross-conversation memory
- Context sharing between subagents

## Error Handling

### Common Issues
- **Recipe Not Found**: Clear error message with suggestions
- **Provider Unavailable**: Graceful fallback handling
- **Configuration Errors**: Detailed validation messages
- **Timeout Handling**: Configurable timeouts for long operations

### Best Practices
- Always validate input parameters
- Provide meaningful error messages
- Log important events for debugging
- Handle edge cases gracefully

## Testing

### Unit Tests
- Recipe loading and validation
- Message formatting and parsing
- Termination condition detection
- Error handling scenarios

### Integration Tests
- End-to-end subagent conversations
- Provider integration testing
- Recipe configuration validation
- Performance under load

This implementation provides a solid foundation for interactive subagents while maintaining compatibility with the existing Goose architecture.