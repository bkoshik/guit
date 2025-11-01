use crate::Message;

impl Message {
    pub fn summary(&self) -> &str {
        &self.summary
    }
}
