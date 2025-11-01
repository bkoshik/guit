use std::path::Path;
use crate::RepositoryInfo;
use crate::utils::helpers::Result;
use git2::Repository;

impl RepositoryInfo {
    pub fn new<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        
        let repo = Repository::open(&path)?;
        let name = path
            .parent()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();

        Ok(Self { name, repo })
    }
}
