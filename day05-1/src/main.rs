use std::io::stdin;
use std::num::IntErrorKind;

fn main() {
    let seeds = parse_numbers(
        stdin()
            .lines()
            .next()
            .unwrap()
            .unwrap()
            .split(':')
            .nth(1)
            .unwrap(),
    );

    let mut skip_line = true;
    stdin().lines().next().unwrap().unwrap();

    let mut tables: Vec<(Vec<(u32, u32, u32)>, Vec<Node>)> = vec![];

    stdin().lines().for_each(|line| {
        let line = line.unwrap();

        if skip_line {
            tables.push((vec![], vec![]));
            skip_line = false;
        } else {
            let map = parse_numbers(&line);
            let last = tables.last_mut().unwrap();

            if map.is_empty() {
                skip_line = true;
                last.1 = construct_binary_tree(&last.0);
            } else {
                last.0.push((map[1], map[1] + (map[2] - 1), map[0]));
            }
        }
    });
    let last = tables.last_mut().unwrap();
    last.1 = construct_binary_tree(&last.0);

    let mut locations: Vec<u32> = vec![];
    for seed in seeds.iter() {
        let mut keys = vec![*seed];
        let mut new_keys = vec![];

        for table in tables.iter() {
            for key in keys.iter() {
                let matching_maps = query(&table.1, *key);

                if matching_maps.is_empty() {
                    new_keys.push(*key);
                } else {
                    new_keys.extend(matching_maps.iter().map(|map| {
                        let map = table.0[*map];

                        let offset = key - map.0;

                        map.2 + offset
                    }));
                }
            }
            std::mem::swap(&mut keys, &mut new_keys);
            new_keys.clear();
        }
        locations.extend(keys.iter());
    }

    locations.sort();
    println!("{}", locations[0]);
}

#[derive(Clone, Debug)]
struct Node {
    start: u32,
    end: u32,
    segments: Vec<usize>,
}

fn construct_binary_tree(data: &[(u32, u32, u32)]) -> Vec<Node> {
    let mut end_points = vec![];

    for d in data.iter() {
        end_points.push(d.0);
        end_points.push(d.1);
    }
    end_points.sort_unstable();
    end_points.dedup();

    // construct leaf intervals
    let mut leafes = vec![];

    leafes.push(Node {
        start: u32::MIN,
        end: end_points[0].saturating_sub(1),
        segments: vec![],
    });

    (0..end_points.len()).for_each(|i| {
        let last = leafes.last().unwrap();
        let end = end_points[i];

        if last.end != end {
            if last.end + 1 != end {
                leafes.push(Node {
                    start: last.end + 1,
                    end: end - 1,
                    segments: vec![],
                });
            }
            leafes.push(Node {
                start: end,
                end,
                segments: vec![],
            });
        }
    });

    let last = leafes.last().unwrap();
    if last.end != u32::MAX {
        leafes.push(Node {
            start: last.end + 1,
            end: u32::MAX,
            segments: vec![],
        });
    }

    let n = leafes.len();
    let depth = if n == 1 { 0 } else { (n - 1).ilog2() + 1 };
    let full_depth = n.ilog2();

    let non_leafs = n - (1 << full_depth);
    let second_to_last = 1 << (depth - 1);
    let last = !(!0 << depth);

    let deepest_leafs = if non_leafs == 0 { n } else { 2 * non_leafs };
    let mut tree: Vec<Node> = vec![
        Node {
            start: 0,
            end: 0,
            segments: Vec::new()
        };
        last + deepest_leafs
    ];
    tree[last..].clone_from_slice(&leafes[..deepest_leafs]);
    tree[last - (second_to_last - non_leafs) % second_to_last..last].clone_from_slice(&leafes[deepest_leafs..]);

    for i in (0..=depth).rev() {
        let start_children = !(!0 << i);
        let start_parents = start_children >> 1;
        let end_children = tree.len().min((start_children << 1) | 1);
        let end_parents = start_children;

        (start_parents..end_parents)
            .zip((start_children..end_children).step_by(2))
            .for_each(|(parent, left_child)| {
                let right_child = if left_child + 1 < tree.len() {
                    left_child + 1
                } else {
                    left_child
                };
                tree[parent].start = tree[left_child].start;
                tree[parent].end = tree[right_child].end;
            });
    }

    for (i, d) in data.iter().enumerate() {
        insert(&mut tree, 0, d, i);
    }

    tree
}

fn insert(tree: &mut Vec<Node>, node: usize, segment: &(u32, u32, u32), data: usize) {
    if tree[node].start >= segment.0 && tree[node].end <= segment.1 {
        tree[node].segments.push(data);
    } else {
        let left_child = 2 * node + 1;
        let right_child = 2 * node + 2;
        if left_child < tree.len()
            && tree[left_child].start <= segment.1
            && segment.0 <= tree[left_child].end
        {
            insert(tree, left_child, segment, data);
        }
        if right_child < tree.len()
            && tree[right_child].start <= segment.1
            && segment.0 <= tree[right_child].end
        {
            insert(tree, right_child, segment, data);
        }
    }
}

fn query(tree: &Vec<Node>, p: u32) -> Vec<usize> {
    query_node(tree, 0, p)
}

fn query_node(tree: &Vec<Node>, node: usize, p: u32) -> Vec<usize> {
    let mut result = tree[node].segments.clone();

    let left_child = 2 * node + 1;
    let right_child = 2 * node + 2;
    if left_child < tree.len() && tree[left_child].start <= p && p <= tree[left_child].end {
        result.extend_from_slice(&query_node(tree, left_child, p));
    }
    if right_child < tree.len() && tree[right_child].start <= p && p <= tree[right_child].end {
        result.extend_from_slice(&query_node(tree, right_child, p));
    }

    result
}

fn parse_numbers(s: &str) -> Vec<u32> {
    Vec::from_iter(s.split(' ').filter_map(|num| match num.parse::<u32>() {
        Ok(n) => Some(n),
        Err(e) if *e.kind() == IntErrorKind::Empty => None,
        _ => panic!(),
    }))
}
