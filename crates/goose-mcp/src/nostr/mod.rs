use nostr_sdk::{Client, EventBuilder, Filter, FromBech32, Keys, PublicKey, RelayUrl, SecretKey, Timestamp, ToBech32};
use rmcp::{
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{
        CallToolResult, Content, ErrorCode, ErrorData, Implementation, ServerCapabilities,
        ServerInfo,
    },
    schemars::JsonSchema,
    tool, tool_handler, tool_router, ServerHandler,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Validates that a parameter exists and has the expected type
#[allow(dead_code)]
fn validate_string_param(params: &Value, param_name: &str) -> Result<String, ErrorData> {
    let value = params.get(param_name).ok_or_else(|| {
        ErrorData::new(
            ErrorCode::INVALID_PARAMS,
            format!("Missing '{}' parameter", param_name),
            None,
        )
    })?;

    value
        .as_str()
        .ok_or_else(|| {
            ErrorData::new(
                ErrorCode::INVALID_PARAMS,
                format!("The '{}' parameter must be a string", param_name),
                None,
            )
        })
        .map(|s| s.to_string())
}

/// Parameters for generating a new nostr public/private key pair
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct GenerateKeyPairParams {
    /// Whether to include the private key in the response (default: false)
    #[serde(default)]
    pub include_private_key: bool,
}

/// Parameters for reading recent messages from a pubkey
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ReadRecentMessagesParams {
    /// The public key (hex format) to read messages from
    pub pubkey: String,
    /// Optional relay URL to connect to (default: wss://relay.damus.io)
    pub relay_url: Option<String>,
    /// Number of minutes to look back (default: 10)
    #[serde(default = "default_minutes")]
    pub minutes_back: u64,
}

fn default_minutes() -> u64 {
    10
}

/// Parameters for publishing a message to Nostr
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PublishMessageParams {
    /// The message content to publish
    pub content: String,
    /// The private key in hex format (without 0x prefix) or bech32 format (nsec...)
    pub private_key: String,
    /// Optional relay URL to publish to (default: wss://relay.damus.io)
    pub relay_url: Option<String>,
}

/// Result of publishing a message
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PublishMessageResult {
    /// Event ID of the published message
    pub event_id: String,
    /// Author's public key
    pub author_pubkey: String,
    /// The relay URL that was used
    pub relay_url: String,
    /// The content that was published
    pub content: String,
    /// Timestamp when the event was created
    pub created_at: u64,
    /// Success status
    pub success: bool,
}

/// Result of generating a key pair
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct KeyPairResult {
    /// The public key in hex format
    pub public_key: String,
    /// The public key in bech32 format (npub...)
    pub public_key_bech32: String,
    /// The private key in hex format (only if requested)
    pub private_key: Option<String>,
    /// The private key in bech32 format (nsec...) (only if requested)
    pub private_key_bech32: Option<String>,
}

/// A nostr message/event
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct NostrMessage {
    /// Event ID
    pub id: String,
    /// Author's public key
    pub pubkey: String,
    /// Event timestamp
    pub created_at: u64,
    /// Event kind
    pub kind: u16,
    /// Event content
    pub content: String,
    /// Event signature
    pub sig: String,
}

pub struct NostrRouter {
    tool_router: ToolRouter<Self>,
    instructions: String,
}

impl Default for NostrRouter {
    fn default() -> Self {
        Self::new()
    }
}

#[tool_handler(router = self.tool_router)]
impl ServerHandler for NostrRouter {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            server_info: Implementation {
                name: "goose-nostr".to_string(),
                version: env!("CARGO_PKG_VERSION").to_owned(),
            },
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            instructions: Some(self.instructions.clone()),
            ..Default::default()
        }
    }
}

#[tool_router(router = tool_router)]
impl NostrRouter {
    pub fn new() -> Self {
        let instructions = r#"This extension provides tools for Nostr protocol operations

## Available Tools:
- **nostr_generate_keypair**: Generate a new Nostr public/private key pair with optional private key inclusion
- **nostr_read_recent_messages**: Read recent messages from a specified Nostr public key from relays
- **nostr_publish_message**: Publish a text message to Nostr using a private key

Use these tools to interact with the Nostr decentralized social network protocol."#.to_string();

        Self {
            tool_router: Self::tool_router(),
            instructions,
        }
    }

    /// Generates a new Nostr public/private key pair
    #[tool(
        name = "nostr_generate_keypair",
        description = r#"Generate a new Nostr public/private key pair

Parameters:
- include_private_key: Boolean (optional, default: false) - Whether to include the private key in the response

Returns:
- public_key: The public key in hex format
- public_key_bech32: The public key in bech32 format (npub...)
- private_key: The private key in hex format (only if requested)
- private_key_bech32: The private key in bech32 format (nsec...) (only if requested)

Example:
{
  "include_private_key": true
}"#
    )]
    pub async fn generate_keypair(
        &self,
        params: Parameters<GenerateKeyPairParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let params = params.0;

        let keys = Keys::generate();
        let public_key = keys.public_key();
        let secret_key = keys.secret_key();

        let mut result = KeyPairResult {
            public_key: public_key.to_hex(),
            public_key_bech32: public_key
                .to_bech32()
                .map_err(|e| ErrorData::new(ErrorCode::INTERNAL_ERROR, e.to_string(), None))?,
            private_key: None,
            private_key_bech32: None,
        };

        if params.include_private_key {
            result.private_key = Some(secret_key.display_secret().to_string());
            result.private_key_bech32 = Some(
                secret_key
                    .to_bech32()
                    .map_err(|e| ErrorData::new(ErrorCode::INTERNAL_ERROR, e.to_string(), None))?,
            );
        }

        let content = serde_json::to_string_pretty(&result)
            .map_err(|e| ErrorData::new(ErrorCode::INTERNAL_ERROR, e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(content)]))
    }

    /// Reads recent messages from a Nostr public key
    #[tool(
        name = "nostr_read_recent_messages",
        description = r#"Read recent messages from a Nostr public key

Parameters:
- pubkey: String - The public key in hex format to read messages from
- relay_url: String (optional, default: "wss://relay.damus.io") - The relay URL to connect to
- minutes_back: Number (optional, default: 10) - Number of minutes to look back for messages

Returns:
- relay_url: The relay URL that was used
- pubkey: The public key that was queried
- messages_found: Number of messages found
- messages: Array of message objects containing id, pubkey, created_at, kind, content, and signature

Example:
{
  "pubkey": "02a1633cabf43e11818c6523757e9c4b4c4b6e5a7c0f7b7d3b4b2e2b2e2b2e2b",
  "relay_url": "wss://relay.damus.io",
  "minutes_back": 60
}"#
    )]
    pub async fn read_recent_messages(
        &self,
        params: Parameters<ReadRecentMessagesParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let params = params.0;

        // Parse the public key
        let public_key = PublicKey::from_hex(&params.pubkey).map_err(|e| {
            ErrorData::new(
                ErrorCode::INVALID_PARAMS,
                format!("Invalid public key: {}", e),
                None,
            )
        })?;

        // Create a new client
        let client = Client::default();

        // Connect to relay
        let relay_url = params
            .relay_url
            .unwrap_or_else(|| "wss://relay.damus.io".to_string());
        let relay_url = RelayUrl::parse(&relay_url).map_err(|e| {
            ErrorData::new(
                ErrorCode::INVALID_PARAMS,
                format!("Invalid relay URL: {}", e),
                None,
            )
        })?;

        client
            .add_relay(relay_url.clone())
            .await
            .map_err(|e| ErrorData::new(ErrorCode::INTERNAL_ERROR, e.to_string(), None))?;
        client.connect().await;

        // Calculate timestamp for X minutes ago
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let since = now - (params.minutes_back * 60);

        // Create filter for recent events from this pubkey
        let filter = Filter::new()
            .author(public_key)
            .since(Timestamp::from(since));

        // Query for events
        let timeout = Duration::from_secs(10);
        let events = client
            .fetch_events_from(vec![relay_url.clone()], filter, timeout)
            .await
            .map_err(|e| ErrorData::new(ErrorCode::INTERNAL_ERROR, e.to_string(), None))?;

        // Convert events to our message format
        let messages: Vec<NostrMessage> = events
            .into_iter()
            .map(|event| NostrMessage {
                id: event.id.to_hex(),
                pubkey: event.pubkey.to_hex(),
                created_at: event.created_at.as_u64(),
                kind: event.kind.as_u16(),
                content: event.content,
                sig: event.sig.to_string(),
            })
            .collect();

        let result = serde_json::json!({
            "relay_url": relay_url.to_string(),
            "pubkey": public_key.to_hex(),
            "messages_found": messages.len(),
            "messages": messages
        });

        let content = serde_json::to_string_pretty(&result)
            .map_err(|e| ErrorData::new(ErrorCode::INTERNAL_ERROR, e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(content)]))
    }

    /// Publishes a text message to Nostr
    #[tool(
        name = "nostr_publish_message",
        description = r#"Publish a text message to Nostr

Parameters:
- content: String - The text content to publish
- private_key: String - The private key in hex format (without 0x prefix) or bech32 format (nsec...)
- relay_url: String (optional, default: "wss://relay.damus.io") - The relay URL to publish to

Returns:
- event_id: The ID of the published event
- author_pubkey: The author's public key
- relay_url: The relay URL that was used
- content: The content that was published
- created_at: Timestamp when the event was created
- success: Boolean indicating success

Example:
{
  "content": "Hello, Nostr world!",
  "private_key": "nsec1abcd...",
  "relay_url": "wss://relay.damus.io"
}"#
    )]
    pub async fn publish_message(
        &self,
        params: Parameters<PublishMessageParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let params = params.0;

        // Parse the private key - try bech32 first, then hex
        let secret_key = if params.private_key.starts_with("nsec") {
            SecretKey::from_bech32(&params.private_key).map_err(|e| {
                ErrorData::new(
                    ErrorCode::INVALID_PARAMS,
                    format!("Invalid bech32 private key: {}", e),
                    None,
                )
            })?
        } else {
            SecretKey::from_hex(&params.private_key).map_err(|e| {
                ErrorData::new(
                    ErrorCode::INVALID_PARAMS,
                    format!("Invalid hex private key: {}", e),
                    None,
                )
            })?
        };

        // Create keys from secret key
        let keys = Keys::new(secret_key);
        let public_key = keys.public_key();

        // Create a text note event (kind 1)
        let event = EventBuilder::text_note(&params.content)
            .sign_with_keys(&keys)
            .map_err(|e| ErrorData::new(ErrorCode::INTERNAL_ERROR, e.to_string(), None))?;

        // Create client and connect to relay
        let client = Client::default();
        let relay_url = params
            .relay_url
            .unwrap_or_else(|| "wss://relay.damus.io".to_string());
        let relay_url = RelayUrl::parse(&relay_url).map_err(|e| {
            ErrorData::new(
                ErrorCode::INVALID_PARAMS,
                format!("Invalid relay URL: {}", e),
                None,
            )
        })?;

        client
            .add_relay(relay_url.clone())
            .await
            .map_err(|e| ErrorData::new(ErrorCode::INTERNAL_ERROR, e.to_string(), None))?;
        client.connect().await;

        // Publish the event
        let event_id = client
            .send_event_to(vec![relay_url.clone()], &event)
            .await
            .map_err(|e| ErrorData::new(ErrorCode::INTERNAL_ERROR, e.to_string(), None))?;

        let result = PublishMessageResult {
            event_id: event_id.to_hex(),
            author_pubkey: public_key.to_hex(),
            relay_url: relay_url.to_string(),
            content: params.content,
            created_at: event.created_at.as_u64(),
            success: true,
        };

        let content = serde_json::to_string_pretty(&result)
            .map_err(|e| ErrorData::new(ErrorCode::INTERNAL_ERROR, e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(content)]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rmcp::model::RawContent;
    use serde_json::json;

    #[test]
    fn test_default_minutes() {
        assert_eq!(default_minutes(), 10);
    }

    #[test]
    fn test_generate_keypair_params() {
        let params = GenerateKeyPairParams {
            include_private_key: true,
        };
        assert_eq!(params.include_private_key, true);

        let default_params = GenerateKeyPairParams {
            include_private_key: false,
        };
        assert_eq!(default_params.include_private_key, false);
    }

    #[test]
    fn test_read_recent_messages_params() {
        let params = ReadRecentMessagesParams {
            pubkey: "02".repeat(32),
            relay_url: Some("wss://test.relay".to_string()),
            minutes_back: 5,
        };
        assert_eq!(params.minutes_back, 5);
        assert_eq!(params.relay_url, Some("wss://test.relay".to_string()));
    }

    #[test]
    fn test_read_recent_messages_params_defaults() {
        let params = ReadRecentMessagesParams {
            pubkey: "02".repeat(32),
            relay_url: None,
            minutes_back: default_minutes(),
        };
        assert_eq!(params.minutes_back, 10);
        assert!(params.relay_url.is_none());
    }

    #[tokio::test]
    async fn test_server_creation() {
        let server = NostrRouter::new();
        let info = server.get_info();
        assert_eq!(info.server_info.name, "goose-nostr");
        assert!(info.instructions.is_some());
        assert!(info
            .instructions
            .unwrap()
            .contains("Nostr protocol operations"));
        assert!(info.capabilities.tools.is_some());
    }

    #[tokio::test]
    async fn test_generate_keypair_without_private_key() {
        let router = NostrRouter::new();
        let params = Parameters(GenerateKeyPairParams {
            include_private_key: false,
        });

        let result = router.generate_keypair(params).await;
        assert!(result.is_ok());

        let tool_result = result.unwrap();
        assert_eq!(tool_result.content.len(), 1);

        // Check it returns text content
        if let RawContent::Text(text_content) = &*tool_result.content[0] {
            let response: serde_json::Value = serde_json::from_str(&text_content.text).unwrap();
            assert!(response.get("public_key").is_some());
            assert!(response.get("public_key_bech32").is_some());
            assert!(response.get("private_key").unwrap().is_null());
            assert!(response.get("private_key_bech32").unwrap().is_null());
        } else {
            panic!("Expected text content");
        }
    }

    #[tokio::test]
    async fn test_generate_keypair_with_private_key() {
        let router = NostrRouter::new();
        let params = Parameters(GenerateKeyPairParams {
            include_private_key: true,
        });

        let result = router.generate_keypair(params).await;
        assert!(result.is_ok());

        let tool_result = result.unwrap();
        assert_eq!(tool_result.content.len(), 1);

        // Check it returns text content
        if let RawContent::Text(text_content) = &*tool_result.content[0] {
            let response: serde_json::Value = serde_json::from_str(&text_content.text).unwrap();
            assert!(response.get("public_key").is_some());
            assert!(response.get("public_key_bech32").is_some());
            assert!(response.get("private_key").is_some());
            assert!(response.get("private_key_bech32").is_some());
            assert!(!response.get("private_key").unwrap().is_null());
            assert!(!response.get("private_key_bech32").unwrap().is_null());
        } else {
            panic!("Expected text content");
        }
    }

    #[tokio::test]
    async fn test_read_recent_messages_invalid_pubkey() {
        let router = NostrRouter::new();
        let params = Parameters(ReadRecentMessagesParams {
            pubkey: "invalid_pubkey".to_string(),
            relay_url: None,
            minutes_back: 10,
        });

        let result = router.read_recent_messages(params).await;
        assert!(result.is_err());

        if let Err(error) = result {
            assert_eq!(error.code, ErrorCode::INVALID_PARAMS);
            assert!(error.message.contains("Invalid public key"));
        }
    }

    #[test]
    fn test_validate_string_param() {
        let valid_params = json!({
            "test_param": "test_value"
        });
        let result = validate_string_param(&valid_params, "test_param");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test_value");
    }

    #[test]
    fn test_validate_string_param_missing() {
        let params = json!({});
        let result = validate_string_param(&params, "missing_param");
        assert!(result.is_err());

        if let Err(error) = result {
            assert_eq!(error.code, ErrorCode::INVALID_PARAMS);
            assert!(error.message.contains("Missing 'missing_param' parameter"));
        }
    }

    #[test]
    fn test_validate_string_param_wrong_type() {
        let params = json!({
            "test_param": 123
        });
        let result = validate_string_param(&params, "test_param");
        assert!(result.is_err());

        if let Err(error) = result {
            assert_eq!(error.code, ErrorCode::INVALID_PARAMS);
            assert!(error.message.contains("must be a string"));
        }
    }

    #[test]
    fn test_nostr_message_serialization() {
        let message = NostrMessage {
            id: "test_id".to_string(),
            pubkey: "test_pubkey".to_string(),
            created_at: 1234567890,
            kind: 1,
            content: "Hello Nostr!".to_string(),
            sig: "test_signature".to_string(),
        };

        let serialized = serde_json::to_string(&message).unwrap();
        let deserialized: NostrMessage = serde_json::from_str(&serialized).unwrap();

        assert_eq!(message.id, deserialized.id);
        assert_eq!(message.content, deserialized.content);
        assert_eq!(message.kind, deserialized.kind);
    }

    #[test]
    fn test_keypair_result_serialization() {
        let result = KeyPairResult {
            public_key: "test_pub".to_string(),
            public_key_bech32: "npub_test".to_string(),
            private_key: Some("test_priv".to_string()),
            private_key_bech32: Some("nsec_test".to_string()),
        };

        let serialized = serde_json::to_string(&result).unwrap();
        let deserialized: KeyPairResult = serde_json::from_str(&serialized).unwrap();

        assert_eq!(result.public_key, deserialized.public_key);
        assert_eq!(result.private_key, deserialized.private_key);
    }

    #[test]
    fn test_publish_message_params() {
        let params = PublishMessageParams {
            content: "Hello Nostr!".to_string(),
            private_key: "test_key".to_string(),
            relay_url: Some("wss://test.relay".to_string()),
        };
        assert_eq!(params.content, "Hello Nostr!");
        assert_eq!(params.private_key, "test_key");
        assert_eq!(params.relay_url, Some("wss://test.relay".to_string()));
    }

    #[test]
    fn test_publish_message_result_serialization() {
        let result = PublishMessageResult {
            event_id: "test_event_id".to_string(),
            author_pubkey: "test_pubkey".to_string(),
            relay_url: "wss://test.relay".to_string(),
            content: "Hello Nostr!".to_string(),
            created_at: 1234567890,
            success: true,
        };

        let serialized = serde_json::to_string(&result).unwrap();
        let deserialized: PublishMessageResult = serde_json::from_str(&serialized).unwrap();

        assert_eq!(result.event_id, deserialized.event_id);
        assert_eq!(result.content, deserialized.content);
        assert_eq!(result.success, deserialized.success);
    }

    #[tokio::test]
    async fn test_publish_message_invalid_private_key() {
        let router = NostrRouter::new();
        let params = Parameters(PublishMessageParams {
            content: "Hello Nostr!".to_string(),
            private_key: "invalid_key".to_string(),
            relay_url: None,
        });

        let result = router.publish_message(params).await;
        assert!(result.is_err());

        if let Err(error) = result {
            assert_eq!(error.code, ErrorCode::INVALID_PARAMS);
            assert!(error.message.contains("Invalid"));
        }
    }
}
