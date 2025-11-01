use crate::RepositoryInfo;
use crate::utils::helpers::Result;

impl RepositoryInfo {
    pub fn push(&self) -> Result<()> {
        let head = self.repo.head()?;
        let branch_name = head.shorthand().unwrap();

        let mut remote = self.repo.find_remote("origin")?;

        let refspec = format!("refs/heads/{}:refs/heads/{}", branch_name, branch_name);

        remote.push(&[&refspec], None)?;

        Ok(())
    }
}