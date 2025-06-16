pub const BREAK_DOWN_TASK_DESCRIPTION: &str = r#"A detailed tool for dynamic break down a recipe into smaller tasks.
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
- You can add more tasks even after reaching what seemed like the end
- You can express uncertainty and explore alternative approaches


You should:
1. Break down the recipe into smaller tasks
2. If a task depends on other tasks, use the depends_on parameter to indicate the task IDs that the current task depends on.
3. Provide a single, ideally correct answer as the final output
4. Track the execution status of each task
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

pub const BREAK_DOWN_TASK_SCHEMA: &str = r#"{
    "type": "object",
    "properties": {
        "tasks": {
            "type": "array",
            "description": "A list of tasks that are break down from the recipe",
            "items": {
                "type": "object",
                "properties": {
                    "task": {
                        "type": "string",
                        "description": "task description"
                    },
                    "task_id": {
                        "type": "string",
                        "description": "A unique ID for the task"
                    },
                    "depends_on": {
                        "type": "array",
                        "description": "Task IDs this task depends on. If not provided, the task is independent.",
                        "items": { "type": "string" }
                    },
                    "execution_status": {
                        "type": "string",
                        "enum": ["pending", "running", "completed", "failed"],
                        "description": "Task execution status"
                    }
                }
            }
        }
    },
    "required": ["tasks"]
}"#;
