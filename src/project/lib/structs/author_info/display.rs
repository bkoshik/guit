use crate::AuthorInfo;
use std::fmt::{Display, Formatter, Result};

impl Display for AuthorInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} <{}>", self.name, self.email)
    }
}
