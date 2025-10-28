use git2::Repository;
use crate::{AuthorInfo, Branches};

mod new;

pub struct RepositoryInfo {
    name: String,
    branches: Branches,
    repo: Repository,
}