/*
 * AUTHOR: toydotgame
 * CREATED: 2024-11-24
 * Generate the nth Fibonacci number input by the user.
 */

use num::bigint::BigUint;
use num::traits::One;
use std::io;
use std::io::Write;
use std::time::SystemTime;

fn main() {
    loop {
        print!("Which (nth) Fibonacci number do you want?: ");
        io::stdout()
            .flush()
            .expect("Failed to flush stdout when printing prompt!");

        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Couldn't read input n!");
        let n: usize = match line.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let start_time = SystemTime::now();

        let mut a = BigUint::ZERO;
        let mut b = BigUint::one();
        for _ in 0..n {
            let c = a + &b;
            a = b;
            b = c;
        }

        println!("The {n}th Fibonacci number is {a}");
        let end_time = SystemTime::now();
        let delta_time = end_time
            .duration_since(start_time)
            .expect("Time went backwards!");
        println!(
            "Took {}.{:09} s",
            delta_time.as_secs(),
            delta_time.subsec_nanos()
        );
    }
}
