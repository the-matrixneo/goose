// Session state management
// This module will contain session initialization, persistence, and metadata handling

use std::path::PathBuf;

// Placeholder - will be implemented in Phase 3
#[allow(dead_code)]
pub struct SessionState {
    pub session_file: Option<PathBuf>,
    pub scheduled_job_id: Option<String>,
    pub max_turns: Option<u32>,
    pub debug: bool,
}

#[allow(dead_code)]
impl SessionState {
    pub fn new(
        session_file: Option<PathBuf>,
        debug: bool,
        scheduled_job_id: Option<String>,
        max_turns: Option<u32>,
    ) -> Self {
        Self {
            session_file,
            scheduled_job_id,
            max_turns,
            debug,
        }
    }
}
