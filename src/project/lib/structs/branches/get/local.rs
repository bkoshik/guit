use crate::Branches;

impl Branches {
    pub fn local(&self) -> &Vec<String> {
        &self.local
    }
}
