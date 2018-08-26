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
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        return Err(DivideError {
            message: "Please provide 2 numeric arguments".into(),
        });
    }
    Err(DivideError {
        message: "Failed".into(),
    })
}
