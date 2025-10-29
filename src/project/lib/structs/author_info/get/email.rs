use crate::AuthorInfo;

impl AuthorInfo {
    pub fn email(&self) -> &str {
        &self.email
    }
}