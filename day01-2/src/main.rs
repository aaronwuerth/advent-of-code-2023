use std::char;
use std::io::stdin;

const SPELLED_DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    let answer: u32 = stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let first_digit = line
                .char_indices()
                .find_map(|(pos, c)| filter(&line, pos, c))
                .unwrap();
            let last_digit = line
                .char_indices()
                .rev()
                .find_map(|(pos, c)| filter(&line, pos, c))
                .unwrap();

            10 * first_digit + last_digit
        })
        .sum();

    println!("{answer}");
}

fn filter(line: &str, pos: usize, c: char) -> Option<u32> {
    if c.is_ascii_digit() {
        Some(c.to_digit(10).unwrap())
    } else {
        SPELLED_DIGITS.iter().enumerate().find_map(|(i, digit)| {
            if line[pos..].starts_with(digit) {
                Some((i + 1) as u32)
            } else {
                None
            }
        })
    }
}
