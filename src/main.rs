use guit::{AuthorInfo, Message, RepositoryInfo};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let repo = RepositoryInfo::new(".")?;

    let _ = repo.add_files(&["*"]);
    let _ = repo.commit_changes(
        AuthorInfo::from(repo.repo().config()?),
        Message::new(
            "Add RepositoryInfo::push()",
            ""
        )
    );
    let _ = repo.push();

    Ok(())
}
