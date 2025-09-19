# Unify Agent Execution: per‑session agents, unified tasks/recipes/scheduler

## Why

Today we have several parallel ways to run “the same thing” (an agent doing work):

- Interactive chat in goosed/goose-server uses a single shared Agent for all sessions (see goose-server AppState holding one AgentRef).
- The scheduler (legacy and Temporal) spins up a fresh Agent per run (see `run_scheduled_job_internal` in `crates/goose/src/scheduler.rs`).
- Dynamic tasks create subagents on demand (see `subagent_handler.rs`), often via the `dynamic_task__create_task` tool.
- Sub‑recipes execute either by spawning the CLI (`goose run --recipe …`) or via subagent execution (`inline_recipe`).

This creates duplicated code paths, inconsistent behavior, and hard‑to‑debug concurrency issues:

- Extension/provider setup logic is re‑implemented in multiple places.
- Shared Agent in the server means sessions interfere with each other (shared ExtensionManager, tool monitor, channels).
- Different execution surfaces behave differently (e.g., scheduler vs chat vs sub‑recipes via CLI).
- It makes multi‑session and multi‑user support brittle.

We want one clear model that scales: agent per session, multiple simultaneous sessions, ad‑hoc dynamic tasks, and a single execution pipeline used by chat, scheduler, and recipes.

## Goals

- Agent per session in goosed/goose-server, with isolation and support for many simultaneous sessions.
- Unify execution for recipes, dynamic tasks, and scheduled jobs.
- Allow agents to create ad‑hoc dynamic tasks that run in a controlled “agent class” (subagent or inline recipe) with clear extension scoping.
- Keep existing tools usable (dynamic_task, subagent__execute_task, scheduler tools), but route them through the same backend.

## What this looks like (examples)

1) Two independent chat sessions at the same time

- Session A enables only “developer” tools and uses Model X.
- Session B enables “browser” tools and uses Model Y.
- Because each session has its own Agent, there’s no cross‑talk: enabling/disabling extensions in A doesn’t affect B.

2) Ad‑hoc fan‑out tasks from chat

- From Session A, the agent creates two dynamic tasks and runs them in parallel with limited extensions:

```json
{
  "tool": "dynamic_task__create_task",
  "arguments": {
    "task_parameters": [
      { "instructions": "Write a quick unit test for foo()", "extensions": ["developer"] },
      { "instructions": "Draft a README section summarizing today’s work", "extensions": [] }
    ],
    "execution_mode": "parallel"
  }
}
```

- Both tasks run as inline recipes under the same Session A context, using the unified executor. Results are returned and appended to Session A.

3) Scheduling that produces normal sessions

- A nightly recipe runs via the scheduler. It executes using the same pipeline as chat, just with `execution_mode=background`, and records a normal session file with `schedule_id` in metadata. The session can be inspected with the same APIs/UI as chat.

4) Sub‑recipes without CLI spawning

- A sub‑recipe reference resolves to a Recipe and is executed through the unified executor (same as dynamic inline recipes). No extra process spawn unless explicitly required.

## Proposed implementation

Introduce a unified execution layer in the goose crate and route all surfaces through it.

1) AgentManager (server/runtime)

- Maps session_id -> Agent (one Agent per session), with lifecycle:
  - create on first use, reuse on subsequent requests
  - idle cleanup and optional pooling caps
- API sketch:

```rust
pub struct AgentManager { /* session map, idle policy, limits */ }

impl AgentManager {
    pub async fn get_agent(&self, sid: SessionId) -> Arc<Agent> { /* … */ }

    pub async fn execute(
        &self,
        sid: SessionId,
        source: RecipeSource,      // File | Inline(Recipe) | Text(String)
        mode: ExecutionMode        // Interactive | Background | SubTask
    ) -> Result<ExecutionResult>;  // streams or buffered
}
```

2) Treat everything as a Recipe at the boundary

- Dynamic tasks already support `inline_recipe` (see `dynamic_task_tools.rs`). Keep `text_instruction` for back‑compat, but internally convert to an inline Recipe.
- Sub‑recipes: resolve the referenced file into a Recipe and execute inline (default), falling back to CLI only when necessary.
- Scheduler: load the recipe file and call AgentManager.execute in `Background` mode, generating a normal session.

3) Server: per‑session agents

- Replace the single shared AgentRef in `goose-server` with AgentManager.
- Endpoints work the same (send message -> reply stream), but now execution goes through AgentManager for that session_id.
- Each session gets its own ExtensionManager/ToolMonitor/channels, avoiding interleaving and global lock contention.

4) Dynamic tasks: keep the tool, unify the backend

- `dynamic_task__create_task` continues to exist.
- Under the hood, created tasks are `inline_recipe` tasks executed by the same unified executor in the parent session, optionally as SubTask mode with scoped extensions.

5) Scheduler integration

- Replace ad‑hoc `Agent::new()` in the scheduler with calls into AgentManager.
- Each run creates a new session (or uses a deterministic session id if desired), and stores `schedule_id` in session metadata (this already exists today).

6) Backward compatibility

- Keep existing tool names and CLI commands.
- Internally route all paths to the unified executor.
- Maintain subagent concept for isolation, but it’s orchestrated inside the same pipeline.

## Benefits

- Isolation: each session has its own Agent, extensions, and tool monitor.
- Consistency: chat, scheduler, dynamic tasks, and sub‑recipes all run through the same execution pipeline.
- Simpler mental model and less duplication: one way to run an agent.
- Better multi‑user and multi‑session support in goosed/goose-server.
- Cleaner observability: one session format, one set of metrics.

## Acceptance criteria

- goose-server can run many sessions concurrently; enabling/disabling extensions in one session never affects another.
- Dynamic tasks and sub‑recipes execute via the same backend as interactive chat (no surprise differences in behavior).
- Scheduler runs show up as normal sessions with schedule_id metadata and can be browsed/inspected via existing APIs/UI.
- No loss of existing functionality: current tools and CLI flows still work.

## Open questions

- Resource management/pooling: what caps and eviction policies do we want for Agents and MCP connections?
- Extension reuse: should some read‑only/state‑free extensions be shared across sessions to save resources, or keep everything per‑session for strict isolation?
- Sub‑recipe CLI fallback: define when/why we still spawn a process (e.g., running from a different workspace or needing process isolation) and make it explicit.

## Notes from the current codebase

- Shared server Agent: `crates/goose-server/src/state.rs` keeps a single `AgentRef` for all requests.
- Scheduler creates its own Agent and provider: `crates/goose/src/scheduler.rs` (`run_scheduled_job_internal`).
- Dynamic tasks already support `inline_recipe` and convert arguments to a Recipe: `crates/goose/src/agents/recipe_tools/dynamic_task_tools.rs` and `.../subagent_execution_tool/tasks.rs`.
- Sub‑recipes currently spawn `goose run --recipe …` in some paths: `crates/goose/src/agents/subagent_execution_tool/tasks.rs`.

By centralizing agent lifecycle per session and pushing all execution through the same path, we get predictability, easier debugging, and a strong base for future features (e.g., quotas, pre‑warmed agents, richer scheduling, better metrics).
