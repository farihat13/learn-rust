use rand::Rng;
use std::io;

fn main() {
    // const SECRET:i32 = 34;

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    for attempt in 1..=10 {
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u8 = guess.trim().parse().expect("Type a number: ");

        println!("Attempt {}: You guessed: {guess}", attempt);
        if guess == secret_number {
            println!("Your guess is correct!");
            break;
        } else if guess > secret_number {
            println!("Too big. Predict a smaller one...");
        } else {
            println!("Too small. Predict a larger one...");
        }
    }
}
