use crate::{FileStatusEntry, FileStatusKind};

impl FileStatusEntry {
    pub fn status(&self) -> &FileStatusKind {
        &self.status
    }
}