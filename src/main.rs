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
    fn from(_pie: num::ParseFloatError) -> Self {
        DivideError {
            message: "Please pass only numeric arguments".into(),
        }
    }
}

fn main() -> Result<(), DivideError> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        return Err(DivideError {
            message: "Need at least 2 numbers on the command line to divide".into(),
        });
    }
    let dividend: f64 = args[1].parse()?;
    let divisor: f64 = args[2].parse()?;
    if divisor == 0f64 {
        return Err(DivideError {
            message: "Cannot divide by zero".into(),
        });
    }
    let quotient: f64 = dividend / divisor;
    println!("{}", quotient);
    Ok(())
}
