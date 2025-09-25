use rand::prelude::*;
use rand::rng;
use std::io;

fn main() {
    let guess_list = ["banana", "apple", "mango", "orange", "grapes"];
    let mut rng = rng();
    let index = rng.random_range(0..guess_list.len());
    let random_fruit = guess_list[index];

    println!("Welcome to the Fruit Guessing Game!");
    println!("Available fruits: {:?}", guess_list);
    println!("Try to guess the randomly selected fruit!");

    let mut turns = 0;
    loop {
        println!("\nEnter your guess:");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let fruit_name = input.trim().to_lowercase();
                turns += 1;
                if !guess_list.contains(&fruit_name.as_str()) {
                    println!("Invalid fruit! Please enter a valid fruit from the list.");
                    continue;
                }
                if fruit_name == random_fruit {
                    println!(
                        "Congratulations! You guessed correctly in {} turn(s)! The fruit was '{}'.",
                        turns, random_fruit
                    );
                    break;
                } else {
                    println!("Wrong guess. Try again!");
                }
            }
            Err(error) => {
                eprintln!("Error reading input: {}", error);
            }
        }
    }
}
