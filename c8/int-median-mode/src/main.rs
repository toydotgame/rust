/*
 * AUTHOR: toydotgame
 * CREATED: 2025-11-25
 * The Book ch. 8 § 3:
 * Given a list of integers, use a vector and return the median (when sorted,
 * the value in the middle position) and mode (the value that occurs most
 * often; a hash map will be helpful here) of the list.
 */

use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    println!(
        "Insert a list of integers, hitting return to denote the next \
        field. The first non-integer (e.g. an empty input) will denote the \
        end of input."
    );

    let mut input_ints: Vec<i32> = Vec::new();

    loop {
        print!("Enter a i32: ");
        io::stdout()
            .flush()
            .expect("Failed to flush stdout when gathering inputs!");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Couldn't read input!");
        match input.trim().parse() {
            Ok(i) => input_ints.push(i),
            Err(_) => {
                println!("Read non-integer value, continuing...");
                break;
            }
        }
    }

    if input_ints.len() <= 0 {
        panic!("Input list is empty!");
    }

    input_ints.sort_unstable(); // _unstable() means equal elements may be swapped (we don't care)
                                // at the benefit of generally faster sort times
    let median: &i32 = &input_ints[input_ints.len() / 2];
    println!("The median is {}", median);

    let mut occurrences: HashMap<i32, i32> = HashMap::new();
    for i in input_ints {
        let count: &mut i32 = occurrences.entry(i).or_insert(0);
        *count += 1;
    }
    // Lazy beginner (definitely un-idiomatic) way:
    struct MaxValue {
        integer: Option<i32>,
        count: i32,
    }
    let mut max_input = MaxValue {
        integer: Option::None,
        count: -1,
    };
    for (k, v) in occurrences.iter() {
        if *v > max_input.count {
            max_input.integer = Some(*k);
            max_input.count = *v;
        }
    }

    if max_input.integer == Option::None {}
    match max_input.integer {
        Some(i) => println!("The mode is {} ({} occurrences)", i, max_input.count),
        None => panic!("Black magic occurred and I couldn't discern the mode!"), // How
    }
}
