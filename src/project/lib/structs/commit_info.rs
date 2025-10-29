mod new;

use git2::Oid;
use crate::{AuthorInfo, Branches, Message};

pub struct CommitInfo {
    pub id: Oid,
    pub author: AuthorInfo,
    pub branches: Branches,
    pub date: String,
    pub is_head: bool,
    pub message: Message,
    pub parents: Vec<String>,
    pub tags: Vec<String>,
}