// Message management
// This module will contain message history, persistence, and processing logic

use goose::message::Message;

// Placeholder - will be implemented in Phase 4
#[allow(dead_code)]
pub struct MessageManager {
    pub messages: Vec<Message>,
}

#[allow(dead_code)]
impl MessageManager {
    pub fn new(messages: Vec<Message>) -> Self {
        Self { messages }
    }

    pub fn get_messages(&self) -> &Vec<Message> {
        &self.messages
    }

    pub fn add_message(&mut self, message: Message) {
        self.messages.push(message);
    }
}
