use std::time::{Duration, UNIX_EPOCH};
use git2::{Commit, Repository};
use crate::{AuthorInfo, CommitInfo, Message};
use crate::consts::NO_MESSAGE;
use crate::utils::log;
use crate::utils::helpers::Result;

pub fn commit(commit: &Commit, repo: &Repository) -> Result<CommitInfo> {
    let id = commit.id();
    let author = AuthorInfo::from(commit.author());
    let branches = log::branches(repo, commit.id())?;
    let date = UNIX_EPOCH + Duration::from_secs(commit.time().seconds() as u64);
    let is_head = log::head(repo, commit.id())?;
    let message = Message::new(
        commit.summary().unwrap_or(NO_MESSAGE),
        commit.message().unwrap_or(NO_MESSAGE)
    );
    let parents = commit.parents()
        .map(|parent| parent.id())
        .collect();
    let tags = log::tags(repo, commit.id())?;

    Ok(CommitInfo::new(
        id,
        author,
        branches,
        date,
        is_head,
        message,
        parents,
        tags
    ))
}