use std::io;

fn main() {
    println!("Guess the number!");
    println!("from 1 to 10");

    let mut guess = String::new();
    // IO library to capture user input, it is a function that returns a Enum type called Result.
    // The Result has two variants Ok and Err
    // Ok means the operation was successful
    // Err means the operation failed
    // The expect method is used to handle the Err variant
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
