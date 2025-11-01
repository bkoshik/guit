use std::path::PathBuf;
use crate::{FileStatusEntry, FileStatusKind};

impl FileStatusEntry {
    pub fn new<P>(path: P, status: FileStatusKind) -> Self
    where
        P: AsRef<PathBuf>,
    {
        Self {
            path: path.as_ref().to_path_buf(),
            status
        }
    }
}