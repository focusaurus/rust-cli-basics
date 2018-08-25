# Command line interface basics in rust

1. `cargo init --name divide --bin .`
1. change signature of main to return `Result<(), ()>`
  * return `Ok(())`
1. Add a custom error struct
  * Declare the struct
  * Update `main` type signature return
  * Return `Err(DivideError { message: "oops"}))`
  * Implement debug via `#[derive(Debug)]` attribute
1. Implement custom Debug trait
  * `use std::fmt;`
  implement `fmt::Debug` trait's `fmt` method
  * remove derive attribute
1. Check command line argument length
  * use `std::env::args`
1. Parse numbers
  * Call `.parse()` to get `i64`
  * Use `?` to get simple error handling
  * Implement `std::convert::From` to allow automatic conversion from `std::num::ParseIntError` to `DivideError`
1. Do the division, print the quotient
1. Handle zero divisor nicely
1. Add helper exit method
  * Takes `message: String` returns `Result<(), DivideError>`
1. Move error code to separate module
  * move to `error.rs`
  * `mod error;`
  * use `error::` namespace
  * clean up `use`
