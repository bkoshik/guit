use crate::RepositoryInfo;
use std::path::Path;

impl RepositoryInfo {
    pub fn remove_files(&self, files: &[&str]) -> crate::utils::helpers::Result<()> {
        let mut index = self.repo.index()?;

        for file in files {
            index.remove_path(Path::new(file))?;
        }

        index.write()?;

        Ok(())
    }
}
