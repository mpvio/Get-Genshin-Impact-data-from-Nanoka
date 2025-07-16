use std::fmt::{self};

pub struct CharacterError {
    pub message: String
}

impl fmt::Display for CharacterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", &self.message)
    }
}

impl fmt::Debug for CharacterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CharacterError").field("message", &self.message).finish()
    }
}