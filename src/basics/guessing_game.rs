use colored::*;
use core::num;
use rand::Rng;
use std::cmp::Ordering;
use std::f32::consts::E;
use std::io::{self, Read};

pub fn run() {
    println!("guess the number");

    let secret = rand::thread_rng().gen_range(1, 101);

    println!("secret number is: {}", secret);

    loop {
        let mut guess = String::new();

        println!("input number:");

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {}", guess);

        match guess.cmp(&secret) {
            Ordering::Less => println!("{}", "lesser".red()),
            Ordering::Equal => {
                println!("{}", "kk".green());
                break;
            }
            Ordering::Greater => println!("{}", "more".red()),
        }
    }
}
