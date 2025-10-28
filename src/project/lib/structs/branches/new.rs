use std::error::Error;
use git2::BranchType;
use crate::Branches;

impl Branches {
    pub fn new(repo: &git2::Repository) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            local: get_branch_name(repo.branches(Some(BranchType::Local))?)?,
            remote: get_branch_name(repo.branches(Some(BranchType::Remote))?)?,
        })
    }
}

fn get_branch_name(branches: git2::Branches) -> Result<Vec<String>, Box<dyn Error>> {
    let mut branch_names = Vec::new();
    for branch in branches {
        let (branch, _) = branch?;
        if let Some(name) = branch.name()? {
            branch_names.push(name.to_string());
        }
    }

    Ok(branch_names)
}