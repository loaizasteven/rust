use rand::Rng; // Bring the Rng trait into scope, Rng is impl for thread_rng() struct
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("\x1b[44m
    Welcome to the guessing game!
    The computer will generate a random number between 1 and 100.
    You have to guess the number.
    If you want to quit the game, type 'quit'.
    \x1b[0m");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // quit statement check lowercase
        if guess.trim().to_lowercase() == "quit" {
            println!("Quit the game");
            break;
        }

        //convert type
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    println!("The secret number is: {}", secret_number);
}
