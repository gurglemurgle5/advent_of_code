use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let orbits: HashMap<&str, &str> = input
        .lines()
        .map(|line| line.split_once(')').unwrap())
        .map(|(a, b)| (b, a))
        .collect();

    let mut you = Vec::new();
    let mut san = Vec::new();

    let mut current = "YOU";
    while current != "COM" {
        current = orbits.get(current).unwrap();
        you.push(current);
    }

    current = "SAN";
    while current != "COM" {
        current = orbits.get(current).unwrap();
        san.push(current);
    }

    while you.last().unwrap() == san.last().unwrap() {
        you.pop();
        san.pop();
    }

    dbg!(you.len() + san.len());
}
