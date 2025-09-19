# Comprehensive Report: Goose Scheduler System

## Executive Summary

The Goose scheduler system is a sophisticated component that enables autonomous execution of AI agent tasks outside the main agent loop. It provides two scheduler implementations (Legacy and Temporal), supports cron-based scheduling, manages recipe execution, and maintains complete isolation between scheduled jobs and the main application. This report provides an in-depth analysis of both the theoretical design and practical implementation.

## Table of Contents

1. [Architecture Overview](#architecture-overview)
2. [Core Components](#core-components)
3. [Scheduler Implementations](#scheduler-implementations)
4. [Recipe System Integration](#recipe-system-integration)
5. [Agent Lifecycle Management](#agent-lifecycle-management)
6. [Job Execution Flow](#job-execution-flow)
7. [State Management](#state-management)
8. [Concurrency Model](#concurrency-model)
9. [Storage and Persistence](#storage-and-persistence)
10. [Error Handling](#error-handling)
11. [Key Design Patterns](#key-design-patterns)
12. [Future Considerations](#future-considerations)

---

## Architecture Overview

The Goose scheduler system follows a modular, trait-based architecture that allows for multiple scheduler implementations while maintaining a consistent interface. The system is designed to:

1. **Run agents independently**: Execute recipe-based agent tasks without blocking the main application
2. **Support multiple scheduler backends**: Currently supports Legacy (tokio-cron-scheduler) and Temporal
3. **Maintain job isolation**: Each scheduled job runs in its own context with its own agent instance
4. **Provide comprehensive job management**: Create, pause, resume, update, and kill scheduled jobs

### High-Level Architecture Diagram

```
┌─────────────────────────────────────────────────────────┐
│                    Goose Application                     │
├─────────────────────────────────────────────────────────┤
│                                                          │
│  ┌──────────────────┐        ┌────────────────────┐    │
│  │  Main Agent Loop │        │  Scheduler System  │    │
│  │                  │        │                    │    │
│  │  - User Sessions │        │  - Scheduled Jobs  │    │
│  │  - Interactive   │        │  - Autonomous      │    │
│  │    Commands      │        │    Execution       │    │
│  └──────────────────┘        └────────────────────┘    │
│           │                           │                 │
│           └───────────┬───────────────┘                 │
│                       │                                 │
│              ┌────────▼────────┐                       │
│              │  Agent Factory  │                       │
│              │                 │                       │
│              │ Creates isolated│                       │
│              │ agent instances │                       │
│              └─────────────────┘                       │
│                                                          │
└─────────────────────────────────────────────────────────┘
```

---

## Core Components

### 1. SchedulerTrait (`scheduler_trait.rs`)

The `SchedulerTrait` defines the common interface that all scheduler implementations must provide:

```rust
#[async_trait]
pub trait SchedulerTrait: Send + Sync {
    async fn add_scheduled_job(&self, job: ScheduledJob) -> Result<(), SchedulerError>;
    async fn list_scheduled_jobs(&self) -> Result<Vec<ScheduledJob>, SchedulerError>;
    async fn remove_scheduled_job(&self, id: &str) -> Result<(), SchedulerError>;
    async fn pause_schedule(&self, id: &str) -> Result<(), SchedulerError>;
    async fn unpause_schedule(&self, id: &str) -> Result<(), SchedulerError>;
    async fn run_now(&self, id: &str) -> Result<String, SchedulerError>;
    async fn sessions(&self, sched_id: &str, limit: usize) 
        -> Result<Vec<(String, SessionMetadata)>, SchedulerError>;
    async fn update_schedule(&self, sched_id: &str, new_cron: String) 
        -> Result<(), SchedulerError>;
    async fn kill_running_job(&self, sched_id: &str) -> Result<(), SchedulerError>;
    async fn get_running_job_info(&self, sched_id: &str) 
        -> Result<Option<(String, DateTime<Utc>)>, SchedulerError>;
}
```

This trait ensures consistency across different scheduler backends and makes the system extensible.

### 2. ScheduledJob Structure

```rust
pub struct ScheduledJob {
    pub id: String,                              // Unique identifier
    pub source: String,                          // Path to recipe file
    pub cron: String,                           // Cron expression
    pub last_run: Option<DateTime<Utc>>,        // Last execution time
    pub currently_running: bool,                 // Execution status
    pub paused: bool,                           // Pause status
    pub current_session_id: Option<String>,     // Active session ID
    pub process_start_time: Option<DateTime<Utc>>, // Process start time
    pub execution_mode: Option<String>,         // "foreground" or "background"
}
```

### 3. SchedulerFactory (`scheduler_factory.rs`)

The factory pattern implementation that creates the appropriate scheduler based on configuration:

```rust
impl SchedulerFactory {
    pub async fn create(storage_path: PathBuf) 
        -> Result<Arc<dyn SchedulerTrait>, SchedulerError> {
        let scheduler_type = SchedulerType::from_config();
        
        match scheduler_type {
            SchedulerType::Legacy => {
                let scheduler = Scheduler::new(storage_path).await?;
                Ok(scheduler as Arc<dyn SchedulerTrait>)
            }
            SchedulerType::Temporal => {
                match TemporalScheduler::new().await {
                    Ok(scheduler) => Ok(scheduler as Arc<dyn SchedulerTrait>),
                    Err(_) => {
                        // Fallback to legacy if Temporal unavailable
                        let scheduler = Scheduler::new(storage_path).await?;
                        Ok(scheduler as Arc<dyn SchedulerTrait>)
                    }
                }
            }
        }
    }
}
```

---

## Scheduler Implementations

### Legacy Scheduler (`scheduler.rs`)

The default scheduler implementation using `tokio-cron-scheduler`:

**Key Features:**
- In-process scheduling using Rust's async runtime
- Persistent job storage in JSON format
- Abort handle tracking for job cancellation
- Mutex-based state management

**Architecture:**
```rust
pub struct Scheduler {
    internal_scheduler: TokioJobScheduler,      // Tokio cron scheduler
    jobs: Arc<Mutex<JobsMap>>,                 // Job registry
    storage_path: PathBuf,                     // Persistence path
    running_tasks: Arc<Mutex<RunningTasksMap>>, // Abort handles
}
```

**Job Execution:**
- Each job spawns as an abortable Tokio task
- Jobs are wrapped in closures that check pause status before execution
- Abort handles stored for graceful cancellation

### Temporal Scheduler (`temporal_scheduler.rs`)

Advanced scheduler using Temporal workflow engine:

**Key Features:**
- External Go service for scheduling
- HTTP API communication
- Dynamic port discovery
- Automatic service management
- Better reliability and scalability

**Architecture:**
```rust
pub struct TemporalScheduler {
    http_client: Client,        // HTTP client for API calls
    service_url: String,        // Service endpoint
    port_config: PortConfig,    // Port configuration
}
```

**Service Management:**
- Automatic Go service startup
- Health checking and readiness probes
- Port discovery (checks environment and default ports)
- Graceful fallback to Legacy scheduler if unavailable

---

## Recipe System Integration

### Recipe Structure

Recipes define the behavior and configuration for scheduled jobs:

```rust
pub struct Recipe {
    pub version: String,
    pub title: String,
    pub description: String,
    pub instructions: Option<String>,    // System instructions
    pub prompt: Option<String>,          // Initial prompt
    pub extensions: Option<Vec<ExtensionConfig>>, // Required extensions
    pub context: Option<Vec<String>>,    // Additional context
    pub settings: Option<Settings>,      // Model settings
    pub activities: Option<Vec<String>>, // Activity indicators
    pub author: Option<Author>,
    pub parameters: Option<Vec<RecipeParameter>>,
    pub response: Option<Response>,      // Response schema
    pub sub_recipes: Option<Vec<SubRecipe>>,
    pub retry: Option<RetryConfig>,
}
```

### Recipe Loading Process

1. **File Discovery**: Recipe files stored in `scheduled_recipes` directory
2. **Format Detection**: Supports JSON and YAML formats
3. **Parsing**: Deserializes into Recipe struct
4. **Validation**: Ensures required fields are present
5. **Extension Loading**: Configures required extensions for the agent

---

## Agent Lifecycle Management

### The `run_scheduled_job_internal` Function

This is the core function that manages agent lifecycle for scheduled jobs:

```rust
async fn run_scheduled_job_internal(
    job: ScheduledJob,
    provider_override: Option<Arc<dyn GooseProvider>>,
    jobs_arc: Option<Arc<Mutex<JobsMap>>>,
    job_id: Option<String>,
) -> Result<String, JobExecutionError>
```

### Lifecycle Phases

#### 1. **Initialization Phase**
```rust
// Load recipe from file
let recipe_content = fs::read_to_string(recipe_path)?;
let recipe: Recipe = parse_recipe(recipe_content)?;

// Create new agent instance
let agent = Agent::new();
```

#### 2. **Configuration Phase**
```rust
// Configure provider (LLM backend)
let provider = if let Some(override) = provider_override {
    override
} else {
    create_provider_from_config()?
};

// Add extensions from recipe
for extension in recipe.extensions {
    agent.add_extension(extension).await?;
}

// Set provider on agent
agent.update_provider(provider).await?;
```

#### 3. **Execution Phase**
```rust
// Create session configuration
let session_config = SessionConfig {
    id: Identifier::Name(session_id),
    working_dir: current_dir,
    schedule_id: Some(job.id.clone()),
    execution_mode: job.execution_mode,
    max_turns: None,
    retry_config: None,
};

// Execute agent with recipe prompt
let mut stream = agent.reply(
    Conversation::new(vec![Message::user().with_text(recipe.prompt)]),
    Some(session_config),
    None
).await?;

// Process agent responses
while let Some(event) = stream.next().await {
    match event {
        Ok(AgentEvent::Message(msg)) => {
            all_session_messages.push(msg);
        }
        // Handle other events...
    }
}
```

#### 4. **Persistence Phase**
```rust
// Save session to storage
let metadata = SessionMetadata {
    working_dir: current_dir,
    schedule_id: Some(job.id),
    message_count: all_session_messages.len(),
    // ... other metadata
};

session::storage::save_messages_with_metadata(
    &session_file_path,
    &metadata,
    &all_session_messages
)?;
```

### Agent Isolation

Each scheduled job gets its own:
- **Agent Instance**: Fresh `Agent::new()` for each execution
- **Provider Instance**: Separate LLM provider connection
- **Extension Manager**: Independent extension configuration
- **Session Storage**: Unique session file with metadata
- **Execution Context**: Isolated working directory and environment

---

## Job Execution Flow

### Cron-Based Triggering

1. **Cron Expression Normalization**
   ```rust
   pub fn normalize_cron_expression(src: &str) -> String {
       // Converts 5-field, 6-field, or 7-field cron to standard format
       // 5-field: min hour dom mon dow → 0 min hour dom mon dow *
       // 6-field: sec min hour dom mon dow → sec min hour dom mon dow *
       // 7-field: Already in quartz format
   }
   ```

2. **Job Creation**
   ```rust
   let cron_task = Job::new_async(&cron_expr, move |_uuid, _l| {
       Box::pin(async move {
           // Check if paused
           if !should_execute { return; }
           
           // Update job status
           job.currently_running = true;
           job.last_run = Some(Utc::now());
           
           // Spawn execution task
           let task = tokio::spawn(run_scheduled_job_internal(...));
           
           // Store abort handle
           running_tasks.insert(job_id, task.abort_handle());
           
           // Wait for completion
           let result = task.await;
           
           // Clean up
           running_tasks.remove(&job_id);
           job.currently_running = false;
       })
   });
   ```

### Execution Modes

**Background Mode** (Default):
- Runs without user interaction
- Automatic tool approval
- No confirmation prompts
- Suitable for autonomous tasks

**Foreground Mode**:
- Simulates interactive session
- May skip certain tool calls
- Respects chat mode restrictions
- Used for user-visible executions

---

## State Management

### Job State Tracking

The scheduler maintains comprehensive state for each job:

```rust
type JobsMap = HashMap<String, (JobId, ScheduledJob)>;
type RunningTasksMap = HashMap<String, tokio::task::AbortHandle>;
```

**State Transitions:**

```
Created → Scheduled → Running → Completed
           ↓     ↑       ↓
         Paused ←────────┘
```

### Persistence Strategy

**Legacy Scheduler:**
- JSON file storage (`schedules.json`)
- Atomic writes with pretty printing
- Loaded on startup, persisted on changes

**Temporal Scheduler:**
- State managed by Temporal service
- HTTP API for state queries
- Session-based status monitoring

### Session Management

Each job execution creates a session:

```rust
pub struct SessionMetadata {
    pub working_dir: PathBuf,
    pub description: String,
    pub schedule_id: Option<String>,  // Links to scheduled job
    pub message_count: usize,
    pub total_tokens: Option<u64>,
    // ... token usage metrics
    pub todo_content: Option<String>,
}
```

Sessions are stored as JSONL files with:
- Metadata header
- Message history
- Tool call results
- Agent responses

---

## Concurrency Model

### Task Spawning

The scheduler uses Tokio's async runtime for concurrency:

1. **Main Scheduler Task**: Manages cron triggers
2. **Job Execution Tasks**: Spawned for each job run
3. **Abort Handles**: Enable graceful cancellation

```rust
// Spawn with abort handle tracking
let job_task = tokio::spawn(run_scheduled_job_internal(...));
let abort_handle = job_task.abort_handle();
running_tasks.insert(job_id, abort_handle);
```

### Synchronization Primitives

**Mutexes:**
- `Arc<Mutex<JobsMap>>`: Shared job registry
- `Arc<Mutex<RunningTasksMap>>`: Abort handle registry

**Lock Ordering:**
- Always acquire job lock before running tasks lock
- Release locks before I/O operations
- Use guards with limited scope

### Cancellation Support

Jobs can be cancelled via:
1. **Kill Command**: Aborts running task
2. **Pause**: Prevents future executions
3. **Remove**: Deletes job and cancels if running

```rust
pub async fn kill_running_job(&self, sched_id: &str) -> Result<(), SchedulerError> {
    // Get abort handle
    let abort_handle = running_tasks.remove(sched_id);
    
    // Abort the task
    abort_handle.abort();
    
    // Update job state
    job.currently_running = false;
    job.current_session_id = None;
}
```

---

## Storage and Persistence

### File System Layout

```
~/.config/goose/           # Or platform-specific config dir
├── schedules.json         # Job definitions (Legacy)
├── scheduled_recipes/     # Recipe file copies
│   ├── job1.yaml
│   ├── job2.json
│   └── ...
└── sessions/             # Execution history
    ├── 2024-01-01-120000-abc123.jsonl
    ├── 2024-01-01-130000-def456.jsonl
    └── ...
```

### Recipe File Management

When a job is scheduled:
1. Original recipe copied to `scheduled_recipes/`
2. Copy named with job ID
3. Original can be modified without affecting scheduled job
4. Deleted when job is removed

### Session Storage

Sessions use JSONL format:
```jsonl
{"metadata": {...}}
{"role": "user", "content": [...]}
{"role": "assistant", "content": [...]}
{"role": "user", "content": [...], "tool_calls": [...]}
```

Benefits:
- Streaming-friendly format
- Partial read capability
- Corruption resistance
- Easy to parse and debug

---

## Error Handling

### Error Types

```rust
pub enum SchedulerError {
    JobIdExists(String),           // Duplicate job ID
    JobNotFound(String),          // Job doesn't exist
    StorageError(io::Error),      // File I/O errors
    RecipeLoadError(String),      // Recipe parsing failed
    AgentSetupError(String),      // Agent configuration failed
    PersistError(String),         // Save operation failed
    CronParseError(String),       // Invalid cron expression
    SchedulerInternalError(String), // Internal scheduler error
    AnyhowError(anyhow::Error),  // Generic errors
}
```

### Error Recovery Strategies

1. **Recipe Load Failures**: Skip job, log warning
2. **Provider Errors**: Retry with backoff
3. **Storage Errors**: Attempt recovery from backup
4. **Agent Crashes**: Mark job as failed, continue scheduler
5. **Temporal Service Down**: Fallback to Legacy scheduler

### Logging and Diagnostics

Comprehensive logging at multiple levels:
```rust
tracing::info!("Executing job: {} (Source: {})", job.id, job.source);
tracing::warn!("Recipe file {} not found. Skipping job load.", path);
tracing::error!("Failed to persist job state: {}", error);
```

---

## Key Design Patterns

### 1. **Factory Pattern**
- `SchedulerFactory` creates appropriate scheduler implementation
- Enables runtime selection based on configuration
- Provides fallback mechanism

### 2. **Strategy Pattern**
- `SchedulerTrait` defines common interface
- Multiple implementations (Legacy, Temporal)
- Runtime strategy selection

### 3. **Builder Pattern**
- `Recipe::builder()` for recipe construction
- `Agent` configuration through method chaining
- Session configuration builders

### 4. **Observer Pattern**
- Event streaming from agent execution
- Notification system for MCP events
- Status monitoring for running jobs

### 5. **Repository Pattern**
- Session storage abstraction
- Recipe file management
- Job persistence layer

### 6. **Dependency Injection**
- Provider override for testing
- Extension configuration injection
- Scheduler service injection into agents

---

## Advanced Features

### 1. **Dynamic Port Discovery** (Temporal)
```rust
async fn discover_http_port(http_client: &Client) -> Result<u16, SchedulerError> {
    // Check PORT environment variable
    // Try default ports
    // Find free port if needed
}
```

### 2. **Health Monitoring**
```rust
async fn update_job_status_from_sessions(&self) -> Result<(), SchedulerError> {
    // Check session file modification times
    // Detect stale jobs
    // Update status accordingly
}
```

### 3. **Graceful Degradation**
- Temporal unavailable → Use Legacy
- Recipe corrupted → Skip job
- Provider down → Retry later

### 4. **Test Support**
```rust
#[cfg(test)]
pub(super) fn create_scheduler_test_mock_provider(
    model_config: ModelConfig
) -> Arc<dyn GooseProvider> {
    // Mock provider for testing
}
```

---

## Performance Considerations

### Memory Management
- Jobs loaded lazily on startup
- Sessions streamed, not loaded entirely
- Abort handles cleaned up after execution
- Mutex guards released quickly

### I/O Optimization
- Batch persistence operations
- Async file operations
- JSONL for streaming writes
- Minimal file system operations

### Scalability
- Legacy: Limited by single process
- Temporal: Horizontally scalable
- Session storage: Partitionable by date
- Recipe storage: Content-addressed possible

---

## Security Considerations

### Job Isolation
- Separate agent instances
- No shared state between jobs
- Independent provider connections
- Isolated session storage

### Recipe Validation
- Schema validation on load
- Extension permission checks
- Prompt injection prevention
- Unicode tag detection

### Access Control
- File system permissions
- Recipe directory protection
- Session privacy
- Configuration security

---

## Future Considerations

### Potential Enhancements

1. **Distributed Scheduling**
   - Redis-based job queue
   - Kubernetes CronJob integration
   - Cloud scheduler backends

2. **Advanced Scheduling**
   - Event-based triggers
   - Dependency chains
   - Conditional execution
   - Rate limiting

3. **Monitoring & Observability**
   - Prometheus metrics
   - OpenTelemetry tracing
   - Job execution dashboards
   - Alert mechanisms

4. **Recipe Management**
   - Version control integration
   - Recipe marketplace
   - Template system
   - Parameter validation

5. **Performance Optimizations**
   - Job pooling
   - Provider connection reuse
   - Session compression
   - Incremental persistence

6. **Enhanced Error Recovery**
   - Automatic retries
   - Dead letter queues
   - Failure analysis
   - Self-healing mechanisms

---

## Conclusion

The Goose scheduler system represents a sophisticated approach to autonomous agent execution. Its dual-implementation strategy (Legacy and Temporal) provides both simplicity and scalability. The system's strength lies in:

1. **Complete Agent Isolation**: Each job runs in its own context
2. **Flexible Architecture**: Trait-based design enables extensibility
3. **Robust State Management**: Comprehensive tracking and persistence
4. **Graceful Error Handling**: Multiple recovery strategies
5. **Production Readiness**: Health monitoring, logging, and diagnostics

The scheduler successfully decouples long-running agent tasks from the main application loop, enabling true autonomous operation while maintaining system stability and user control. The architecture provides a solid foundation for future enhancements while remaining maintainable and testable.

### Key Takeaways

- **Agents run outside the main loop** through spawned Tokio tasks or Temporal workflows
- **Each job gets a fresh agent instance** ensuring complete isolation
- **Recipes define agent behavior** including prompts, extensions, and settings
- **State is carefully managed** through persistent storage and in-memory tracking
- **The system is extensible** through trait-based design and factory patterns
- **Error handling is comprehensive** with fallbacks and recovery mechanisms
- **Concurrency is well-managed** using async/await and proper synchronization

This architecture enables Goose to run scheduled AI tasks reliably and autonomously, making it suitable for production workloads requiring scheduled agent execution.

---

*Report compiled: 2025-08-27*
*Based on source code analysis of the Goose project*
