use colored::{self, Colorize};
use rand::random_range;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guest the number.");

    let secret_number = random_range(1..=100);

    loop {
        println!("Enter your number:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Cannot read the line");

        let guess: u32 = match guess.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Please enter your number.");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("{}", "Bravo, you've got it!".on_green());
                break;
            }
        }
    }
}
