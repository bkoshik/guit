use git2::Repository;
use crate::RepositoryInfo;
use crate::utils::helpers::Result;

impl RepositoryInfo {
    pub fn repo(&self) -> Result<Repository> {
        Ok(Repository::open(self.repo.path())?)
    }
}