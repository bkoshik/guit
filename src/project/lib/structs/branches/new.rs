use crate::Branches;
use std::error::Error;

impl Branches {
    pub fn new(local: Vec<String>, remote: Vec<String>) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            local,
            remote
        })
    }
}