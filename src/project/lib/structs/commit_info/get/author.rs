use crate::{AuthorInfo, CommitInfo};

impl CommitInfo {
    pub fn author(&self) -> &AuthorInfo {
        &self.author
    }
}