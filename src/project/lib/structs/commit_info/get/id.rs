use git2::Oid;
use crate::CommitInfo;

impl CommitInfo {
    pub fn id(&self) -> &Oid {
        &self.id
    }
}