use std::io::{self, Read};

fn read_inputs() -> Vec<i64> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    buffer
        .split_ascii_whitespace()
        .map(|s| s.parse::<i64>().unwrap()).collect()
}

fn fuel_for_mass(mass: i64) -> i64 {
    let fuel = (mass / 3) - 2;
    if fuel < 0 {
        0
    } else {
        fuel
    }
}

pub fn part1() {
    let output: i64 = read_inputs()
        .iter()
        .map(|mass| fuel_for_mass(*mass))
        .fold(0, |a, b| a + b);

    println!("{}", output);
}

fn net_fuel_for_mass(mass: i64) -> i64 {
    let mut total = 0;
    let mut fuel = mass;
    loop {
        fuel = fuel_for_mass(fuel);
        total += fuel;
        if fuel <= 0 {
            return total;
        }
    }
}

pub fn part2() {
    let output: i64 = read_inputs()
        .iter()
        .map(|mass| net_fuel_for_mass(*mass))
        .fold(0, |a, b| a + b);

    println!("{}", output);
}
