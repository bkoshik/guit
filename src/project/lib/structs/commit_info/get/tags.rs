use crate::CommitInfo;

impl CommitInfo {
    pub fn tags(&self) -> &Vec<String> {
        &self.tags
    }
}
