use git2::Signature;
use crate::AuthorInfo;
use crate::consts::{NO_EMAIL, NO_NAME};

impl From<Signature<'_>> for AuthorInfo {
    fn from(s: Signature) -> Self {
        Self {
            name: s.name().unwrap_or(NO_NAME).to_string(),
            email: s.email().unwrap_or(NO_EMAIL).to_string(),
        }
    }
}