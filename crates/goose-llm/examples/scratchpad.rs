use std::vec;

use anyhow::Result;
use goose_llm::{
    completion, message::{ToolRequestToolCall, ToolResponseToolResult},  types::{completion::{
        CompletionRequest, CompletionResponse, ExtensionConfig, ToolApprovalMode, ToolConfig
    }, core::{Content, ToolCall}}, Message, ModelConfig
};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    let provider = "databricks";
    let provider_config = json!({
        "host": std::env::var("DATABRICKS_HOST").expect("Missing DATABRICKS_HOST"),
        "token": std::env::var("DATABRICKS_TOKEN").expect("Missing DATABRICKS_TOKEN"),
    });
    let model_name = "claude-3-5-haiku"; // "goose-gpt-4-1";
    let model_config = ModelConfig::new(model_name.to_string());

    let lookup_sources_tool = ToolConfig::new(
        "lookup_sources",
        "Get the raw source data for Slack tile updates",
        json!({
            "type": "object",
            "required": ["source_ids"],
            "additionalProperties": false,
            "properties": {
                "source_ids": {
                    "type": "array",
                    "items": { "type": "string" },
                    "description": "List of source IDs in the format SLACK::<channel>::<entity>"
                },
                "dry_run": {
                    "type": "boolean",
                    "default": false,
                    "description": "Validate the query without fetching data"
                }
            }
        }),
        ToolApprovalMode::Auto,
    );

    let get_user_info_tool = ToolConfig::new(
        "get_user_info",
        "Fetch information about a Slack user",
        json!({
            "type": "object",
            "required": ["user_id"],
            "additionalProperties": false,
            "properties": {
                "user_id": {
                    "type": "string",
                    "description": "Slack user ID (e.g. U123ABC456)"
                }
            }
        }),
        ToolApprovalMode::Auto,
    );

    let slack_extension = ExtensionConfig::new(
        "slack".to_string(),
        Some("Slack MCP tools".to_string()),
        vec![lookup_sources_tool, get_user_info_tool],
    );


    let extensions = vec![
        slack_extension,
    ];

    let user_setup_msg = r#" We surface custom Tiles that users configure to aggregate and visualize data from their connected sources.
       For example, a user might set up a Tile to show highlights from a specific Slack channel.
       Each Tile can render its data in different formats—plain text (following a user-defined schema), bar charts, pie charts, etc.
       Below you’ll find the Tile’s metadata and the most recent result. The user may ask follow-up questions about this result.
       You can re-fetch the raw data via the get_sources tool on the extension named extension_source from tile_metadata if needed to answer their queries.
       tile_metadata also contains the prop_schema field which contains information on how the result is formatted.
       The parameters that you would use to call would be available from the tile_result in the field called source. The format
       of that field is extension_source::ID and this ID field should be passed to get_sources tool call on the
        extension_source extension to get further details. You should request tool calls as needed
        to dive further, you are not limited to the information provided here

     {
      "tile_metadata" :{"type": TILE_TYPE_UPDATE,
       "prop_schema" :         "name": "daily_update",
  "description": "JSON schema for daily update tile",
  "parameters": {
    "type": "object",
    "required": [
      "updates"
    ],
    "properties": {
      "updates": {
        "type": "array",
        "description": "Updates sourced from the raw data",
        "items": {
          "type": "object",
          "required": [
            "short",
            "detail",
            "avatar",
            "source"
          ],
          "properties": {
            "short": {
              "type": "string",
              "description": "A few word description of the update for an overview"
            },
            "detail": {
              "type": "string",
              "description": "The full update content for the zoomed in view"
            },
            "avatar": {
              "type": "string",
              "description": "The avatar url of the person who is the source of the update. Copy this value from the original message's 'author.avatar_url' field."
            },
            "source": {
              "type": "string",
              "description": "The source of the update, such as the slack channel and message/thread. Copy this value from the original message's 'id' field."
            }
          },
          "additionalProperties": false
        }
      }
    },
    "additionalProperties": false
  }
}}
      "tile_result" = {
  "updates": [
    {
      "short": "Slack integration issue resolved",
      "detail": "Kerry fixed an issue where their Slack identifier was out of sync with the server. They suggest adding a way for the server to alert clients for reconfiguration if identifiers mismatch. The problem was resolved after purging local encryption keys. Ticket GOOS-41 has been created to address credential sync issues for all platforms.",
      "avatar": "https://avatars.slack-edge.com/2023-02-09/4771905968598_d784c995ff94be7ca5fc_192.jpg",
      "source": "SLACK::goose-client::1748475656.945379"
    },
    {
      "short": "Mobile Slack credential error handling",
      "detail": "Colby notes Android has no error handling for Slack credentials—if they get out of sync, things just fail, though users can easily add new ones. Arthur confirms iOS would also be affected. The team agrees a clearer server response about encryption or credential mismatch is needed.",
      "avatar": "",
      "source": "SLACK::goose-client::1748476099.059189"
    },
    {
      "short": "Ongoing staging API/504 issues",
      "detail": "There are persistent 504 Gateway Timeouts when calling Slack functions in staging. Despite timeouts, some endpoints (like get messages) still respond. Team members treat 504s as non-blocking for now and continue testing.",
      "avatar": "",
      "source": "SLACK::goose-client::1748476605.298179"
    },
    {
      "short": "Goose Web UI project kickoff",
      "detail": "Vitalie Scurtu started planning a temporary Goose Web UI for PTO and time off requests. They seek advice on implementation to ensure future migration is easy. The team discusses whether to use a new repo or branch from cash-web, and possible reuse of existing Goose components.",
      "avatar": "",
      "source": "SLACK::goose-client::1748470607.125029"
    }
  ]
}
      }"#;

    let question_msg = "Who is working on the API/Timeout issue";

    // assistant's tool-call request (note: arguments is a JSON object)
    let assistant_tool_call = ToolRequestToolCall::new( ToolCall::new(
        "slack__lookup_sources".to_string(),
        json!({
                "source_ids": ["SLACK::goose-client::1748476605.298179"]
        })
    ));
    

    // user's tool response
    let tool_resp = ToolResponseToolResult::new(
        vec![
            Content::text("{\n            \"ok\": true,\n            \"channel_id\": \"C08PPKTMZFC\",\n            \"thread_ts\": \"1748476605.298179\",\n            \"replies\": [{\n                \"text\": \"I believe 504 is a good thing. for now at least.\",\n                \"ts\": \"1748476605.298179\",\n                \"user\": \"U03GC4QG4LF\",\n                \"thread_ts\": \"1748471678.314739\"\n            }],\n            \"has_more\": false,\n            \"source_id\": \"SLACK::goose-client::1748476605.298179\"\n          }")
        ]
    );

    let messages = vec![
        Message::user().with_text(user_setup_msg),
        Message::user().with_text(question_msg),
        Message::assistant().with_tool_request("id1", assistant_tool_call),
        Message::user().with_tool_response("id1", tool_resp)
    ];

    let response: CompletionResponse = completion(CompletionRequest::new(
        provider.to_string(),
        provider_config,
        model_config,
        Some("You are a helpful AI assistant.".to_string()),
        None,                       // no system prompt override
        messages,
        extensions,      // only the Slack extension
    ))
    .await?;

    println!("{}", serde_json::to_string_pretty(&response)?);

    Ok(())
}
