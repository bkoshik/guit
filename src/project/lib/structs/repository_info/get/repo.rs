use git2::Repository;
use crate::RepositoryInfo;

impl RepositoryInfo {
    pub fn repo(&self) -> &Repository{
        &self.repo
    }
}