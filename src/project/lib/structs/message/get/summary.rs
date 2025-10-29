use crate::Message;

impl Message {
    pub fn summary(&self) -> String {
        self.summary.clone()
    }
}