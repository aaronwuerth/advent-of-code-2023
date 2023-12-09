use std::io::stdin;
use util::parse_numbers;

fn main() {
    let answer: i32 = stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let mut data = vec![parse_numbers::<i32>(&line)];

            while data.last().unwrap().iter().any(|x| *x != 0) {
                let last = data.last().unwrap();

                data.push(last.windows(2).map(|p| p[1] - p[0]).collect());
            }

            data.iter()
                .rev()
                .map(|d| d.last().unwrap())
                .sum::<i32>()
        })
        .sum();

    println!("{answer}");
}
