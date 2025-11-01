use crate::utils::helpers::Result;
use crate::AuthorInfo;
use git2::Signature;

impl AuthorInfo {
    pub fn to_signature(&self) -> Result<Signature<'_>> {
        Ok(Signature::now(self.name(), &self.email())?)
    }
}
