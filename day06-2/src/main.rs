use std::io::stdin;

fn main() {
    let mut buf = String::new();

    stdin().read_line(&mut buf).unwrap();
    let time: f64 = buf
        .split(':')
        .nth(1)
        .unwrap()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse()
        .unwrap();
    buf.clear();
    stdin().read_line(&mut buf).unwrap();
    let distance: f64 = buf
        .split(':')
        .nth(1)
        .unwrap()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse()
        .unwrap();

    let shortest = (0.5 * (time - (time.powi(2) - 4.0 * (distance + 1.0)).sqrt())).ceil();
    let longest = (0.5 * (time + (time.powi(2) - 4.0 * (distance + 1.0)).sqrt())).floor();
    let anwser = (longest - shortest) as u32 + 1;

    println!("{anwser}");
}
