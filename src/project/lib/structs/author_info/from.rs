use crate::AuthorInfo;
use git2::{Config, Signature};

impl From<Signature<'_>> for AuthorInfo {
    fn from(sig: Signature) -> Self {
        Self {
            name: sig.name().unwrap_or("\x1b[2;3mNo name\x1b[0m").to_string(),
            email: sig.email().unwrap_or("\x1b[2;3mNo email\x1b[0m").to_string(),
        }
    }
}

impl From<Config> for AuthorInfo {
    fn from(config: Config) -> Self {
        Self {
            name: config.get_string("user.name").unwrap_or("".to_string()),
            email: config.get_string("user.email").unwrap_or("".to_string()),
        }
    }
}
