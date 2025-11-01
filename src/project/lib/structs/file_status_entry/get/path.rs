use std::path::PathBuf;
use crate::FileStatusEntry;

impl FileStatusEntry {
    pub fn path(&self) -> &PathBuf {
        &self.path
    }
}