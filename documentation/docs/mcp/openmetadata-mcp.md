---
title: OpenMetadata Extension
description: Add OpenMetadata MCP Server as a Goose Extension
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';
import GooseDesktopInstaller from '@site/src/components/GooseDesktopInstaller';

The [OpenMetadata MCP Server](https://open-metadata.org/mcp) extension allows Goose to interact directly with your OpenMetadata, enabling operations for OpenMetadata assets, glossaries, and lineage. This makes it easy to work with your metadata stored in OpenMetadata through natural language interactions.

:::tip TLDR
<Tabs groupId="interface">
  <TabItem value="ui" label="Goose Desktop" default>
  [Launch the installer](goose://extension?cmd=npx&arg=-y&arg=mcp-remote&arghttp://localhost:8585/mcp&arg=--auth-server-url&arg=http://localhost:8585/mcp&id=openmetadata&name=OpenMetadata&description=OpenMetadata%20integration)
  </TabItem>
  <TabItem value="cli" label="Goose CLI">
  **Command**
  ```sh
  npx -y mcp-remote http://localhost:8585/mcp --auth-server-url=http://localhost:8585/mcp --client-id=openmetadata --verbose --clean --header Authorization:${AUTH_HEADER}
  ```
  </TabItem>
</Tabs>
:::

## Customizing Your Connection

The OpenMetadata MCP server connects to an OpenMetadata instance using OpenMetadata's embedded MCP server. We're using `http://localhost:8585` as an example here to access a local OpenMetadata instance, but you can configure this for your own environment. AUTH_HEADER here would an [OpenMetadata Personal Access Token (PAT)](https://docs.open-metadata.org/latest/how-to-guides/mcp#adding-a-personal-access-token-to-your-mcp-client).

## Configuration

:::info
Note that you'll need [Node.js](https://nodejs.org/) installed on your system to run this command, as it uses `npx`. You'll also need a running OpenMetadata instance or access to the [OpenMetadata sandbox](https://sandbox.open-metadata.org/).
:::

<Tabs groupId="interface">
  <TabItem value="ui" label="Goose Desktop" default>
    <GooseDesktopInstaller
      extensionId="openmetadata"
      extensionName="OpenMetadata"
      description="OpenMetadata integration"
      command="npx"
      args={["-y", "mcp-remote", "http://localhost:8585/mcp", "--auth-server-url=http://localhost:8585/mcp", "--client-id=openmetadata", "--verbose", "--clean", "--header", "Authorization:${AUTH_HEADER}"]}
    />
    
    :::info Configure Your Connection String
    If needed, [update the extension](/docs/getting-started/using-extensions#updating-extension-properties) to match to your [OpenMetadata environment](#customizing-your-connection).
    :::

  </TabItem>
  <TabItem value="cli" label="Goose CLI">
  1. Run the `configure` command:
  ```sh
  goose configure
  ```

  2. Choose to add a `Command-line Extension`
  ```sh
    ┌   goose-configure 
    │
    ◇  What would you like to configure?
    │  Add Extension (Connect to a new extension) 
    │
    ◆  What type of extension would you like to add?
    │  ○ Built-in Extension 
    // highlight-start    
    │  ● Command-line Extension (Run a local command or script)
    // highlight-end    
    │  ○ Remote Extension 
    └ 
  ```

  3. Name your extension
  ```sh
    ┌   goose-configure 
    │
    ◇  What would you like to configure?
    │  Add Extension (Connect to a new extension) 
    │
    ◇  What type of extension would you like to add?
    │  Command-line Extension 
    │
    // highlight-start
    ◆  What would you like to call this extension?
    │  OpenMetadata
    // highlight-end
    └ 
  ```

  4. Enter the command with an URL that matches your [OpenMetadata environment](#customizing-your-connection)
  ```sh
    ┌   goose-configure 
    │
    ◇  What would you like to configure?
    │  Add Extension (Connect to a new extension) 
    │
    ◇  What would you like to call this extension?
    │  OpenMetadata
    │
    // highlight-start
    ◆  What command should be run?
    │  npx -y mcp-remote http://localhost:8585/mcp --auth-server-url=http://localhost:8585/mcp --client-id=openmetadata --verbose --clean --header Authorization:${AUTH_HEADER}
    // highlight-end
    └ 
  ```  

  5. Set the timeout (default 300s is usually sufficient)
  ```sh
    ┌   goose-configure 
    │
    ◇  What would you like to configure?
    │  Add Extension (Connect to a new extension) 
    │
    ◇  What would you like to call this extension?
    │  OpenMetadata
    │
    ◇  What command should be run?
    │  npx -y mcp-remote http://localhost:8585/mcp --auth-server-url=http://localhost:8585/mcp --client-id=openmetadata --verbose --clean --header Authorization:${AUTH_HEADER}
    │
    // highlight-start
    ◆  Please set the timeout for this tool (in secs):
    │  300
    // highlight-end
    └ 
  ```

  6. Choose to add a description. If you select "Yes" here, you will be prompted to enter a description for the extension.
  ```sh
    ┌   goose-configure 
    │
    ◇  What would you like to configure?
    │  Add Extension (Connect to a new extension) 
    │
    ◇  What would you like to call this extension?
    │  OpenMetadata
    │
    ◇  What command should be run?
    │  npx -y mcp-remote http://localhost:8585/mcp --auth-server-url=http://localhost:8585/mcp --client-id=openmetadata --verbose --clean --header Authorization:${AUTH_HEADER}
    │
    ◇  Please set the timeout for this tool (in secs):
    │  300
    │
    // highlight-start
    ◆  Would you like to add a description?
    │  No
    // highlight-end
    └ 
  ```

  7. Complete the configuration by added your [OpenMetadata Personal Access Token (PAT)](https://docs.open-metadata.org/latest/how-to-guides/mcp#adding-a-personal-access-token-to-your-mcp-client)
  ```sh
    ┌   goose-configure 
    │
    ◇  What would you like to configure?
    │  Add Extension (Connect to a new extension) 
    │
    ◇  What would you like to call this extension?
    │  OpenMetadata
    │
    ◇  What command should be run?
    │  npx -y mcp-remote http://localhost:8585/mcp --auth-server-url=http://localhost:8585/mcp --client-id=openmetadata --verbose --clean --header Authorization:${AUTH_HEADER}
    │     
    ◇  Please set the timeout for this tool (in secs):
    │  300
    │
    ◇  Would you like to add a description?
    │  No
    │    
    // highlight-start
    ◆  Would you like to add environment variables?
    │  Yes
    │ 
    ◇  Environment variable name:
    │  AUTH_HEADER
    │
    ◇  Environment variable value:
    │  "Bearer <YOUR_OpenMetadata_PAT>"
    │
    ◇  Add another environment variable?
    │  No
    │
// highlight-end
    └  Added OpenMetadata extension
  ```  

  </TabItem>
</Tabs>

## Available Operations

The OpenMetadata extension provides comprehensive metadata management capabilities through natural language interactions. You can perform a wide range of operations including:

### OpenMetadata Operations
- **search_metadata** Find data assets and business terms in your OpenMetadata catalog.
- **get_entity_details** Retrieve detailed information about a specific entity using its fully qualified name.
- **create_glossary** Create a new glossary to organize business terms and definitions.
- **create_glossary_term** Create a new term within an existing glossary, with support for hierarchical relationships.
- **get_entity_lineage** Retrieve upstream and downstream lineage information for any entity to understand data dependencies and impact analysis. 

Just describe what you want to accomplish in natural language, and Goose will help you execute the appropriate OpenMetadata operations safely and efficiently.

## Example Usage

In this example, Goose looks at the metadata for a Redshift table named `customers` stored in OpenMetadata, particularly its lineage. If the lineage does not show that a Databricks table named `customerinfo` is used to build `customers`, Goose will add it.

#### Goose Prompt
```
which tables are used to create my customers table in redshift? if the databricks table customerinfo is not already, add it to the customers table lineage.
```

#### Goose Output
```
Based on my analysis, I found your Redshift customers table at `redshift prod.dev.dbt_jaffle.customers`. Here are the tables currently used to create it:

##### Current Upstream Tables for Redshift Customers Table
###### Direct Dependencies:
1. `redshift prod.dev.dbt_jaffle.stg_customers` (Staging Table for Customers)
2. `redshift prod.dev.dbt_jaffle.stg_payments`
3. `redshift prod.dev.dbt_jaffle.stg_orders`

######Indirect Dependencies (Raw Tables):
4. `redshift prod.dev.dbt_jaffle.raw_customers` → feeds into `stg_customers`
5. `redshift prod.dev.dbt_jaffle.raw_payments` → feeds into `stg_payments`
6. `redshift prod.dev.dbt_jaffle.raw_orders` → feeds into `stg_orders`

######Missing Databricks CustomerInfo Table
I found your Databricks `customerinfo` table at:

`DatabricksBanking.demo.mssqlbanking_customer.customerinfo`
This table is NOT currently connected to your Redshift customers table lineage.

I have successfully created the lineage between the Databricks customerinfo table and the Redshift customers table. Here's what I accomplished:
######Lineage Created Successfully
Source Table (Databricks):

Name: `customerinfo`
Full Name: `DatabricksBanking.demo.mssqlbanking_customer.customerinfo`
Platform: Databricks
Columns: customerid, firstname, lastname, dateofbirth, email, phonenumber, _fivetran_deleted, _fivetran_synced

Target Table (Redshift):

Name: `customers`
Full Name: `redshift prod.dev.dbt_jaffle.customers`
Platform: Redshift
Columns: customer_id, first_name, last_name, first_order, most_recent_order, number_of_orders, customer_lifetime_value

The lineage relationship now shows that the Databricks customerinfo table is an upstream source that feeds data to create the Redshift customers table. This relationship will be visible in OpenMetadata's lineage view, helping you track data flow and understand dependencies between your Databricks and Redshift systems.
You can now view this lineage in the OpenMetadata UI by navigating to either table and checking the "Lineage" tab to see the connection between these customer data assets.
```
