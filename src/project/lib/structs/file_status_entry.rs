mod new;

use std::path::PathBuf;
use crate::FileStatusKind;

mod get {
    mod path;
    mod status;
}

pub struct FileStatusEntry {
    path: PathBuf,
    status: FileStatusKind,
}