use git2::Oid;
use crate::CommitInfo;

impl CommitInfo {
    pub fn parents(&self) -> &Vec<Oid> {
        &self.parents
    }
}