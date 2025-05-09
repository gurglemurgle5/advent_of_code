use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let total: i32 = input
        .lines()
        .map(|line| line.parse::<i32>().unwrap() / 3 - 2)
        .sum();

    dbg!(total);
}
