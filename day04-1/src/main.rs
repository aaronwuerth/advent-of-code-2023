use std::io::stdin;
use std::num::IntErrorKind;

fn main() {
    let answer: u32 = stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let mut numbers: Vec<_> = line.split(&[':', '|']).skip(1).map(parse_numbers).collect();
            numbers.iter_mut().for_each(|nums| nums.sort());

            let my_numbers = numbers.remove(1);
            let winning_numbers = numbers.remove(0);

            let mut current_winning_index = 0;

            let n = my_numbers
                .into_iter()
                .filter(|num| {
                    while current_winning_index < winning_numbers.len()
                        && winning_numbers[current_winning_index] < *num
                    {
                        current_winning_index += 1;
                    }

                    current_winning_index < winning_numbers.len()
                        && winning_numbers[current_winning_index] == *num
                })
                .count();

            match n {
                0 => 0,
                n => 2u32.pow((n - 1) as u32),
            }
        })
        .sum();

    println!("{answer}");
}

fn parse_numbers(s: &str) -> Vec<u32> {
    Vec::from_iter(s.split(' ').filter_map(|num| match num.parse::<u32>() {
        Ok(n) => Some(n),
        Err(e) if *e.kind() == IntErrorKind::Empty => None,
        _ => panic!(),
    }))
}
