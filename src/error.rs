use std::{convert, fmt, num};

pub struct DivideError {
    message: String,
}

impl fmt::Debug for DivideError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl convert::From<num::ParseFloatError> for DivideError {
    fn from(_error: num::ParseFloatError) -> Self {
        DivideError {
            message: "Please provide valid numbers as arguments".into(),
        }
    }
}

pub fn exit(message: &str) -> Result<(), DivideError> {
    Err(DivideError {
        message: message.into(),
    })
}
