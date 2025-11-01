use crate::RepositoryInfo;
use std::path::Path;

impl RepositoryInfo {
    pub fn remove_files<P>(&self, files: &[P]) -> crate::utils::helpers::Result<()>
    where
        P: AsRef<Path>,
    {
        let mut index = self.repo.index()?;

        for file in files {
            index.remove_path(Path::new(file.as_ref()))?;
        }

        index.write()?;

        Ok(())
    }
}
