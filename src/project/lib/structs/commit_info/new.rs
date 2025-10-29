use crate::utils::helpers::Result;
use git2::{Commit, Repository};
use crate::{AuthorInfo, CommitInfo, Message};
use crate::consts::NO_MESSAGE;
use crate::utils::log;

impl CommitInfo {
    pub fn new(commit: &Commit, repo: &Repository) -> Result<Self> {
        let id = commit.id();
        let author = AuthorInfo::from(commit.author());
        let branches = log::branches(repo, commit.id())?;
        let date = log::date(&commit.time());
        let is_head = log::head(repo, commit.id())?;
        let message = Message::new(
            commit.summary().unwrap_or(NO_MESSAGE), 
            commit.message().unwrap_or(NO_MESSAGE)
        );
        let parents = commit.parents()
            .map(|parent| parent.id().to_string())
            .collect();
        let tags = log::tags(repo, commit.id())?;
        
        Ok(CommitInfo {
            id,
            author,
            branches,
            date,
            is_head,
            message,
            parents,
            tags
        })
    }
}