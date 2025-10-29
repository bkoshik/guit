use crate::Message;

impl Message {
    pub fn description(&self) -> &str {
        &self.description
    }
}