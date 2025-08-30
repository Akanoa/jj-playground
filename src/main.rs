//! Entry point of the program.
//!
//! The `main` function is the starting point of any Rust application.
//! When the program is executed, this function is called.
//! In this example, it prints "Hello, world!" to the console.
//!
//! # Example
//! ```
//! $ cargo run
//! Hello, world!
//! ```
/// The entry point of the Rust program.
///
/// This `main` function serves as the starting point for the execution of the program.
/// When executed, it prints the message "Hello, world!" to the standard output.
///
/// # Example
///
/// ```
/// // This will print "Hello, world!" to the console.
/// fn main() {
///     println!("Hello, world!");
/// }
/// ```
fn main() {
    println!("Hello, world!");
    goodbye()
}


fn goodbye() {
    println!("Good bye")
}