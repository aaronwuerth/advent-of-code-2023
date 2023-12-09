use std::str::FromStr;
use std::fmt::Debug;

pub fn parse_numbers<T: FromStr>(s: &str) -> Vec<T>
where
    <T as FromStr>::Err: Debug,
{
    Vec::from_iter(s.split(&[' ', '\n', '\r']).filter_map(|num| {
        if num.is_empty() {
            None
        } else {
            Some(num.parse::<T>().unwrap())
        }
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
