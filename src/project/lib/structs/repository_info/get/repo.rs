use crate::RepositoryInfo;
use git2::Repository;

impl RepositoryInfo {
    pub fn repo(&self) -> &Repository {
        &self.repo
    }
}
