use crate::Message;
use std::fmt::Display;

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{}\n\n{}", self.summary, self.message))
    }
}
