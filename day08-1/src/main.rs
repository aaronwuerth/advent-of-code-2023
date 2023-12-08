use std::collections::HashMap;
use std::io::stdin;

fn main() {
    let directions = stdin().lines().next().unwrap().unwrap().as_bytes().to_vec();

    stdin().lines().next().unwrap().unwrap();

    let graph: HashMap<_, _> = stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();

            let nodes: Vec<_> = line
                .split(&[' ', '=', '(', ')', ','])
                .filter_map(|s| {
                    if s.is_empty() {
                        None
                    } else {
                        Some(s.to_string())
                    }
                })
                .collect();

            (nodes[0].clone(), (nodes[1].clone(), nodes[2].clone()))
        })
        .collect();

    let mut cur = "AAA".to_string();
    let mut cnt = 0;

    while cur != "ZZZ" {
        if directions[cnt % directions.len()] == b'L' {
            cur = graph[&cur].0.clone();
        } else {
            cur = graph[&cur].1.clone();
        }

        cnt += 1;
    }

    println!("{cnt}");
}
