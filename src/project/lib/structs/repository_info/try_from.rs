use git2::Repository;
use crate::RepositoryInfo;

impl TryFrom<Repository> for RepositoryInfo {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: Repository) -> Result<Self, Self::Error> {
        let name = value
            .path()
            .parent()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();

        Ok(Self {
            name,
            repo: value
        })
    }
}