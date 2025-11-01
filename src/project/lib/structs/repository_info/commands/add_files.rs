use crate::utils::helpers::Result;
use crate::RepositoryInfo;
use git2::IndexAddOption;
use std::path::Path;

impl RepositoryInfo {
    pub fn add_files<P>(&self, files: &[P]) -> Result<()>
    where P: AsRef<Path>
    {
        let files = files.iter().map(AsRef::as_ref).collect::<Vec<_>>();
        let mut index = self.repo.index()?;

        if files.len() == 1
            && files[0].to_str().unwrap_or("") == "*" 
        {
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
