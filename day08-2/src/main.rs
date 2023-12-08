use std::collections::HashMap;
use std::io::stdin;
use util::gcd;

fn main() {
    let directions = stdin().lines().next().unwrap().unwrap().as_bytes().to_vec();

    stdin().lines().next().unwrap().unwrap();

    let graph: HashMap<_, _> = stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();

            let mut nodes: Vec<_> = line
                .split(&[' ', '=', '(', ')', ','])
                .filter_map(|s| {
                    if s.is_empty() {
                        None
                    } else {
                        Some(s.as_bytes().to_vec())
                    }
                })
                .collect();

            let right = nodes.pop().unwrap();
            let left = nodes.pop().unwrap();
            let key = nodes.pop().unwrap();
            assert_eq!(right.len(), 3);
            assert_eq!(left.len(), 3);
            assert_eq!(key.len(), 3);
            (key, (left, right))
        })
        .collect();

    let mut cur: Vec<_> = graph.keys().filter(|s| s[2] == b'A').cloned().collect();
    let mut path_lengths = vec![None; cur.len()];
    let mut cnt = 0;

    while path_lengths.iter().any(Option::is_none) {
        if directions[cnt % directions.len()] == b'L' {
            cur.iter_mut().for_each(|s| *s = graph[s].0.clone());
        } else {
            cur.iter_mut().for_each(|s| *s = graph[s].1.clone());
        }

        cnt += 1;

        cur.iter()
            .zip(path_lengths.iter_mut())
            .for_each(|(node, length)| {
                if length.is_none() && node[2] == b'Z' {
                    *length = Some(cnt);
                }
            });
    }

    let answer = path_lengths
        .iter()
        .map(|n| n.unwrap())
        .fold(1, |a, b| a * b / gcd(a, b));

    println!("{answer}");
}
