mod get {
    mod local;
    mod remote;
}
mod new;

pub struct Branches {
    local: Vec<String>,
    remote: Vec<String>,
}
