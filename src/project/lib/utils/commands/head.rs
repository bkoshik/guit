use git2::{Error, Oid, Repository};

pub fn head(repo: &Repository, commit_id: Oid) -> Result<bool, Error> {
    let head = repo.head()?;
    let head_commit = head.peel_to_commit()?;

    if head_commit.id() == commit_id {
        return Ok(true);
    }

    Ok(false)
}
