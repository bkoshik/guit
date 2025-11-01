use crate::utils::commands;
use crate::utils::helpers::Result;
use crate::{AuthorInfo, CommitInfo, Message};
use git2::{Commit, Repository};
use std::time::{Duration, UNIX_EPOCH};

pub fn commit(commit: &Commit, repo: &Repository) -> Result<CommitInfo> {
    let id = commit.id();
    let author = AuthorInfo::from(commit.author());
    let branches = commands::branches(repo, commit.id())?;
    let date = UNIX_EPOCH + Duration::from_secs(commit.time().seconds() as u64);
    let is_head = commands::head(repo, commit.id())?;
    let message = Message::new(
        commit.summary().unwrap_or("\x1b[2;3mNo message\x1b[0m"),
        commit.message().unwrap_or("\x1b[2;3mNo message\x1b[0m"),
    );
    let parents = commit.parents().map(|parent| parent.id()).collect();
    let tags = commands::tags(repo, commit.id())?;

    Ok(CommitInfo::new(
        id, author, branches, date, is_head, message, parents, tags,
    ))
}
