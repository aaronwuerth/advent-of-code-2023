use std::io::stdin;

fn main() {
    let answer: u64 = stdin()
        .lines()
        .map(|line| {
            line.unwrap()
                .as_bytes()
                .split(|c| *c == b',')
                .map(|s| hash(s) as u64)
                .sum::<u64>()
        })
        .sum();

    println!("{answer}");
}

fn hash(s: &[u8]) -> u8 {
    s.iter().fold(0, |acc, e| {
        (((acc as u16 + *e as u16) * 17) % 256).try_into().unwrap()
    })
}
