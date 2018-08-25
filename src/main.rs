use std::{env, fmt};

struct DivideError {
    message: String,
}

impl fmt::Debug for DivideError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

fn main() -> Result<(), DivideError> {
    if env::args().len() < 3 {
        return Err(DivideError {
            message: "Need at least 2 numbers on the command line to divide".into(),
        });
    }
    Err(DivideError {
        message: "oops".into(),
    })
}
