#[derive(Debug)]
struct DivideError {
    message: String,
}

fn main() -> Result<(), DivideError> {
    println!("Hello, world!");
    Err(DivideError { message: "Failed".into() })
}
