
//! Prints a farewell message to the standard output.
//!
//! This `goodbye` function is used to print the message "Good bye" to the console.
//! It can be invoked when you want to display a goodbye message to the user.
//!
//! # Example
//!
//! ```
//! // This will print "Good bye" to the console.
//! fn goodbye() {
//!     println!("Good bye");
//! }
//! ```

mod farewell;

mod goodbye;


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
    farewell::farewell();
    goodbye::goodbye();
}