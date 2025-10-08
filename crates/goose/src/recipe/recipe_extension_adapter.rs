use crate::agents::extension::{Envs, ExtensionConfig};
use rmcp::model::Tool;
use serde::de::Deserializer;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
#[serde(tag = "type")]
enum RecipeExtensionConfigInternal {
    #[serde(rename = "sse")]
    Sse {
        name: String,
        #[serde(default)]
        description: Option<String>,
        uri: String,
        #[serde(default)]
        envs: Envs,
        #[serde(default)]
        env_keys: Vec<String>,
        timeout: Option<u64>,
        #[serde(default)]
        bundled: Option<bool>,
        #[serde(default)]
        available_tools: Vec<String>,
    },
    #[serde(rename = "stdio")]
    Stdio {
        name: String,
        #[serde(default)]
        description: Option<String>,
        cmd: String,
        args: Vec<String>,
        #[serde(default)]
        envs: Envs,
        #[serde(default)]
        env_keys: Vec<String>,
        timeout: Option<u64>,
        #[serde(default)]
        bundled: Option<bool>,
        #[serde(default)]
        available_tools: Vec<String>,
    },
    #[serde(rename = "builtin")]
    Builtin {
        name: String,
        #[serde(default)]
        description: Option<String>,
        display_name: Option<String>,
        timeout: Option<u64>,
        #[serde(default)]
        bundled: Option<bool>,
        #[serde(default)]
        available_tools: Vec<String>,
    },
    #[serde(rename = "platform")]
    Platform {
        name: String,
        #[serde(default)]
        description: Option<String>,
        #[serde(default)]
        bundled: Option<bool>,
        #[serde(default)]
        available_tools: Vec<String>,
    },
    #[serde(rename = "streamable_http")]
    StreamableHttp {
        name: String,
        #[serde(default)]
        description: Option<String>,
        uri: String,
        #[serde(default)]
        envs: Envs,
        #[serde(default)]
        env_keys: Vec<String>,
        #[serde(default)]
        headers: HashMap<String, String>,
        timeout: Option<u64>,
        #[serde(default)]
        bundled: Option<bool>,
        #[serde(default)]
        available_tools: Vec<String>,
    },
    #[serde(rename = "frontend")]
    Frontend {
        name: String,
        #[serde(default)]
        description: Option<String>,
        tools: Vec<Tool>,
        instructions: Option<String>,
        #[serde(default)]
        bundled: Option<bool>,
        #[serde(default)]
        available_tools: Vec<String>,
    },
    #[serde(rename = "inline_python")]
    InlinePython {
        name: String,
        #[serde(default)]
        description: Option<String>,
        code: String,
        timeout: Option<u64>,
        #[serde(default)]
        dependencies: Option<Vec<String>>,
        #[serde(default)]
        available_tools: Vec<String>,
    },
}

impl From<RecipeExtensionConfigInternal> for ExtensionConfig {
    fn from(internal_variant: RecipeExtensionConfigInternal) -> Self {
        match internal_variant {
            RecipeExtensionConfigInternal::Sse {
                name,
                description,
                uri,
                envs,
                env_keys,
                timeout,
                bundled,
                available_tools,
            } => ExtensionConfig::Sse {
                name,
                description: description.unwrap_or_default(),
                uri,
                envs,
                env_keys,
                timeout,
                bundled,
                available_tools,
            },
            RecipeExtensionConfigInternal::Stdio {
                name,
                description,
                cmd,
                args,
                envs,
                env_keys,
                timeout,
                bundled,
                available_tools,
            } => ExtensionConfig::Stdio {
                name,
                description: description.unwrap_or_default(),
                cmd,
                args,
                envs,
                env_keys,
                timeout,
                bundled,
                available_tools,
            },
            RecipeExtensionConfigInternal::Builtin {
                name,
                description,
                display_name,
                timeout,
                bundled,
                available_tools,
            } => ExtensionConfig::Builtin {
                name,
                description: description.unwrap_or_default(),
                display_name,
                timeout,
                bundled,
                available_tools,
            },
            RecipeExtensionConfigInternal::Platform {
                name,
                description,
                bundled,
                available_tools,
            } => ExtensionConfig::Platform {
                name,
                description: description.unwrap_or_default(),
                bundled,
                available_tools,
            },
            RecipeExtensionConfigInternal::StreamableHttp {
                name,
                description,
                uri,
                envs,
                env_keys,
                headers,
                timeout,
                bundled,
                available_tools,
            } => ExtensionConfig::StreamableHttp {
                name,
                description: description.unwrap_or_default(),
                uri,
                envs,
                env_keys,
                headers,
                timeout,
                bundled,
                available_tools,
            },
            RecipeExtensionConfigInternal::Frontend {
                name,
                description,
                tools,
                instructions,
                bundled,
                available_tools,
            } => ExtensionConfig::Frontend {
                name,
                description: description.unwrap_or_default(),
                tools,
                instructions,
                bundled,
                available_tools,
            },
            RecipeExtensionConfigInternal::InlinePython {
                name,
                description,
                code,
                timeout,
                dependencies,
                available_tools,
            } => ExtensionConfig::InlinePython {
                name,
                description: description.unwrap_or_default(),
                code,
                timeout,
                dependencies,
                available_tools,
            },
        }
    }
}

pub fn deserialize_recipe_extensions<'de, D>(
    deserializer: D,
) -> Result<Option<Vec<ExtensionConfig>>, D::Error>
where
    D: Deserializer<'de>,
{
    let remotes = Option::<Vec<RecipeExtensionConfigInternal>>::deserialize(deserializer)?;
    Ok(remotes.map(|items| {
        items
            .into_iter()
            .map(ExtensionConfig::from)
            .collect::<Vec<_>>()
    }))
}
