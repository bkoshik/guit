use crate::CommitInfo;

impl CommitInfo {
    pub fn is_head(&self) -> &bool {
        &self.is_head
    }
}
