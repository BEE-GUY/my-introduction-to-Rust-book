use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // generation of secret number
    let secret_number = rand::random_range(1..=100);

    loop {
        println!("Please input your guess.");

        //creating input string.
        let mut guess = String::new();

        // getting input for the input string
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // converting input string to u32
        let guess: u32 = match guess.trim().parse() {
            Ok(number) => number,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        //checking if the numbers match
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
