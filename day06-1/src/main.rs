use std::io::stdin;
use util::parse_numbers;

fn main() {
    let mut buf = String::new();

    stdin().read_line(&mut buf).unwrap();
    let times: Vec<u32> = parse_numbers(buf.split(':').nth(1).unwrap());
    buf.clear();
    stdin().read_line(&mut buf).unwrap();
    let distances: Vec<u32> = parse_numbers(buf.split(':').nth(1).unwrap());

    let anwser: u32 = times
        .iter()
        .zip(distances)
        .map(|(time, distance)| {
            let time = *time as f64;
            let distance = distance as f64;
            let shortest = (0.5 * (time - (time.powi(2) - 4.0 * (distance + 1.0)).sqrt())).ceil();
            let longest = (0.5 * (time + (time.powi(2) - 4.0 * (distance + 1.0)).sqrt())).floor();

            (longest - shortest) as u32 + 1
        })
        .product();

    println!("{anwser}");
}
