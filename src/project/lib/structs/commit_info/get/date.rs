use crate::CommitInfo;
use std::time::SystemTime;

impl CommitInfo {
    pub fn date(&self) -> &SystemTime {
        &self.date
    }
}
