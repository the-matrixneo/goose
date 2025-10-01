use etcetera::AppStrategyArgs;
use once_cell::sync::Lazy;

pub static APP_STRATEGY: Lazy<AppStrategyArgs> = Lazy::new(|| AppStrategyArgs {
    top_level_domain: "Block".to_string(),
    author: "Block".to_string(),
    app_name: "goose".to_string(),
});

pub mod autovisualiser;
pub mod computercontroller;
pub mod developer;
pub mod mcp_server_runner;
mod memory;
mod moim;
pub mod tutorial;

pub use autovisualiser::AutoVisualiserRouter;
pub use computercontroller::ComputerControllerServer;
pub use developer::rmcp_developer::DeveloperServer;
pub use memory::MemoryServer;
pub use moim::MoimServer;
pub use tutorial::TutorialServer;
