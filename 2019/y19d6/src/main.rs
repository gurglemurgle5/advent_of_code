use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let orbits: HashMap<&str, &str> = input
        .lines()
        .map(|line| line.split_once(')').unwrap())
        .map(|(a, b)| (b, a))
        .collect();

    let mut total = 0;

    for &orbit in orbits.keys() {
        let mut current = orbit;
        while current != "COM" {
            total += 1;
            current = orbits.get(current).unwrap();
        }
    }

    dbg!(total);
}
