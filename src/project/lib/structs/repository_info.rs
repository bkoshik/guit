use crate::Branches;
use git2::Repository;

mod commands {
    mod commit {}
    mod log {
        mod log;
    }
    mod status {}
}
mod new;

pub struct RepositoryInfo {
    name: String,
    repo: Repository,
}
