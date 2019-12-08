use std::io::{self, Read};

mod day1;

fn read_inputs() -> Vec<i64> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    buffer
        .split_ascii_whitespace()
        .map(|s| s.parse::<i64>().unwrap()).collect()
}

fn main() {
    day1::part2(&read_inputs());
}