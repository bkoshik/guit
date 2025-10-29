use std::time::SystemTime;
use crate::CommitInfo;

impl CommitInfo {
    pub fn date(&self) -> &SystemTime {
        &self.date
    }
}