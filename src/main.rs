use std::env;
mod error;

fn main() -> Result<(), error::DivideError> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        return error::exit("Need at least 2 numbers on the command line to divide");
    }
    let dividend: f64 = args[1].parse()?;
    let divisor: f64 = args[2].parse()?;
    if divisor == 0f64 {
        return error::exit("Cannot divide by zero");
    }
    let quotient: f64 = dividend / divisor;
    println!("{}", quotient);
    Ok(())
}
