use git2::{Error, Oid, Repository};

pub fn tags(repo: &Repository, commit_oid: Oid) -> Result<Vec<String>, Error> {
    let mut tags: Vec<String> = Vec::new();

    for tag in repo.tag_names(None)?.iter() {
        let tag = tag.unwrap_or("").to_string();
        let tag_ref = repo.find_reference(&format!("refs/tags/{}", &tag))?;

        if let Ok(peeled) = tag_ref.peel_to_commit()
            && peeled.id() == commit_oid
        {
            tags.push(tag)
        }
    }

    Ok(tags)
}