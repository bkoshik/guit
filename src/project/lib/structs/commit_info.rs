mod get {
    mod id;
    mod author;
    mod branches;
    mod date;
    mod is_head;
    mod message;
    mod parents;
    mod tags;
}
mod new;

use std::time::SystemTime;
use git2::Oid;
use crate::{AuthorInfo, Branches, Message};

pub struct CommitInfo {
    pub id: Oid,
    pub author: AuthorInfo,
    pub branches: Branches,
    pub date: SystemTime,
    pub is_head: bool,
    pub message: Message,
    pub parents: Vec<Oid>,
    pub tags: Vec<String>,
}