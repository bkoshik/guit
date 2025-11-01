use crate::Message;

impl Message {
    pub fn new(summary: &str, description: &str) -> Self {
        Self {
            summary: summary.to_string(),
            message: description.to_string(),
        }
    }
}
