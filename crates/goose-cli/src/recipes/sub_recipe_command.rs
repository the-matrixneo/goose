use goose::recipe::{Recipe, SubRecipe};

pub fn create_sub_recipe_instructions(recipe: &Recipe) -> String {
    let mut instructions = String::new();
    if let Some(sub_recipes) = &recipe.sub_recipes {
        for sub_recipe in sub_recipes {
            let sub_recipe_command = create_sub_recipe_command(sub_recipe);
            instructions.push_str(&format!("if {} is required to run, then use sub_recipe_run tool to run the command: {} \n", sub_recipe.name, sub_recipe_command));
        }
    }
    instructions
}

pub fn recipe_runner_instructions(recipe: &Recipe) -> String {
    let mut recipe_plan_instructions = format!(
        "Before start running the recipe, take the recipe instructions and prompt \
        and a structured task list in YAML format with the following rules:\n\
        \n\
        - Each task must have a unique `id`.
        - Tasks should be **parallelized wherever possible**:
            - If two tasks are independent, place them without `depends_on`.
            - Only use `depends_on` when there is an explicit dependency.
        - Avoid sequential execution unless truly necessary.
        - If a task produces output needed by another, define its `output` and reference it in the dependent task via `{{output_id}}`.
        - Ensure the YAML structure is deterministic, correct, and does not include explanation or commentary.
        - Output only a YAML block (no prose or markdown syntax).
        Format Example:\n\
        ```yaml\n\
        tasks:\n\
        - id: fetch_weather\n\
            run: use weather recipe to get weather for {{city}}\n\
            output: weather_info\n\
        \n\
        - id: write_weather_file\n\
            run: write {{weather_info}} to weather.txt\n\
            depends_on: [fetch_weather]\n\
        \n\
        - id: fetch_joke\n\
            run: use joke-of-the-day recipe\n\
            output: joke_text\n\
        \n\
        - id: write_joke_file\n\
            run: write {{joke_text}} to joke.txt\n\
            depends_on: [fetch_joke]\n\
        ```\n\
        \n\
        recipe instructions:\n\
        {:?}\n\
        recipe prompt:\n\
        {:?}\n\
        \n\
        Please print the task list first. \n\
        After generating the task list, execute the tasks efficiently with the following strategy:\n\
        \n\
        - Tasks with no dependencies run immediately.
        - Tasks with dependencies wait until all required tasks complete.
        - If a task involves running a sub-recipe, submit it to a job queue with its ID, run command, and dependencies.
        - If a task is a local action, execute it directly once dependencies are satisfied.
        - Continue executing tasks as they become unblocked.",
        recipe.instructions,
        recipe.prompt
    );
    recipe_plan_instructions.push_str(&create_sub_recipe_instructions(recipe));
    recipe_plan_instructions
}

fn create_sub_recipe_command(sub_recipe: &SubRecipe) -> String {
    let mut command = String::new();
    command.push_str(&format!("goose run --recipe {} ", sub_recipe.file));
    if let Some(params) = &sub_recipe.params {
        for param in params {
            command.push_str(&format!(" --params {}={}", param.name, param.value));
        }
    }
    return command;
}