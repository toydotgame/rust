/*
 * AUTHOR: toydotgame
 * CREATED: 2025-11-21
 * Chapter 2 guessing game example as shown in The
 * Book.
 */

use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Guess le number!");

    let secret = rand::thread_rng().gen_range(1..=100);
    //println!("secret = {secret}");

    loop {
        println!("Go:");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Couldn't read line!");
        let guess:u16 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_)  => continue,
        };
            //.expect("NaN or too big!");
        println!("Guess was {guess}");

        match guess.cmp(&secret) {
            Ordering::Less =>       println!("Too small"),
            Ordering::Greater =>    println!("Too big"),
            Ordering::Equal => {
                println!("Correct!");
                break;
            }
        }
    }
}

