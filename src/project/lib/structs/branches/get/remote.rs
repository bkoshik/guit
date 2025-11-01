use crate::Branches;

impl Branches {
    pub fn remote(&self) -> &Vec<String> {
        &self.remote
    }
}
