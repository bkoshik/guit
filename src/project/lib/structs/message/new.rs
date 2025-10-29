use crate::Message;

impl Message {
    pub fn new(summary: &str, message: &str) -> Self {
        Self {
            summary: summary.to_string(),
            message: message.to_string(),
        }
    }
}
