extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

/// GuessGame from std docs of rust

fn main() {
    println!("Guess the number!");

    let mut chances: u32 = 0;

    // Generate and store a randowm number b/w 1<->100
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // Go into a infinite loop
    loop {
        println!("Chances used {}", chances);

        println!("Please input your guess.");

        // guess is a mutable empty string
        let mut guess = String::new();

        // Get input from user
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // Convert and shadow guess into a unsigned 32bit integer
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,         // Match num if everything is OK
            Err(_) => continue,     // Continue even if anything != OK happens
        };

        println!("You guessed: {}", guess);

        // Match the respective value of guess wrt secret number
        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("You Win!");
                println!("Total Chances used => {}", chances);
                break;              // Break the Inf Loop and exit
            }
        }

        chances += 1;
    }
}
