use crate::Branches;
use git2::Repository;

mod new;

pub struct RepositoryInfo {
    name: String,
    branches: Branches,
    repo: Repository,
}
