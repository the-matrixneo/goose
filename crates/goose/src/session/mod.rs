pub mod extension_data;
pub mod info;
pub mod storage;

// Re-export common session types and functions
pub use storage::{
    ensure_session_dir, flush_background_saves, generate_session_id, get_most_recent_session,
    get_path, list_sessions, persist_messages_background,
    persist_messages_with_schedule_id_background, read_messages, read_metadata,
    shutdown_background_saves, update_metadata, Identifier, SessionMetadata,
};

pub use extension_data::{ExtensionData, ExtensionState, TodoState};
pub use info::{get_valid_sorted_sessions, SessionInfo};
