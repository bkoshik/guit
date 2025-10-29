use crate::utils::helpers::Result;
use git2::{BranchType, Oid, Repository};

pub fn find_branches(repo: &Repository, commit_id: Oid, branch_type: BranchType) -> Result<Vec<String>> {
    let mut branches: Vec<String> = Vec::new();

    for branch in repo.branches(Some(branch_type))? {
        let (branch, _) = branch?;

        if let Some(branch_name) = branch.get().name()
            && let Some(target_oid) = branch.get().target()
            && target_oid == commit_id
        {
            branches.push(branch_name.to_string().to_string());
        }
    }

    Ok(branches)
}