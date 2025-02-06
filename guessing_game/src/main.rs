use rand::Rng;
use std::cmp::Ordering;
use std::io; // Basic input output library

fn main() {
    println!("Guess the number!");
    // Rng --> must be in scope
    // gen_range --> Random number generator
    // (a..=b) range
    let secret_number = rand::rng().random_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");
        // In rust, the variables are immutable by default
        // To make it mutable we use "mut"
        let mut guess = String::new(); // Calling a new string fn                               // Empty instance of string
                                       // ::new --> means new is associated to the function

        // let mut guess = String::new(); line has created a mutable variable that is currently bound to a new, empty instance of a String

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line"); // Handling Potential Failure with Result

        // Rust allows us to shadow the previous value of guess with a new one.
        // u32 is variable type for the guess`
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed : {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!!!"),
            Ordering::Equal => {
                println!("You win!!");
                break;
            }
        }
    }
}
