use std::io::stdin;

fn main() {
    let map: Vec<_> = stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();

            line.as_bytes().to_vec()
        })
        .collect();

    let start = map
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(j, c)| if *c == b'S' { Some((i, j)) } else { None })
        })
        .next()
        .unwrap();

    let mut cnt = 1;
    let mut last = start;
    let mut cur = if [b'|', b'7', b'F']
        .contains(&get_by_pos(step(start, Direction::North, &map), &map))
    {
        step(start, Direction::North, &map)
    } else if [b'-', b'J', b'7'].contains(&get_by_pos(step(start, Direction::East, &map), &map)) {
        step(start, Direction::East, &map)
    } else if [b'|', b'L', b'J'].contains(&get_by_pos(step(start, Direction::South, &map), &map)) {
        step(start, Direction::South, &map)
    } else if [b'-', b'L', b'F'].contains(&get_by_pos(step(start, Direction::West, &map), &map)) {
        step(start, Direction::West, &map)
    } else {
        panic!()
    };

    while cur != start {
        let directions = pipe_to_directions(get_by_pos(cur, &map));
        let next = match step(cur, directions.0, &map) {
            c if c == last => step(cur, directions.1, &map),
            c => c,
        };
        last = cur;
        cur = next;

        cnt += 1;
    }

    println!("{}", cnt / 2);
}

enum Direction {
    North,
    East,
    South,
    West,
}

fn get_by_pos(pos: (usize, usize), map: &[Vec<u8>]) -> u8 {
    map[pos.0][pos.1]
}

fn step(pos: (usize, usize), direction: Direction, map: &[Vec<u8>]) -> (usize, usize) {
    match direction {
        Direction::North => (pos.0.saturating_sub(1), pos.1),
        Direction::East => (pos.0, pos.1.saturating_add(1).min(map[pos.0].len() - 1)),
        Direction::West => (pos.0, pos.1.saturating_sub(1)),
        Direction::South => (pos.0.saturating_add(1).min(map.len() - 1), pos.1),
    }
}

fn pipe_to_directions(pipe: u8) -> (Direction, Direction) {
    match pipe {
        b'|' => (Direction::North, Direction::South),
        b'-' => (Direction::East, Direction::West),
        b'L' => (Direction::North, Direction::East),
        b'J' => (Direction::North, Direction::West),
        b'7' => (Direction::South, Direction::West),
        b'F' => (Direction::South, Direction::East),
        _ => panic!(),
    }
}
