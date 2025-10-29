use git2::{Config, Signature};
use crate::AuthorInfo;
use crate::consts::{NO_EMAIL, NO_NAME};

impl From<Signature<'_>> for AuthorInfo {
    fn from(sig: Signature) -> Self {
        Self {
            name: sig.name().unwrap_or(NO_NAME).to_string(),
            email: sig.email().unwrap_or(NO_EMAIL).to_string(),
        }
    }
}

impl From<Config> for AuthorInfo {
    fn from(config: Config) -> Self {
        Self {
            name: config.get_string("user.name").unwrap_or("".to_string()),
            email: config.get_string("user.email").unwrap_or("".to_string())
        }
    }
}