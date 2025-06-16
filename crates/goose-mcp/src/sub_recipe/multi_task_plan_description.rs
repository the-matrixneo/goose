pub const MULTI_TASK_PLAN_DESCRIPTION: &str = r#"A detailed tool for dynamic and reflective problem-solving through tasks.
This tool helps analyze problems through a flexible thinking process that can adapt and evolve, and track execution outcomes related to tasks.
Each task can build on, question, or revise previous insights as understanding deepens.

When to use this tool:
- Breaking down complex problems into tasks
- Planning and design with room for revision
- Analysis that might need course correction
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
2. Feel free to question or revise previous tasks
3. Don't hesitate to add more tasks if needed, even at the "end"
4. Express uncertainty when present
5. Mark tasks that revise previous tasks or branch into new paths
6. Ignore information that is irrelevant to the current task
7. Generate a solution hypothesis when appropriate
8. Verify the hypothesis based on the Chain of Thought tasks
9. Repeat the process until satisfied with the solution
10. Provide a single, ideally correct answer as the final output
11. Only set next_task_needed to false when truly done and a satisfactory answer is reached
12. Track the execution status of each task
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
"#;