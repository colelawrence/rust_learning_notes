extern crate rand;

// bring in io library from standard library
use std::cmp::Ordering;
use std::io;
use rand::Rng;

// entry point into the program
fn main() {
    let number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", number);
    loop {
        println!("Guess the number!");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Couldn't read guess!");
        println!("You guessed: {}", guess);
        let guess_number: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Couldn't read guess, please try again!");
                continue;
            }
        };

        match guess_number.cmp(&number) {
            Ordering::Less => println!("Too low"),
            Ordering::Greater => println!("Too high"),
            Ordering::Equal => {
                println!("Nailed it!");
                break;
            }
        }
    }
}
