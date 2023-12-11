use std::io::stdin;

fn main() {
    let mut map: Vec<_> = stdin()
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
        .rev()
        .collect();

    let empty_cols: Vec<_> = (0..num_cols)
        .filter(|i| (0..num_rows).map(|j| map[j][*i]).all(|c| c == b'.'))
        .rev()
        .collect();

    for empty_row in empty_rows.iter() {
        map.insert(*empty_row, map[*empty_row].clone());
    }

    for empty_col in empty_cols.iter() {
        map.iter_mut().for_each(|row| row.insert(*empty_col, b'.'));
    }

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
            galaxies.iter().enumerate().filter_map(move |(j, b)| {
                if j <= i {
                    None
                } else {
                    Some(dist(*a, *b))
                }
            })
        })
        .sum();

    println!("{}", answer);
}

fn dist(p1: (usize, usize), p2: (usize, usize)) -> usize {
    (p1.0.max(p2.0) - p1.0.min(p2.0)) + (p1.1.max(p2.1) - p1.1.min(p2.1))
}
