use crate::utils::helpers::Result;
use crate::RepositoryInfo;
use git2::IndexAddOption;
use std::path::Path;

impl RepositoryInfo {
    pub fn add_files(&self, files: &[&str]) -> Result<()> {
        let mut index = self.repo.index()?;

        if files.len() == 1 && files[0] == "*" {
            index.add_all(["*"].iter(), IndexAddOption::DEFAULT, None)?
        } else {
            for file in files {
                index.add_path(Path::new(file))?;
            }
        }

        index.write()?;

        Ok(())
    }
}
