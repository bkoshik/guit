use crate::utils::helpers::Result;
use crate::{utils, AuthorInfo, CommitInfo, Message, RepositoryInfo};

impl RepositoryInfo {
    pub fn commit_changes(&self, author: AuthorInfo, message: Message) -> Result<CommitInfo> {
        let mut index = self.repo.index()?;

        let tree_id = index.write_tree()?;
        let tree = self.repo.find_tree(tree_id)?;

        let signature = self.repo.signature()?;

        let parent = self.repo.head()?.peel_to_commit()?;

        let commit_id = self.repo.commit(
            Some("HEAD"),
            &signature,
            &author.to_signature()?,
            &message.to_string(),
            &tree,
            &[&parent],
        )?;

        let commit = self.repo.find_commit(commit_id)?;
        Ok(utils::commands::commit(&commit, self.repo())?)
    }
}
