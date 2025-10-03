use super::base::Usage;
use super::errors::GoogleErrorCode;
use crate::model::ModelConfig;
use anyhow::Result;
use base64::Engine;
use regex::Regex;
use reqwest::{Response, StatusCode};
use rmcp::model::{AnnotateAble, ImageContent, RawImageContent};
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use std::io::Read;
use std::path::Path;
use std::time::Duration;

use crate::providers::errors::{OpenAIError, ProviderError};

#[derive(serde::Deserialize)]
struct OpenAIErrorResponse {
    error: OpenAIError,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum ImageFormat {
    OpenAi,
    Anthropic,
}

/// Convert an image content into an image json based on format
pub fn convert_image(image: &ImageContent, image_format: &ImageFormat) -> Value {
    match image_format {
        ImageFormat::OpenAi => json!({
            "type": "image_url",
            "image_url": {
                "url": format!("data:{};base64,{}", image.mime_type, image.data)
            }
        }),
        ImageFormat::Anthropic => json!({
            "type": "image",
            "source": {
                "type": "base64",
                "media_type": image.mime_type,
                "data": image.data,
            }
        }),
    }
}

fn check_context_length_exceeded(text: &str) -> bool {
    let check_phrases = [
        "too long",
        "context length",
        "context_length_exceeded",
        "reduce the length",
        "token count",
        "exceeds",
        "exceed context limit",
        "input length",
        "max_tokens",
        "decrease input length",
        "context limit",
    ];
    let text_lower = text.to_lowercase();
    check_phrases
        .iter()
        .any(|phrase| text_lower.contains(phrase))
}

pub fn map_http_error_to_provider_error(
    status: StatusCode,
    payload: Option<Value>,
) -> ProviderError {
    let error = match status {
        StatusCode::OK => unreachable!("Should not call this function with OK status"),
        StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => {
            let message = format!(
                "Authentication failed. Please ensure your API keys are valid and have the required permissions. \
        Status: {}{}",
                status,
                payload.as_ref().map(|p| format!(". Response: {:?}", p)).unwrap_or_default()
            );
            ProviderError::Authentication(message)
        }
        StatusCode::BAD_REQUEST => {
            let mut error_msg = "Unknown error".to_string();
            if let Some(payload) = &payload {
                let payload_str = payload.to_string();
                if check_context_length_exceeded(&payload_str) {
                    ProviderError::ContextLengthExceeded(payload_str)
                } else {
                    if let Some(error) = payload.get("error") {
                        error_msg = error
                            .get("message")
                            .and_then(|m| m.as_str())
                            .unwrap_or("Unknown error")
                            .to_string();
                    }
                    ProviderError::RequestFailed(format!(
                        "Request failed with status: {}. Message: {}",
                        status, error_msg
                    ))
                }
            } else {
                ProviderError::RequestFailed(format!(
                    "Request failed with status: {}. Message: {}",
                    status, error_msg
                ))
            }
        }
        StatusCode::TOO_MANY_REQUESTS => ProviderError::RateLimitExceeded {
            details: format!("{:?}", payload),
            retry_delay: None,
        },
        _ if status.is_server_error() => ProviderError::ServerError(format!("{:?}", payload)),
        _ => ProviderError::RequestFailed(format!("Request failed with status: {}", status)),
    };

    if !status.is_success() {
        tracing::warn!(
            "Provider request failed with status: {}. Payload: {:?}. Returning error: {:?}",
            status,
            payload,
            error
        );
    }

    error
}

/// Handles HTTP responses from OpenAI-compatible endpoints.
///
/// Returns the response if status is OK; otherwise, reads the body and maps to a `ProviderError`,
/// with special handling for context length exceeded and other OpenAI-formatted errors.
///
/// ### References
/// - Error Codes: https://platform.openai.com/docs/guides/error-codes
/// - Context Window Exceeded: https://community.openai.com/t/help-needed-tackling-context-length-limits-in-openai-models/617543
///
/// ### Arguments
/// - `response`: The HTTP response to process.
///
/// ### Returns
/// - `Ok(Response)`: The original response on success.
/// - `Err(ProviderError)`: Describes the failure reason.```
pub async fn handle_status_openai_compat(response: Response) -> Result<Response, ProviderError> {
    let status = response.status();
    if status == StatusCode::OK {
        return Ok(response);
    }

    let body_str = response
        .text()
        .await
        .map_err(|_| map_http_error_to_provider_error(status, None))?;

    if matches!(status, StatusCode::BAD_REQUEST | StatusCode::NOT_FOUND) {
        if let Ok(err_resp) = serde_json::from_str::<OpenAIErrorResponse>(&body_str) {
            let err = err_resp.error;
            if err.is_context_length_exceeded() {
                return Err(ProviderError::ContextLengthExceeded(
                    err.message.unwrap_or("Unknown error".to_string()),
                ));
            } else {
                return Err(ProviderError::RequestFailed(format!(
                    "{} (status {})",
                    err,
                    status.as_u16()
                )));
            }
        }
    }

    let payload = serde_json::from_str::<Value>(&body_str).ok();
    Err(map_http_error_to_provider_error(status, payload))
}

pub async fn handle_response_openai_compat(response: Response) -> Result<Value, ProviderError> {
    let response = handle_status_openai_compat(response).await?;

    response.json::<Value>().await.map_err(|e| {
        ProviderError::RequestFailed(format!("Response body is not valid JSON: {}", e))
    })
}

/// Check if the model is a Google model based on the "model" field in the payload.
///
/// ### Arguments
/// - `payload`: The JSON payload as a `serde_json::Value`.
///
/// ### Returns
/// - `bool`: Returns `true` if the model is a Google model, otherwise `false`.
pub fn is_google_model(payload: &Value) -> bool {
    if let Some(model) = payload.get("model").and_then(|m| m.as_str()) {
        // Check if the model name contains "google"
        return model.to_lowercase().contains("google");
    }
    false
}

/// Extracts `StatusCode` from response status or payload error code.
/// This function first checks the status code of the response. If the status is successful (2xx),
/// it then checks the payload for any error codes and maps them to appropriate `StatusCode`.
/// If the status is not successful (e.g., 4xx or 5xx), the original status code is returned.
fn get_google_final_status(status: StatusCode, payload: Option<&Value>) -> StatusCode {
    // If the status is successful, check for an error in the payload
    if status.is_success() {
        if let Some(payload) = payload {
            if let Some(error) = payload.get("error") {
                if let Some(code) = error.get("code").and_then(|c| c.as_u64()) {
                    if let Some(google_error) = GoogleErrorCode::from_code(code) {
                        return google_error.to_status_code();
                    }
                }
            }
        }
    }
    status
}

fn parse_google_retry_delay(payload: &Value) -> Option<Duration> {
    payload
        .get("error")
        .and_then(|error| error.get("details"))
        .and_then(|details| details.as_array())
        .and_then(|details_array| {
            details_array.iter().find_map(|detail| {
                if detail
                    .get("@type")
                    .and_then(|t| t.as_str())
                    .is_some_and(|s| s.ends_with("RetryInfo"))
                {
                    detail
                        .get("retryDelay")
                        .and_then(|delay| delay.as_str())
                        .and_then(|s| s.strip_suffix('s'))
                        .and_then(|num| num.parse::<u64>().ok())
                        .map(Duration::from_secs)
                } else {
                    None
                }
            })
        })
}

/// Handle response from Google Gemini API-compatible endpoints.
///
/// Processes HTTP responses, handling specific statuses and parsing the payload
/// for error messages. Logs the response payload for debugging purposes.
///
/// ### References
/// - Error Codes: https://ai.google.dev/gemini-api/docs/troubleshooting?lang=python
///
/// ### Arguments
/// - `response`: The HTTP response to process.
///
/// ### Returns
/// - `Ok(Value)`: Parsed JSON on success.
/// - `Err(ProviderError)`: Describes the failure reason.
pub async fn handle_response_google_compat(response: Response) -> Result<Value, ProviderError> {
    let status = response.status();
    let payload: Option<Value> = response.json().await.ok();
    let final_status = get_google_final_status(status, payload.as_ref());

    match final_status {
        StatusCode::OK =>  payload.ok_or_else( || ProviderError::RequestFailed("Response body is not valid JSON".to_string()) ),
        StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => {
            Err(ProviderError::Authentication(format!("Authentication failed. Please ensure your API keys are valid and have the required permissions. \
                Status: {}. Response: {:?}", final_status, payload )))
        }
        StatusCode::BAD_REQUEST | StatusCode::NOT_FOUND => {
            let mut error_msg = "Unknown error".to_string();
            if let Some(payload) = &payload {
                if let Some(error) = payload.get("error") {
                    error_msg = error.get("message").and_then(|m| m.as_str()).unwrap_or("Unknown error").to_string();
                    let error_status = error.get("status").and_then(|s| s.as_str()).unwrap_or("Unknown status");
                    if error_status == "INVALID_ARGUMENT" && error_msg.to_lowercase().contains("exceeds") {
                        return Err(ProviderError::ContextLengthExceeded(error_msg.to_string()));
                    }
                }
            }
            tracing::debug!(
                "{}", format!("Provider request failed with status: {}. Payload: {:?}", final_status, payload)
            );
            Err(ProviderError::RequestFailed(format!("Request failed with status: {}. Message: {}", final_status, error_msg)))
        }
        StatusCode::TOO_MANY_REQUESTS => {
            let retry_delay = payload.as_ref().and_then(parse_google_retry_delay);
            Err(ProviderError::RateLimitExceeded {
                details: format!("{:?}", payload),
                retry_delay,
            })
        }
        _ if final_status.is_server_error() => {
            Err(ProviderError::ServerError(format!("{:?}", payload)))
        }
        StatusCode::INTERNAL_SERVER_ERROR | StatusCode::SERVICE_UNAVAILABLE => {
            Err(ProviderError::ServerError(format!("{:?}", payload)))
        }
        _ => {
            tracing::debug!(
                "{}", format!("Provider request failed with status: {}. Payload: {:?}", final_status, payload)
            );
            Err(ProviderError::RequestFailed(format!("Request failed with status: {}", final_status)))
        }
    }
}

pub fn sanitize_function_name(name: &str) -> String {
    let re = Regex::new(r"[^a-zA-Z0-9_-]").unwrap();
    re.replace_all(name, "_").to_string()
}

pub fn is_valid_function_name(name: &str) -> bool {
    let re = Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap();
    re.is_match(name)
}

/// Extract the model name from a JSON object. Common with most providers to have this top level attribute.
pub fn get_model(data: &Value) -> String {
    if let Some(model) = data.get("model") {
        if let Some(model_str) = model.as_str() {
            model_str.to_string()
        } else {
            "Unknown".to_string()
        }
    } else {
        "Unknown".to_string()
    }
}

/// Check if a file is actually an image by examining its magic bytes
fn is_image_file(path: &Path) -> bool {
    if let Ok(mut file) = std::fs::File::open(path) {
        let mut buffer = [0u8; 8]; // Large enough for most image magic numbers
        if file.read(&mut buffer).is_ok() {
            // Check magic numbers for common image formats
            return match &buffer[0..4] {
                // PNG: 89 50 4E 47
                [0x89, 0x50, 0x4E, 0x47] => true,
                // JPEG: FF D8 FF
                [0xFF, 0xD8, 0xFF, _] => true,
                // GIF: 47 49 46 38
                [0x47, 0x49, 0x46, 0x38] => true,
                _ => false,
            };
        }
    }
    false
}

/// Detect if a string contains a path to an image file
pub fn detect_image_path(text: &str) -> Option<&str> {
    // Basic image file extension check
    let extensions = [".png", ".jpg", ".jpeg"];

    // Find any word that ends with an image extension
    for word in text.split_whitespace() {
        if extensions
            .iter()
            .any(|ext| word.to_lowercase().ends_with(ext))
        {
            let path = Path::new(word);
            // Check if it's an absolute path and file exists
            if path.is_absolute() && path.is_file() {
                // Verify it's actually an image file
                if is_image_file(path) {
                    return Some(word);
                }
            }
        }
    }
    None
}

/// Convert a local image file to base64 encoded ImageContent
pub fn load_image_file(path: &str) -> Result<ImageContent, ProviderError> {
    let path = Path::new(path);

    // Verify it's an image before proceeding
    if !is_image_file(path) {
        return Err(ProviderError::RequestFailed(
            "File is not a valid image".to_string(),
        ));
    }

    // Read the file
    let bytes = std::fs::read(path)
        .map_err(|e| ProviderError::RequestFailed(format!("Failed to read image file: {}", e)))?;

    // Detect mime type from extension
    let mime_type = match path.extension().and_then(|e| e.to_str()) {
        Some(ext) => match ext.to_lowercase().as_str() {
            "png" => "image/png",
            "jpg" | "jpeg" => "image/jpeg",
            _ => {
                return Err(ProviderError::RequestFailed(
                    "Unsupported image format".to_string(),
                ))
            }
        },
        None => {
            return Err(ProviderError::RequestFailed(
                "Unknown image format".to_string(),
            ))
        }
    };

    // Convert to base64
    let data = base64::prelude::BASE64_STANDARD.encode(&bytes);

    Ok(RawImageContent {
        mime_type: mime_type.to_string(),
        data,
        meta: None,
    }
    .no_annotation())
}

pub fn unescape_json_values(value: &Value) -> Value {
    match value {
        Value::Object(map) => {
            let new_map: Map<String, Value> = map
                .iter()
                .map(|(k, v)| (k.clone(), unescape_json_values(v))) // Process each value
                .collect();
            Value::Object(new_map)
        }
        Value::Array(arr) => {
            let new_array: Vec<Value> = arr.iter().map(unescape_json_values).collect();
            Value::Array(new_array)
        }
        Value::String(s) => {
            let unescaped = s
                .replace("\\\\n", "\n")
                .replace("\\\\t", "\t")
                .replace("\\\\r", "\r")
                .replace("\\\\\"", "\"")
                .replace("\\n", "\n")
                .replace("\\t", "\t")
                .replace("\\r", "\r")
                .replace("\\\"", "\"");
            Value::String(unescaped)
        }
        _ => value.clone(),
    }
}

pub fn emit_debug_trace<T1, T2>(
    model_config: &ModelConfig,
    payload: &T1,
    response: &T2,
    usage: &Usage,
) where
    T1: ?Sized + Serialize,
    T2: ?Sized + Serialize,
{
    tracing::debug!(
        model_config = %serde_json::to_string_pretty(model_config).unwrap_or_default(),
        input = %serde_json::to_string_pretty(payload).unwrap_or_default(),
        output = %serde_json::to_string_pretty(response).unwrap_or_default(),
        input_tokens = ?usage.input_tokens.unwrap_or_default(),
        output_tokens = ?usage.output_tokens.unwrap_or_default(),
        total_tokens = ?usage.total_tokens.unwrap_or_default(),
    );
}

/// Safely parse a JSON string that may contain doubly-encoded or malformed JSON.
/// This function first attempts to parse the input string as-is. If that fails,
/// it applies control character escaping and tries again.
///
/// This approach preserves valid JSON like `{"key1": "value1",\n"key2": "value"}`
/// (which contains a literal \n but is perfectly valid JSON) while still fixing
/// broken JSON like `{"key1": "value1\n","key2": "value"}` (which contains an
/// unescaped newline character).
pub fn safely_parse_json(s: &str) -> Result<serde_json::Value, serde_json::Error> {
    // First, try parsing the string as-is
    match serde_json::from_str(s) {
        Ok(value) => Ok(value),
        Err(_) => {
            // If that fails, try with control character escaping
            let escaped = json_escape_control_chars_in_string(s);
            serde_json::from_str(&escaped)
        }
    }
}

/// Helper to escape control characters in a string that is supposed to be a JSON document.
/// This function iterates through the input string `s` and replaces any literal
/// control characters (U+0000 to U+001F) with their JSON-escaped equivalents
/// (e.g., '\n' becomes "\\n", '\u0001' becomes "\\u0001").
///
/// It does NOT escape quotes (") or backslashes (\) because it assumes `s` is a
/// full JSON document, and these characters might be structural (e.g., object delimiters,
/// existing valid escape sequences). The goal is to fix common LLM errors where
/// control characters are emitted raw into what should be JSON string values,
/// making the overall JSON structure unparsable.
///
/// If the input string `s` has other JSON syntax errors (e.g., an unescaped quote
/// *within* a string value like `{"key": "string with " quote"}`), this function
/// will not fix them. It specifically targets unescaped control characters.
pub fn json_escape_control_chars_in_string(s: &str) -> String {
    let mut r = String::with_capacity(s.len()); // Pre-allocate for efficiency
    for c in s.chars() {
        match c {
            // ASCII Control characters (U+0000 to U+001F)
            '\u{0000}'..='\u{001F}' => {
                match c {
                    '\u{0008}' => r.push_str("\\b"), // Backspace
                    '\u{000C}' => r.push_str("\\f"), // Form feed
                    '\n' => r.push_str("\\n"),       // Line feed
                    '\r' => r.push_str("\\r"),       // Carriage return
                    '\t' => r.push_str("\\t"),       // Tab
                    // Other control characters (e.g., NUL, SOH, VT, etc.)
                    // that don't have a specific short escape sequence.
                    _ => {
                        r.push_str(&format!("\\u{:04x}", c as u32));
                    }
                }
            }
            // Other characters are passed through.
            // This includes quotes (") and backslashes (\). If these are part of the
            // JSON structure (e.g. {"key": "value"}) or part of an already correctly
            // escaped sequence within a string value (e.g. "string with \\\" quote"),
            // they are preserved as is. This function does not attempt to fix
            // malformed quote or backslash usage *within* string values if the LLM
            // generates them incorrectly (e.g. {"key": "unescaped " quote in string"}).
            _ => r.push(c),
        }
    }
    r
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::{matchers, Mock, MockServer, ResponseTemplate};

    #[test]
    fn test_detect_image_path() {
        // Create a temporary PNG file with valid PNG magic numbers
        let temp_dir = tempfile::tempdir().unwrap();
        let png_path = temp_dir.path().join("test.png");
        let png_data = [
            0x89, 0x50, 0x4E, 0x47, // PNG magic number
            0x0D, 0x0A, 0x1A, 0x0A, // PNG header
            0x00, 0x00, 0x00, 0x0D, // Rest of fake PNG data
        ];
        std::fs::write(&png_path, png_data).unwrap();
        let png_path_str = png_path.to_str().unwrap();

        // Create a fake PNG (wrong magic numbers)
        let fake_png_path = temp_dir.path().join("fake.png");
        std::fs::write(&fake_png_path, b"not a real png").unwrap();

        // Test with valid PNG file using absolute path
        let text = format!("Here is an image {}", png_path_str);
        assert_eq!(detect_image_path(&text), Some(png_path_str));

        // Test with non-image file that has .png extension
        let text = format!("Here is a fake image {}", fake_png_path.to_str().unwrap());
        assert_eq!(detect_image_path(&text), None);

        // Test with non-existent file
        let text = "Here is a fake.png that doesn't exist";
        assert_eq!(detect_image_path(text), None);

        // Test with non-image file
        let text = "Here is a file.txt";
        assert_eq!(detect_image_path(text), None);

        // Test with relative path (should not match)
        let text = "Here is a relative/path/image.png";
        assert_eq!(detect_image_path(text), None);
    }

    #[test]
    fn test_load_image_file() {
        // Create a temporary PNG file with valid PNG magic numbers
        let temp_dir = tempfile::tempdir().unwrap();
        let png_path = temp_dir.path().join("test.png");
        let png_data = [
            0x89, 0x50, 0x4E, 0x47, // PNG magic number
            0x0D, 0x0A, 0x1A, 0x0A, // PNG header
            0x00, 0x00, 0x00, 0x0D, // Rest of fake PNG data
        ];
        std::fs::write(&png_path, png_data).unwrap();
        let png_path_str = png_path.to_str().unwrap();

        // Create a fake PNG (wrong magic numbers)
        let fake_png_path = temp_dir.path().join("fake.png");
        std::fs::write(&fake_png_path, b"not a real png").unwrap();
        let fake_png_path_str = fake_png_path.to_str().unwrap();

        // Test loading valid PNG file
        let result = load_image_file(png_path_str);
        assert!(result.is_ok());
        let image = result.unwrap();
        assert_eq!(image.mime_type, "image/png");

        // Test loading fake PNG file
        let result = load_image_file(fake_png_path_str);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("not a valid image"));

        // Test non-existent file
        let result = load_image_file("nonexistent.png");
        assert!(result.is_err());

        // Create a GIF file with valid header bytes
        let gif_path = temp_dir.path().join("test.gif");
        // Minimal GIF89a header
        let gif_data = [0x47, 0x49, 0x46, 0x38, 0x39, 0x61];
        std::fs::write(&gif_path, gif_data).unwrap();
        let gif_path_str = gif_path.to_str().unwrap();

        // Test loading unsupported GIF format
        let result = load_image_file(gif_path_str);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Unsupported image format"));
    }

    #[test]
    fn test_sanitize_function_name() {
        assert_eq!(sanitize_function_name("hello-world"), "hello-world");
        assert_eq!(sanitize_function_name("hello world"), "hello_world");
        assert_eq!(sanitize_function_name("hello@world"), "hello_world");
    }

    #[test]
    fn test_is_valid_function_name() {
        assert!(is_valid_function_name("hello-world"));
        assert!(is_valid_function_name("hello_world"));
        assert!(!is_valid_function_name("hello world"));
        assert!(!is_valid_function_name("hello@world"));
    }

    #[test]
    fn unescape_json_values_with_object() {
        let value = json!({"text": "Hello\\nWorld"});
        let unescaped_value = unescape_json_values(&value);
        assert_eq!(unescaped_value, json!({"text": "Hello\nWorld"}));
    }

    #[test]
    fn unescape_json_values_with_array() {
        let value = json!(["Hello\\nWorld", "Goodbye\\tWorld"]);
        let unescaped_value = unescape_json_values(&value);
        assert_eq!(unescaped_value, json!(["Hello\nWorld", "Goodbye\tWorld"]));
    }

    #[test]
    fn unescape_json_values_with_string() {
        let value = json!("Hello\\nWorld");
        let unescaped_value = unescape_json_values(&value);
        assert_eq!(unescaped_value, json!("Hello\nWorld"));
    }

    #[test]
    fn unescape_json_values_with_mixed_content() {
        let value = json!({
            "text": "Hello\\nWorld\\\\n!",
            "array": ["Goodbye\\tWorld", "See you\\rlater"],
            "nested": {
                "inner_text": "Inner\\\"Quote\\\""
            }
        });
        let unescaped_value = unescape_json_values(&value);
        assert_eq!(
            unescaped_value,
            json!({
                "text": "Hello\nWorld\n!",
                "array": ["Goodbye\tWorld", "See you\rlater"],
                "nested": {
                    "inner_text": "Inner\"Quote\""
                }
            })
        );
    }

    #[test]
    fn unescape_json_values_with_no_escapes() {
        let value = json!({"text": "Hello World"});
        let unescaped_value = unescape_json_values(&value);
        assert_eq!(unescaped_value, json!({"text": "Hello World"}));
    }

    #[test]
    fn test_is_google_model() {
        // Define the test cases as a vector of tuples
        let test_cases = vec![
            // (input, expected_result)
            (json!({ "model": "google_gemini" }), true),
            (json!({ "model": "microsoft_bing" }), false),
            (json!({ "model": "" }), false),
            (json!({}), false),
            (json!({ "model": "Google_XYZ" }), true),
            (json!({ "model": "google_abc" }), true),
        ];

        // Iterate through each test case and assert the result
        for (payload, expected_result) in test_cases {
            assert_eq!(is_google_model(&payload), expected_result);
        }
    }

    #[test]
    fn test_get_google_final_status_success() {
        let status = StatusCode::OK;
        let payload = json!({});
        let result = get_google_final_status(status, Some(&payload));
        assert_eq!(result, StatusCode::OK);
    }

    #[test]
    fn test_get_google_final_status_with_error_code() {
        // Test error code mappings for different payload error codes
        let test_cases = vec![
            // (error code, status, expected status code)
            (200, None, StatusCode::OK),
            (429, Some(StatusCode::OK), StatusCode::TOO_MANY_REQUESTS),
            (400, Some(StatusCode::OK), StatusCode::BAD_REQUEST),
            (401, Some(StatusCode::OK), StatusCode::UNAUTHORIZED),
            (403, Some(StatusCode::OK), StatusCode::FORBIDDEN),
            (404, Some(StatusCode::OK), StatusCode::NOT_FOUND),
            (500, Some(StatusCode::OK), StatusCode::INTERNAL_SERVER_ERROR),
            (503, Some(StatusCode::OK), StatusCode::SERVICE_UNAVAILABLE),
            (999, Some(StatusCode::OK), StatusCode::INTERNAL_SERVER_ERROR),
            (500, Some(StatusCode::BAD_REQUEST), StatusCode::BAD_REQUEST),
            (
                404,
                Some(StatusCode::INTERNAL_SERVER_ERROR),
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
        ];

        for (error_code, status, expected_status) in test_cases {
            let payload = if let Some(_status) = status {
                json!({
                    "error": {
                        "code": error_code,
                        "message": "Error message"
                    }
                })
            } else {
                json!({})
            };

            let result = get_google_final_status(status.unwrap_or(StatusCode::OK), Some(&payload));
            assert_eq!(result, expected_status);
        }
    }

    #[test]
    fn test_safely_parse_json() {
        // Test valid JSON that should parse without escaping (contains proper escape sequence)
        let valid_json = r#"{"key1": "value1","key2": "value2"}"#;
        let result = safely_parse_json(valid_json).unwrap();
        assert_eq!(result["key1"], "value1");
        assert_eq!(result["key2"], "value2");

        // Test JSON with actual unescaped newlines that needs escaping
        let invalid_json = "{\"key1\": \"value1\n\",\"key2\": \"value2\"}";
        let result = safely_parse_json(invalid_json).unwrap();
        assert_eq!(result["key1"], "value1\n");
        assert_eq!(result["key2"], "value2");

        // Test already valid JSON - should parse on first try
        let good_json = r#"{"test": "value"}"#;
        let result = safely_parse_json(good_json).unwrap();
        assert_eq!(result["test"], "value");

        // Test completely invalid JSON that can't be fixed
        let broken_json = r#"{"key": "unclosed_string"#;
        assert!(safely_parse_json(broken_json).is_err());

        // Test empty object
        let empty_json = "{}";
        let result = safely_parse_json(empty_json).unwrap();
        assert!(result.as_object().unwrap().is_empty());

        // Test JSON with escaped newlines (valid JSON) - should parse on first try
        let escaped_json = r#"{"key": "value with\nnewline"}"#;
        let result = safely_parse_json(escaped_json).unwrap();
        assert_eq!(result["key"], "value with\nnewline");
    }

    #[test]
    fn test_json_escape_control_chars_in_string() {
        // Test basic control character escaping
        assert_eq!(
            json_escape_control_chars_in_string("Hello\nWorld"),
            "Hello\\nWorld"
        );
        assert_eq!(
            json_escape_control_chars_in_string("Hello\tWorld"),
            "Hello\\tWorld"
        );
        assert_eq!(
            json_escape_control_chars_in_string("Hello\rWorld"),
            "Hello\\rWorld"
        );

        // Test multiple control characters
        assert_eq!(
            json_escape_control_chars_in_string("Hello\n\tWorld\r"),
            "Hello\\n\\tWorld\\r"
        );

        // Test that quotes and backslashes are preserved (not escaped)
        assert_eq!(
            json_escape_control_chars_in_string("Hello \"World\""),
            "Hello \"World\""
        );
        assert_eq!(
            json_escape_control_chars_in_string("Hello\\World"),
            "Hello\\World"
        );

        // Test JSON-like string with control characters
        assert_eq!(
            json_escape_control_chars_in_string("{\"message\": \"Hello\nWorld\"}"),
            "{\"message\": \"Hello\\nWorld\"}"
        );

        // Test no changes for normal strings
        assert_eq!(
            json_escape_control_chars_in_string("Hello World"),
            "Hello World"
        );

        // Test other control characters get unicode escapes
        assert_eq!(
            json_escape_control_chars_in_string("Hello\u{0001}World"),
            "Hello\\u0001World"
        );
    }

    #[test]
    fn test_parse_google_retry_delay() {
        let payload = json!({
            "error": {
                "details": [
                    {
                        "@type": "type.googleapis.com/google.rpc.RetryInfo",
                        "retryDelay": "42s"
                    }
                ]
            }
        });
        assert_eq!(
            parse_google_retry_delay(&payload),
            Some(Duration::from_secs(42))
        );
    }

    #[tokio::test]
    async fn test_handle_status_openai_compat() {
        let test_cases = vec![
            // (status_code, body, expected_result)
            // Success case - 200 OK returns response as-is
            (
                200,
                Some(json!({
                "choices": [{
                    "finish_reason": "stop",
                    "index": 0,
                    "message": {
                        "content": "Hi there! How can I help you today?",
                        "role": "assistant"
                    }
                }],
                "created": 1755133833,
                "id": "chatcmpl-test",
                "model": "gpt-5-nano",
                "usage": {
                    "completion_tokens": 10,
                    "prompt_tokens": 8,
                    "total_tokens": 18
                }
            })),
                Ok(()),
            ),
            // 400 Bad Request with OpenAI-formatted error (directly handled)
            (
                400,
                Some(json!({
                "error": {
                    "code": "unsupported_parameter",
                    "message": "Unsupported parameter: 'max_tokens' is not supported with this model. Use 'max_completion_tokens' instead.",
                    "param": "max_tokens",
                    "type": "invalid_request_error"
                }
            })),
                Err(ProviderError::RequestFailed(
                    "Unsupported parameter: 'max_tokens' is not supported with this model. Use 'max_completion_tokens' instead. (code: unsupported_parameter, type: invalid_request_error) (status 400)".to_string(),
                )),
            ),
            // 400 with context_length_exceeded in OpenAI format (directly handled)
            (
                400,
                Some(json!({
                "error": {
                    "code": "context_length_exceeded",
                    "message": "This model's maximum context length is 4096 tokens.",
                    "type": "invalid_request_error"
                }
            })),
                Err(ProviderError::ContextLengthExceeded(
                    "This model's maximum context length is 4096 tokens.".to_string(),
                )),
            ),
            // 404 Not Found with OpenAI-formatted error (directly handled like 400)
            (
                404,
                Some(json!({
                "error": {
                    "code": "model_not_found",
                    "message": "The model 'gpt-5' does not exist",
                    "type": "invalid_request_error"
                }
            })),
                Err(ProviderError::RequestFailed(
                    "The model 'gpt-5' does not exist (code: model_not_found, type: invalid_request_error) (status 404)".to_string(),
                )),
            ),
            // Non-JSON body error (tests parse failure path)
            (
                413,
                Some(Value::String("Payload Too Large".to_string())),
                Err(ProviderError::RequestFailed(
                    "Request failed with status: 413 Payload Too Large".to_string(),
                )),
            ),
        ];

        for (status_code, body, expected_result) in test_cases {
            let mock_server = MockServer::start().await;

            let mut response_template = ResponseTemplate::new(status_code);

            // Set body based on test case
            if let Some(body_value) = body {
                if body_value.is_string() {
                    // For non-JSON bodies (like "Payload Too Large")
                    response_template =
                        response_template.set_body_string(body_value.as_str().unwrap().to_string());
                } else {
                    // For JSON bodies
                    response_template = response_template.set_body_json(&body_value);
                }
            }

            Mock::given(matchers::method("GET"))
                .and(matchers::path("/test"))
                .respond_with(response_template)
                .mount(&mock_server)
                .await;

            // Make request to mock server
            let client = reqwest::Client::new();
            let response = client
                .get(format!("{}/test", &mock_server.uri()))
                .send()
                .await
                .unwrap();

            // Test handle_status_openai_compat
            let result = handle_status_openai_compat(response).await.map(|_| ());

            assert_eq!(result, expected_result, "for status {}", status_code);
        }
    }

    #[test]
    fn test_map_http_error_to_provider_error() {
        let test_cases = vec![
            // UNAUTHORIZED/FORBIDDEN - with payload
            (
                StatusCode::UNAUTHORIZED,
                Some(json!({"error": "auth failed"})),
                ProviderError::Authentication(
                    "Authentication failed. Please ensure your API keys are valid and have the required permissions. Status: 401 Unauthorized. Response: Object {\"error\": String(\"auth failed\")}".to_string(),
                ),
            ),
            // UNAUTHORIZED/FORBIDDEN - without payload
            (
                StatusCode::FORBIDDEN,
                None,
                ProviderError::Authentication(
                    "Authentication failed. Please ensure your API keys are valid and have the required permissions. Status: 403 Forbidden".to_string(),
                ),
            ),
            // BAD_REQUEST - with context_length_exceeded detection
            (
                StatusCode::BAD_REQUEST,
                Some(json!({"error": {"message": "context_length_exceeded"}})),
                ProviderError::ContextLengthExceeded(
                    "{\"error\":{\"message\":\"context_length_exceeded\"}}".to_string(),
                ),
            ),
            // BAD_REQUEST - with error.message extraction
            (
                StatusCode::BAD_REQUEST,
                Some(json!({"error": {"message": "Custom error"}})),
                ProviderError::RequestFailed(
                    "Request failed with status: 400 Bad Request. Message: Custom error".to_string(),
                ),
            ),
            // BAD_REQUEST - without payload
            (
                StatusCode::BAD_REQUEST,
                None,
                ProviderError::RequestFailed(
                    "Request failed with status: 400 Bad Request. Message: Unknown error".to_string(),
                ),
            ),
            // TOO_MANY_REQUESTS
            (
                StatusCode::TOO_MANY_REQUESTS,
                Some(json!({"retry_after": 60})),
                ProviderError::RateLimitExceeded{
                    details: "Some(Object {\"retry_after\": Number(60)})".to_string(),
                    retry_delay: None,
                },
            ),
            // is_server_error() without payload
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                None,
                ProviderError::ServerError("None".to_string()),
            ),
            // is_server_error() with payload
            (
                StatusCode::BAD_GATEWAY,
                Some(json!({"error": "upstream error"})),
                ProviderError::ServerError("Some(Object {\"error\": String(\"upstream error\")})".to_string()),
            ),
            // Default - any other status code
            (
                StatusCode::IM_A_TEAPOT,
                Some(json!({"ignored": "payload"})),
                ProviderError::RequestFailed(
                    "Request failed with status: 418 I'm a teapot".to_string(),
                ),
            ),
        ];

        for (status, payload, expected_error) in test_cases {
            let result = map_http_error_to_provider_error(status, payload);
            assert_eq!(result, expected_error);
        }
    }
}
