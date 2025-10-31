use crate::{AuthorInfo, Branches, CommitInfo, Message};
use git2::Oid;
use std::time::SystemTime;

impl CommitInfo {
    pub fn new(id: Oid, author: AuthorInfo, branches: Branches, date: SystemTime, is_head: bool, message: Message, parents: Vec<Oid>, tags: Vec<String>) -> Self {
        Self {
            id,
            author,
            branches,
            date,
            is_head,
            message,
            parents,
            tags
        }
    }
}