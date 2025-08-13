use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallToolRequest {
    name: String,
    parameters: serde_json::Value,
}

impl CallToolRequest {
    pub fn new(name: String, parameters: serde_json::Value) -> Self {
        Self { name, parameters }
    }

    fn matches(&self, other: &CallToolRequest) -> bool {
        goose::call_tool::name(&self) == goose::call_tool::name(&other) && self.parameters == other.parameters
    }
}

#[derive(Debug)]
pub struct ToolMonitor {
    max_repetitions: Option<u32>,
    last_call: Option<CallToolRequest>,
    repeat_count: u32,
    call_counts: HashMap<String, u32>,
}

impl ToolMonitor {
    pub fn new(max_repetitions: Option<u32>) -> Self {
        Self {
            max_repetitions,
            last_call: None,
            repeat_count: 0,
            call_counts: HashMap::new(),
        }
    }

    pub fn check_tool_call(&mut self, tool_call: CallToolRequest) -> bool {
        let total_calls = self.call_counts.entry(goose::call_tool::name(&tool_call).clone()).or_insert(0);
        *total_calls += 1;

        if self.max_repetitions.is_none() {
            self.last_call = Some(tool_call);
            self.repeat_count = 1;
            return true;
        }

        if let Some(last) = &self.last_call {
            if last.matches(&tool_call) {
                self.repeat_count += 1;
                if self.repeat_count > self.max_repetitions.unwrap() {
                    return false;
                }
            } else {
                self.repeat_count = 1;
            }
        } else {
            self.repeat_count = 1;
        }

        self.last_call = Some(tool_call);
        true
    }

    pub fn get_stats(&self) -> HashMap<String, u32> {
        self.call_counts.clone()
    }

    pub fn reset(&mut self) {
        self.last_call = None;
        self.repeat_count = 0;
        self.call_counts.clear();
    }
}
