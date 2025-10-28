use crate::{Branches, RepositoryInfo};
use git2::Repository;
use std::error::Error;

impl RepositoryInfo {
    pub fn new(repo: Repository) -> Result<Self, Box<dyn Error>> {
        let name = repo
            .path()
            .parent()
            .unwrap_or("".as_ref())
            .file_name()
            .unwrap_or("".as_ref())
            .to_str()
            .map(|s| s.to_string())
            .unwrap_or("".to_string());
        let branches = Branches::new(&repo)?;

        Ok(Self {
            name,
            branches,
            repo,
        })
    }
}
