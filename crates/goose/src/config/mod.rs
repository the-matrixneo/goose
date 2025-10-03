pub mod base;
pub mod custom_providers;
mod experiments;
pub mod extensions;
pub mod permission;
pub mod signup_openrouter;
pub mod signup_tetrate;

pub use crate::agents::ExtensionConfig;
pub use base::{get_config_dir, Config, ConfigError, APP_STRATEGY};
pub use custom_providers::CustomProviderConfig;
pub use experiments::ExperimentManager;
pub use extensions::{ExtensionConfigManager, ExtensionEntry};
pub use permission::PermissionManager;
pub use signup_openrouter::configure_openrouter;
pub use signup_tetrate::configure_tetrate;

pub use extensions::DEFAULT_DISPLAY_NAME;
pub use extensions::DEFAULT_EXTENSION;
pub use extensions::DEFAULT_EXTENSION_DESCRIPTION;
pub use extensions::DEFAULT_EXTENSION_TIMEOUT;
