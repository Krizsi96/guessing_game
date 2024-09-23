// The io library comes from the standard library (a.k.a std)
// The `use` statement brings the type into scope
use std::io;

// The `main` function is the entry point to the program
fn main() {
    // The `println!` macro prints a string to the screen
    println!("Guess the number!");
    println!("Please input your guess.");

    // The 'let` statement is used to create a variable
    // In Rust variables are immutable by default, the `mut` keyword makes
    // the variable mutable.
    // Calling `String::new()` returns a new instance of a `String`.
    // `String` is a string type provided by the standard library that is growable,
    // UTF-8 encoded bit of text. The `::` syntax in the `::new` line indicates that `new`
    // is an `associated function` of the `String` type.
    let mut guess = String::new();
    // The `stdin` function returns an instance of `std::io::Stdin`, which is a type
    // that represents a handle to the standard input for the terminal. The `read_line`
    // method is called on the standard input handle to get input from the user.
    // The guess variable is passed to the `read_line` method as a mutable reference.
    //
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
    // The `expect` method will cause the program to crash and display the message
    // passed as an argument, when an error occurs in the `read_line` method.

    println!("You guessed: {}", guess);
}
