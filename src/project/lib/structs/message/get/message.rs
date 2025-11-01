use crate::Message;

impl Message {
    pub fn message(&self) -> &str {
        &self.message
    }
}
