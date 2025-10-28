use crate::Message;

impl Message {
    pub fn new(summary: &str, description: &str) -> Self {
        Self {
            summary: summary.to_string(),
            description: description.to_string(),
        }
    }
}