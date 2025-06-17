---
draft: true
---
import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';

:::note Preview Feature
Tool Selection Strategy is currently in preview and works only with Claude models served on Databricks.
:::

Enabling an [extension](/docs/getting-started/using-extensions) gives you access to all of its tools. For example, when you enable the Google Drive extension, you get access to tools for reading documents, managing spreadsheets, updating permissions, handling comments, and more.

Enabling multiple extensions gives you access to a wider range of tools, but loading them all into the context can be inefficient and potentially confusing for the LLM. It's like having every tool in your workshop spread out on your bench when you only need to use one or two. 

Instead of loading all tools every time Goose interacts with the LLM, use an intelligent tool selection strategy to load just the tools needed for your current task. Both vector and LLM-based strategies ensure that only the functionality you actually need is loaded to context. This feature provides:

- Reduced token consumption
- Improved LLM performance
- Better context management
- More accurate and efficient tool selection

## Selection Strategies

Aside from the default strategy, you can choose a vector or LLM-based strategy to manage which tools are loaded into context.

### Default
Loads all tools from enabled extensions into context. This strategy works well when you have a small number of extensions enabled.

### Vector-based
Recommended when multiple extensions are enabled and you want fast, keyword-based tool matching. Uses mathematical similarity between embeddings to find relevant tools, providing efficient matching based on vector similarity between your query and available tools.

Best for:
- Situations where fast response times are critical
- Queries with keywords that match tool names or descriptions

Example:
- Query: "read pdf file"
- Result: Quickly matches with PDF-related tools based on keyword similarity


### LLM-based
Recommended when multiple extensions are enabled and you need context-aware tool selection. Leverages natural language understanding to analyze tools and queries semantically, making selections based on the full meaning of your request.

Best for:
- Complex or ambiguous queries that require understanding context
- Cases where exact keyword matches might miss relevant tools
- Situations where nuanced tool selection is important

Example:
- Query: "help me analyze the contents of my document"
- Result: Understands context and might suggest both PDF readers and content analysis tools

## Configuration

<Tabs groupId="interface">
  <TabItem value="ui" label="Goose Desktop" default>
    1. Click `⚙️` in the upper right corner
    2. Click `Advanced settings`
    3. Under `Tool Selection Strategy`, select your preferred strategy:
       - `Default`
       - `Vector`
       - `LLM-based`
  </TabItem>
  <TabItem value="cli" label="Goose CLI">
    1. Run the `configuration` command:
    ```sh
    goose configure
    ```

    2. Select `Goose Settings`:
    ```sh
    ┌   goose-configure
    │
    ◆  What would you like to configure?
    │  ○ Configure Providers
    │  ○ Add Extension
    │  ○ Toggle Extensions
    │  ○ Remove Extension
    // highlight-start
    │  ● Goose Settings (Set the Goose Mode, Tool Output, Tool Permissions, Experiment, Goose recipe github repo and more)
    // highlight-end
    └ 
    ```

    3. Select `Router Tool Selection Strategy`:
    ```sh
    ┌   goose-configure
    │
    ◇  What would you like to configure?
    │  Goose Settings
    │
    ◆  What setting would you like to configure?
    │  ○ Goose Mode 
    // highlight-start
    │  ● Router Tool Selection Strategy (Configure the strategy for selecting tools to use)
    // highlight-end
    │  ○ Tool Permission 
    │  ○ Tool Output 
    │  ○ Toggle Experiment 
    │  ○ Goose recipe github repo 
    └ 
    ```

    4. Select your preferred strategy:
    ```sh
   ┌   goose-configure 
   │
   ◇  What would you like to configure?
   │  Goose Settings 
   │
   ◇  What setting would you like to configure?
   │  Router Tool Selection Strategy 
   │
    // highlight-start
   ◆  Which router strategy would you like to use?
   │  ● Vector Strategy (Use vector-based similarity to select tools)
   │  ○ Default Strategy 
    // highlight-end
   └  
    ```
      
       :::note
       Currently, the LLM-based strategy can't be configured using the CLI.
       :::

       The following example shows the `Vector Strategy` was selected:

    ```
    ┌   goose-configure
    │
    ◇  What would you like to configure?
    │  Goose Settings
    │
    ◇  What setting would you like to configure?
    │  Router Tool Selection Strategy
    │
    ◇  Which router strategy would you like to use?
    │  Vector Strategy
    │
    └  Set to Vector Strategy - using vector-based similarity for tool selection
    ```

  </TabItem>
</Tabs>

Goose displays messages that indicate when the vector or LLM-based strategy is currently being used.