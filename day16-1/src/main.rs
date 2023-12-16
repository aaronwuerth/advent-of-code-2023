use std::collections::HashSet;
use std::io::stdin;

fn main() {
    let (height, map) =
        stdin()
            .lines()
            .enumerate()
            .fold((0, vec![]), |(_, mut map), (cnt, line)| {
                map.extend_from_slice(line.unwrap().as_bytes());

                (cnt + 1, map)
            });

    let width = map.len() / height;

    let to_index = |(x, y): &(usize, usize)| y * width + x;

    let get_next_node = |(node, direction): &((usize, usize), _)| {
        let dir = match direction {
            Directions::North => (Some(node.0), node.1.checked_sub(1)),
            Directions::East => (node.0.checked_add(1), Some(node.1)),
            Directions::South => (Some(node.0), node.1.checked_add(1)),
            Directions::West => (node.0.checked_sub(1), Some(node.1)),
        };

        if dir.0.is_none() || dir.1.is_none() || dir.0.unwrap() >= width || dir.1.unwrap() >= height
        {
            return None;
        }

        Some(((dir.0.unwrap(), dir.1.unwrap()), *direction))
    };

    let mut branches = vec![((0, 0), Directions::East)];
    let mut visited_nodes = HashSet::new();
    let mut visited_tiles = HashSet::new();

    while let Some(current_node) = branches.pop() {
        if visited_nodes.contains(&current_node) {
            continue;
        }
        visited_nodes.insert(current_node);
        visited_tiles.insert(current_node.0);
        let (current_tile, direction) = current_node;

        match map[to_index(&current_node.0)] {
            b'.' => {
                if let Some(next_state) = get_next_node(&current_node) {
                    branches.push(next_state)
                }
            }
            b'/' => match direction {
                Directions::North => {
                    if let Some(next_node) = get_next_node(&(current_tile, Directions::East)) {
                        branches.push(next_node)
                    }
                }
                Directions::East => {
                    if let Some(next_node) = get_next_node(&(current_tile, Directions::North)) {
                        branches.push(next_node)
                    }
                }
                Directions::South => {
                    if let Some(next_node) = get_next_node(&(current_tile, Directions::West)) {
                        branches.push(next_node)
                    }
                }
                Directions::West => {
                    if let Some(next_node) = get_next_node(&(current_tile, Directions::South)) {
                        branches.push(next_node)
                    }
                }
            },
            b'\\' => match direction {
                Directions::North => {
                    if let Some(next_node) = get_next_node(&(current_tile, Directions::West)) {
                        branches.push(next_node)
                    }
                }
                Directions::East => {
                    if let Some(next_node) = get_next_node(&(current_tile, Directions::South)) {
                        branches.push(next_node)
                    }
                }
                Directions::South => {
                    if let Some(next_node) = get_next_node(&(current_tile, Directions::East)) {
                        branches.push(next_node)
                    }
                }
                Directions::West => {
                    if let Some(next_node) = get_next_node(&(current_tile, Directions::North)) {
                        branches.push(next_node)
                    }
                }
            },
            b'|' => match direction {
                Directions::North | Directions::South => {
                    if let Some(next_node) = get_next_node(&(current_tile, direction)) {
                        branches.push(next_node)
                    }
                }
                Directions::East | Directions::West => {
                    if let Some(next_node) = get_next_node(&(current_tile, Directions::North)) {
                        branches.push(next_node)
                    }
                    if let Some(next_node) = get_next_node(&(current_tile, Directions::South)) {
                        branches.push(next_node)
                    }
                }
            },
            b'-' => match direction {
                Directions::North | Directions::South => {
                    if let Some(next_node) = get_next_node(&(current_tile, Directions::East)) {
                        branches.push(next_node)
                    }
                    if let Some(next_node) = get_next_node(&(current_tile, Directions::West)) {
                        branches.push(next_node)
                    }
                }
                Directions::East | Directions::West => {
                    if let Some(next_node) = get_next_node(&(current_tile, direction)) {
                        branches.push(next_node)
                    }
                }
            },
            _ => panic!(),
        }
    }

    println!("{}", visited_tiles.len());
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Directions {
    North,
    East,
    South,
    West,
}
