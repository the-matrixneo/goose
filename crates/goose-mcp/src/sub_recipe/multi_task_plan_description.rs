pub const MULTI_TASK_PLAN_DESCRIPTION: &str = r#"A detailed tool for dynamic and reflective problem-solving through tasks.
This tool helps analyze problems through a flexible thinking process that can adapt and evolve, and track execution outcomes related to tasks.
Each task can build on, question, or revise previous insights as understanding deepens.

When to use this tool:
- Breaking down complex problems into tasks
- Planning and design with room for revision
- Analysis that might need course correction
- Analysis that task depends on other tasks
- Problems where the full scope might not be clear initially
- Problems that require a multi-step solution
- Tasks that need to maintain context over multiple tasks
- Situations where irrelevant information needs to be filtered out

Key features:
- You can adjust total_tasks up or down as you progress
- You can question or revise previous tasks
- You can add more tasks even after reaching what seemed like the end
- You can express uncertainty and explore alternative approaches
- Not every task needs to build linearly - you can branch or backtrack
- Generates a solution hypothesis
- Verifies the hypothesis based on the Chain of Task steps
- Repeats the process until satisfied
- Provides a correct answer

Parameters explained:
- task: Your current task, which can include:
* Regular analytical steps
* Revisions of previous tasks
* Questions about previous decisions
* Realizations about needing more analysis
* Changes in approach
* Hypothesis generation
* Hypothesis verification
- next_task_needed: True if you need more tasks, even if at what seemed like the end
- task_id: A unique ID for the task
- task_number: Current number in sequence (can go beyond initial total if needed)
- total_tasks: Current estimate of tasks needed (can be adjusted up/down)
- is_revision: A boolean indicating if this task revises previous task
- revises_task: If is_revision is true, which task number is being reconsidered
- branch_from_task: If branching, which task number is the branching point
- branch_id: Identifier for the current branch (if any)
- needs_more_tasks: If reaching end but realizing more tasks needed
- depends_on: Task IDs this task depends on. If not provided, the task is independent.
- execution_id: A unique ID for the execution attempt for the task
- execution_status: task execution status. One of 'pending', 'running', 'completed', 'failed'

You should:
1. Start with an initial estimate of needed tasks, but be ready to adjust
2. If a task depends on other tasks, use the depends_on parameter to indicate the task IDs that the current task depends on.
3. Feel free to question or revise previous tasks
4. Don't hesitate to add more tasks if needed, even at the "end"
5. Express uncertainty when present
6. Mark tasks that revise previous tasks or branch into new paths
7. Ignore information that is irrelevant to the current task
8. Generate a solution hypothesis when appropriate
9. Verify the hypothesis based on the Chain of Thought tasks
10. Repeat the process until satisfied with the solution
11. Provide a single, ideally correct answer as the final output
12. Only set next_task_needed to false when truly done and a satisfactory answer is reached
13. Track the execution status of each task
    - Set "execution_status" to "pending" initially
    - Then update it to "running", "completed", or "failed"
    Example:
    {
        "task": "Perform task 1",
        "execution_status": "pending"
    }

    Then later:

    {
        "task": "Perform task 1",
        "execution_status": "completed",
    }
14. Optimize sub-recipe execution by checking the dependencies of the task. If the sub recipe task does not depend on any other task, then run it immediately.
"#;

pub const MULTI_TASK_PLAN_SCHEMA: &str = r#"{
    "type": "object",
    "properties": {
        "task": {
            "type": "string",
            "description": "Your current thinking step"
        },
        "task_id": {
            "type": "string",
            "description": "A unique ID for the task"
        },
        "next_task_needed": {
            "type": "boolean",
            "description": "Whether another task step is needed"
        },
        "task_number": {
            "type": "integer",
            "description": "Current task number",
            "minimum": 1
        },
        "total_tasks": {
            "type": "integer",
            "description": "Estimated total tasks needed",
            "minimum": 1
        },
        "is_revision": {
            "type": "boolean",
            "description": "Whether this revises previous thinking"
        },
        "revises_task": {
            "type": "integer",
            "description": "Which task is being reconsidered",
            "minimum": 1
        },
        "branch_from_task": {
            "type": "integer",
            "description": "Branching point task number",
            "minimum": 1
        },
        "branch_id": {
            "type": "string",
            "description": "Branch identifier"
        },
        "needs_more_tasks": {
            "type": "boolean",
            "description": "If more tasks are needed"
        },
        "depends_on": {
            "type": "array",
            "description": "Task IDs this task depends on. If not provided, the task is independent.",
            "items": { "type": "string" }
        },
        "execution_id": {
            "type": "string",
            "description": "A unique ID for the execution attempt for the task"
        },
        "execution_status": {
            "type": "string",
            "enum": ["pending", "running", "completed", "failed"],
            "description": "Task execution status"
        }
    },
    "required": ["task", "next_task_needed", "task_number", "total_tasks", "task_id"]
}"#;
