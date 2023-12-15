use std::io::stdin;

fn main() {
    let mut boxes: Vec<Vec<(Vec<u8>, u8)>> = vec![vec![]; 256];

    stdin().lines().for_each(|line| {
        line.unwrap()
            .as_bytes()
            .split(|c| *c == b',')
            .for_each(|s| {
                let instruction = parse(s);
                let r#box = hash(&instruction.label) as usize;

                match instruction.op {
                    Operation::Add(fl) => {
                        if let Some(lens) =
                            boxes[r#box].iter_mut().find(|l| l.0 == instruction.label)
                        {
                            lens.1 = fl;
                        } else {
                            boxes[r#box].push((instruction.label, fl));
                        }
                    }
                    Operation::Remove => {
                        if let Some(position) =
                            boxes[r#box].iter().position(|l| l.0 == instruction.label)
                        {
                            boxes[r#box].remove(position);
                        }
                    }
                };
            })
    });

    let answer: u64 = boxes
        .iter()
        .enumerate()
        .map(|(i, r#box)| {
            r#box
                .iter()
                .enumerate()
                .map(|(j, lens)| (i as u64 + 1) * (j as u64 + 1) * lens.1 as u64)
                .sum::<u64>()
        })
        .sum();

    println!("{answer}");
}

fn hash(s: &[u8]) -> u8 {
    s.iter().fold(0, |acc, e| {
        (((acc as u16 + *e as u16) * 17) % 256).try_into().unwrap()
    })
}

enum Operation {
    Add(u8),
    Remove,
}

struct Instruction {
    label: Vec<u8>,
    op: Operation,
}

fn parse(s: &[u8]) -> Instruction {
    let mut i = 0;
    let mut label = vec![];

    while i < s.len() && s[i].is_ascii_alphabetic() {
        label.push(s[i]);
        i += 1;
    }

    let op = match s[i] {
        b'=' => Operation::Add(s[i + 1] - b'0'),
        b'-' => Operation::Remove,
        _ => panic!(),
    };

    Instruction { label, op }
}
