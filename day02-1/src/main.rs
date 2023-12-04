use std::io::stdin;

fn main() {
    let anwser: u32 = stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            parse_line(&line)
        })
        .filter_map(|game| {
            let max_number_of_cubes = game.max_cube_count();
            if max_number_of_cubes.0 <= 12
                && max_number_of_cubes.1 <= 13
                && max_number_of_cubes.2 <= 14
            {
                Some(game.id)
            } else {
                None
            }
        })
        .sum();

    println!("{anwser}");
}

struct Game {
    id: u32,
    rounds: Vec<(u32, u32, u32)>,
}

impl Game {
    fn max_cube_count(&self) -> (u32, u32, u32) {
        self.rounds.iter().fold((0, 0, 0), |max, r| {
            (max.0.max(r.0), max.1.max(r.1), max.2.max(r.2))
        })
    }
}

fn parse_line(s: &str) -> Game {
    let mut colon_split = s.split(':');

    let id = parse_digits(colon_split.next().unwrap());

    let rounds: Vec<_> = colon_split
        .next()
        .unwrap()
        .split(';')
        .map(|round| {
            round.split(',').fold((0, 0, 0), |acc, cube| {
                let num = parse_digits(cube);

                if cube.ends_with("red") {
                    (acc.0 + num, acc.1, acc.2)
                } else if cube.ends_with("green") {
                    (acc.0, acc.1 + num, acc.2)
                } else {
                    (acc.0, acc.1, acc.2 + num)
                }
            })
        })
        .collect();

    Game { id, rounds }
}

fn parse_digits(s: &str) -> u32 {
    s.chars()
        .filter_map(|c| {
            if c.is_ascii_digit() {
                Some(c.to_digit(10).unwrap())
            } else {
                None
            }
        })
        .fold(0, |prod, a| prod * 10 + a)
}
