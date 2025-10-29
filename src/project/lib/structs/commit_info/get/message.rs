use crate::{CommitInfo, Message};

impl CommitInfo {
    pub fn message(&self) -> &Message {
        &self.message
    }
}