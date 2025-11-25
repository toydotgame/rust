/*
 * AUTHOR: toydotgame
 * CREATED: 2025-11-25
 * The Book ch. 8 § 3:
 * Convert strings to pig latin. The first consonant of each word is moved to
 * the end of the word and ay is added, so first becomes irst-fay. Words that
 * start with a vowel have hay added to the end instead (apple becomes
 * apple-hay). Keep in mind the details about UTF-8 encoding!
 */

use std::io;

const VOWELS: [char; 6] = ['a', 'e', 'i', 'o', 'u', 'y'];

fn main() {
    println!("Write a sentence:");
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input!");
    let input = input.trim().to_ascii_lowercase(); // Drops mut

    let mut output: String = String::new();
    for word in input.split_whitespace() {
        let chars: Vec<char> = word.chars().collect();
        if chars.len() <= 0 {
            continue;
        }

        let mut output_word: String = String::new();
        if VOWELS.contains(&chars[0]) {
            output_word = String::from(word) + "-hay";
            // `chars` is left unconsumed
        } else {
            let mut chars: Vec<char> = chars; // Add mutability in this scope
            let first_consonant: char = chars.remove(0);
            chars.push('-');
            chars.push(first_consonant);
            output_word = chars.into_iter().collect(); // Consumes `chars`
            output_word += "ay";
        }

        if output.len() > 0 {
            output_word.insert_str(0, " ");
        }
        output += &output_word;
    }

    println!("{}", output);
}
