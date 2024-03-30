use std::error;
use std::fmt;

// struct
#[derive(Debug, Clone)]
pub struct DeserializeError;

//impl
impl fmt::Display for DeserializeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unable to deserialize.")
    }
}
impl error::Error for DeserializeError {}