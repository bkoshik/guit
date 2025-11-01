use crate::RepositoryInfo;
use git2::Repository;
use std::error::Error;

impl RepositoryInfo {
    pub fn new(repo: Repository) -> Result<Self, Box<dyn Error>> {
        let name = repo
            .path()
            .parent()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();

        Ok(Self { name, repo })
    }
}
