use std::io::stdin;

const EXPANSION: usize = 1_000_000;

fn main() {
    let map: Vec<_> = stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            line.as_bytes().to_vec()
        })
        .collect();

    let num_cols = map[0].len();
    let num_rows = map.len();

    let empty_rows: Vec<_> = map
        .iter()
        .enumerate()
        .filter_map(|(i, row)| {
            if row.iter().all(|c| *c == b'.') {
                Some(i)
            } else {
                None
            }
        })
        .collect();

    let empty_cols: Vec<_> = (0..num_cols)
        .filter(|i| (0..num_rows).map(|j| map[j][*i]).all(|c| c == b'.'))
        .collect();

    let galaxies: Vec<_> = map
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(j, c)| if *c == b'#' { Some((i, j)) } else { None })
        })
        .collect();

    let answer: usize = galaxies
        .iter()
        .enumerate()
        .flat_map(|(i, a)| {
            galaxies.iter().skip(i).map(|b| {
                let h = empty_space_between(a.1, b.1, &empty_cols);
                let w = empty_space_between(a.0, b.0, &empty_rows);
                dist(*a, *b) + h + w
            })
        })
        .sum();

    println!("{}", answer);
}

fn dist(p1: (usize, usize), p2: (usize, usize)) -> usize {
    (p1.0.max(p2.0) - p1.0.min(p2.0)) + (p1.1.max(p2.1) - p1.1.min(p2.1))
}

fn empty_space_between(a: usize, b: usize, empty_space: &[usize]) -> usize {
    let first = empty_space.partition_point(|x| x < &a.min(b));
    let last = empty_space.partition_point(|x| x < &a.max(b));
    (EXPANSION - 1) * (last - first)
}
