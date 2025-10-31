use crate::utils::helpers::Result;
use crate::{utils, CommitInfo, RepositoryInfo};

impl RepositoryInfo {
    pub fn log(&self) -> Result<Vec<CommitInfo>> {
        let mut revwalk = self.repo.revwalk()?;
        revwalk.push_head()?;

        let mut commits = Vec::new();
        for commit_id in revwalk {
            let commit_id = commit_id?;
            let commit = self.repo.find_commit(commit_id)?;

            commits.push(utils::log::commit(&commit, &self.repo)?);
        }

        Ok(commits)
    }
}
