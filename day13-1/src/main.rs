use std::io::stdin;

fn main() {
    let answer = stdin()
        .lines()
        .chain([Ok("".to_string())])
        .map(|line| line.unwrap())
        .fold(
            (0, vec![], vec![]),
            |(mut acc, mut rows, mut cols), line| {
                if line.is_empty() {
                    if left_mirror(&rows) != 0 {
                        let e = left_mirror(&rows) + 1;
                        acc += 100 * (e / 2);
                    } else if right_mirror(&rows) != rows.len() - 1 {
                        let s = right_mirror(&rows);
                        acc += 100 * (s + (rows.len() - s) / 2);
                    } else if left_mirror(&cols) != 0 {
                        let e = left_mirror(&cols) + 1;
                        acc += e / 2;
                    } else if right_mirror(&cols) != cols.len() - 1 {
                        let s = right_mirror(&cols);
                        acc += s + (cols.len() - s) / 2;
                    }

                    rows.clear();
                    cols.clear();
                } else {
                    if cols.is_empty() {
                        cols.resize(line.as_bytes().len(), 0);
                    }

                    let mut row = 0;

                    for (c, col) in line.as_bytes().iter().zip(cols.iter_mut()) {
                        row = row << 1
                            | match c {
                                b'.' => 0,
                                b'#' => 1,
                                _ => panic!(),
                            };

                        *col = *col << 1
                            | match c {
                                b'.' => 0,
                                b'#' => 1,
                                _ => panic!(),
                            };
                    }

                    rows.push(row);
                }

                (acc, rows, cols)
            },
        )
        .0;

    println!("{answer}");
}

fn left_mirror(s: &[usize]) -> usize {
    let s = if s.len() % 2 == 0 {
        s
    } else {
        &s[0..s.len() - 1]
    };
    let mut end = s.len() - 1;
    let mut left = 0;
    let mut right = end;

    while left < right {
        if s[left] != s[right] {
            end = end.saturating_sub(2);
            left = 0;
            right = end;
        } else {
            left += 1;
            right -= 1;
        }
    }

    end
}

fn right_mirror(s: &[usize]) -> usize {
    let offset = s.len() % 2;
    let s = if s.len() % 2 == 0 { s } else { &s[1..] };
    let mut start = 0;
    let mut left = 0;
    let mut right = s.len() - 1;


    while left < right {
        if s[left] != s[right] {
            start += 2;
            left = start;
            right = s.len() - 1;
        } else {
            left += 1;
            right -= 1;
        }
    }

    offset + start.min(right)
}
