use std::{io, cmp::Ordering};
use rand::Rng;

fn main() {
    println!("Guess the number!");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess:");
    
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        // Convert the String the program reads as input into a real number type
        let guess: u32 = guess.trim().parse().expect("Please type a number!");
    
        println!("You guessed: {guess}");
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("To small!"),
            Ordering::Greater => println!("To big!"),
            Ordering::Equal => println!("Your win!"),
        }
    }
}
