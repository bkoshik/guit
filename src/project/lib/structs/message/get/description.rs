use crate::Message;

impl Message {
    pub fn description(&self) -> String {
        self.description.clone()
    }
}