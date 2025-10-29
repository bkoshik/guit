use git2::Repository;

mod commands {
    mod commit {}
    mod log {
        mod log;
    }
    mod status {}
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
