use crate::RepositoryInfo;

impl RepositoryInfo {
    pub fn name(&self) -> String {
        self.name.clone()
    }
}