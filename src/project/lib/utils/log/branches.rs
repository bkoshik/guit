use crate::utils::helpers::{find_branches, Result};
use git2::{BranchType, Oid, Repository};
use crate::Branches;

pub fn branches(repo: &Repository, commit_id: Oid) -> Result<Branches> {
    let local_branches = find_branches(repo, commit_id, BranchType::Local)?;
    let remote_branches = find_branches(repo, commit_id, BranchType::Remote)?;
    
    Ok(Branches::new(local_branches, remote_branches)?)
}
