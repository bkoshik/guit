use crate::{Branches, CommitInfo};

impl CommitInfo {
    pub fn branches(&self) -> &Branches {
        &self.branches
    }
}