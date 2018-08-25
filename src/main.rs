use std::fmt;

struct DivideError {
    message: String,
}

impl fmt::Debug for DivideError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

fn main() -> Result<(), DivideError> {
    Err(DivideError {
        message: "oops".into(),
    })
}
