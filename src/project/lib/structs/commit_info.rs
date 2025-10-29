mod get {
    mod author;
    mod branches;
    mod date;
    mod id;
    mod is_head;
    mod message;
    mod parents;
    mod tags;
}
mod new;

use crate::{AuthorInfo, Branches, Message};
use git2::Oid;
use std::time::SystemTime;

pub struct CommitInfo {
    id: Oid,
    author: AuthorInfo,
    branches: Branches,
    date: SystemTime,
    is_head: bool,
    message: Message,
    parents: Vec<Oid>,
    tags: Vec<String>,
}
