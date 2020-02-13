use std::error::Error;
use std::fmt;


#[derive(Debug)]
pub struct PokeError {
    details: String
}

impl PokeError {
    pub fn new(message: &str) -> PokeError {
        PokeError { details: message.to_string() }
    }
}

impl fmt::Display for PokeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl  Error for PokeError {
    fn description(&self) -> &str {
        &self.details
    }
}
