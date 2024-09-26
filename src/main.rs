// Let Rust know that we use an external dependency
extern crate rand;

// The io library comes from the standard library (a.k.a std)
// The `use` statement brings the type into scope
use std::io;
// Bringing a type called `std::cmp::Ordering` into scope. `Ordering` is an enum
// which has the variants of `Less`, `Greater`, and `Equal`.
use std::cmp::Ordering;
// Put the `Rng` trait in scope for us to use its methods
use rand::Rng;

// The `main` function is the entry point to the program
fn main() {
    // The `println!` macro prints a string to the screen
    println!("Guess the number!");

    // The `rand::thread_rng` function will give us the particular random number generator
    // that we're going to use. We call the `gen_range` method on the random number generator
    // which takes a range and returns a random number from the range.
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
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

        // Convert the `String` into a real number type, so we can compare it numerically.
        // We use the same variable name to `shadow` the previous value with a new one.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue
            },
        };
        // With the `match` expression, we can do error handling.
        // If `parse` can successfully convert the string to a number,
        // it will return the `Ok` variant of `Result` that contains the resulting number
        // and the `match` expression will return the number that we want from the `Ok` value
        // If `parse` returns an `Err` `Result` variant, the `match` expression will
        // print the message and continues the loop. The `_` is a catchall value: it will match
        // all `Err` values, no matter what information they have inside them.

        println!("You guessed: {}", guess);

        // We use a `match` expression to decide what to do based on the variant of `Ordering`
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                // Brake from the loop when the guess is correct, and so closing the program.
                break;
            },
        }
    }
}
