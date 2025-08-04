pub mod anthropic;
pub mod openai;

pub use anthropic::{AnthropicCompatibleConfig, AnthropicCompatibleProvider};
pub use openai::{OpenAICompatibleConfig, OpenAICompatibleProvider};
