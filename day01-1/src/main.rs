use std::char;
use std::io::stdin;

fn main() {
    let answer: u32 = stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let first_digit = line.chars().find(char::is_ascii_digit).unwrap();
            let last_digit = line.chars().rev().find(char::is_ascii_digit).unwrap();

            10 * first_digit.to_digit(10).unwrap() + last_digit.to_digit(10).unwrap()
        })
        .sum();

    println!("{answer}");
}
