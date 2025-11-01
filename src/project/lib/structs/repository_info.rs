use git2::Repository;

mod commands {
    mod add_files;
    mod commit_changes;
    mod log;
    mod remove_files;
}
mod get {
    mod name;
    mod repo;
}
mod new;

pub struct RepositoryInfo {
    name: String,
    repo: Repository,
}
