use crate::CommitInfo;
use git2::Oid;

impl CommitInfo {
    pub fn id(&self) -> &Oid {
        &self.id
    }
}
