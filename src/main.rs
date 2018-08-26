use std::env;
mod error;
use error::DivideError;

fn main() -> Result<(), DivideError> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        DivideError::exit("Please provide 2 numeric arguments")?;
    }
    let dividend: f64 = args[1].parse()?;
    let divisor: f64 = args[2].parse()?;
    let quotient = dividend / divisor;
    if divisor == 0f64 {
        DivideError::exit("Cannot divide by zero")?;
    }
    println!("{}", quotient);
    Ok(())
}
