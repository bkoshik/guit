use crate::CommitInfo;
use git2::Oid;

impl CommitInfo {
    pub fn parents(&self) -> &Vec<Oid> {
        &self.parents
    }
}
