use std::num::IntErrorKind;

pub fn parse_numbers(s: &str) -> Vec<u32> {
    Vec::from_iter(s.split(&[' ', '\n', '\r']).filter_map(|num| match num.parse::<u32>() {
        Ok(n) => Some(n),
        Err(e) if *e.kind() == IntErrorKind::Empty => None,
        Err(e) => panic!("Could not parse: \"{}\". Error: {}", s, e),
    }))
}

pub fn gcd(a: usize, b: usize) -> usize {
    let mut a = a;
    let mut b = b;
    let mut t;

    while b != 0 {
        t = b;
        b = a % b;
        a = t;
    }

    a
}
