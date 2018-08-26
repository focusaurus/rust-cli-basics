use std::{convert, env, fmt, num};
struct DivideError {
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

fn main() -> Result<(), DivideError> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        return Err(DivideError {
            message: "Please provide 2 numeric arguments".into(),
        });
    }
    let dividend: f64 = args[1].parse()?;
    let divisor: f64 = args[2].parse()?;
    let quotient = dividend / divisor;
    println!("{}", quotient);
    Ok(())
}
