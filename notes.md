# Command line interface basics in rust

1. `cargo init --bin .`
1. change signature of main to return `Result<(), ()>`
  * return `Ok(())`
1. Add a custom error struct
  * Declare the struct
  * Update `main` type signature return
  * Return `Err(DivideError { message: "oops"}))`
  * Implement debug via annotation
