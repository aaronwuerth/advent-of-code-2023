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

    let mut answer = 0;

    for x in 0..width {
        answer = answer.max(energized_tiles(
            &map,
            width,
            height,
            ((x, 0), Directions::South),
        ));
        answer = answer.max(energized_tiles(
            &map,
            width,
            height,
            ((x, height - 1), Directions::North),
        ));
    }
    for y in 0..height {
        answer = answer.max(energized_tiles(
            &map,
            width,
            height,
            ((0, y), Directions::East),
        ));
        answer = answer.max(energized_tiles(
            &map,
            width,
            height,
            ((width - 1, y), Directions::West),
        ));
    }

    println!("{answer}");
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Directions {
    North,
    East,
    South,
    West,
}

fn direction_left(direction: Directions) -> Directions {
    match direction {
        Directions::North => Directions::West,
        Directions::East => Directions::North,
        Directions::South => Directions::East,
        Directions::West => Directions::South,
    }
}

fn direction_right(direction: Directions) -> Directions {
    (0..3).fold(direction, |direction, _| direction_left(direction))
}

fn energized_tiles(
    map: &[u8],
    width: usize,
    height: usize,
    start: ((usize, usize), Directions),
) -> usize {
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

    let mut branches = vec![start];
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
                Directions::North | Directions::South => {
                    if let Some(next_node) =
                        get_next_node(&(current_tile, direction_right(direction)))
                    {
                        branches.push(next_node)
                    }
                }
                Directions::East | Directions::West => {
                    if let Some(next_node) =
                        get_next_node(&(current_tile, direction_left(direction)))
                    {
                        branches.push(next_node)
                    }
                }
            },
            b'\\' => match direction {
                Directions::North | Directions::South => {
                    if let Some(next_node) =
                        get_next_node(&(current_tile, direction_left(direction)))
                    {
                        branches.push(next_node)
                    }
                }
                Directions::East | Directions::West => {
                    if let Some(next_node) =
                        get_next_node(&(current_tile, direction_right(direction)))
                    {
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

    visited_tiles.len()
}
