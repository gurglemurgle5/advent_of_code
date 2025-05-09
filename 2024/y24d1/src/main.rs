use std::fs;

fn main() {
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = fs::read_to_string("./input.txt")
        .unwrap()
        .trim()
        .lines()
        .map(|line| {
            let (first, second) = line.split_once(" ").unwrap();
            let second = second.trim();
            (
                first.parse::<i32>().unwrap(),
                second.parse::<i32>().unwrap(),
            )
        })
        .unzip();
    left.sort();
    right.sort();
    let mut distance = 0;
    for i in 0..left.len() {
        distance += (left[i] - right[i]).abs();
    }
    println!("{distance}");

    let mut score = 0;

    for i in 0..left.len() {
        let count = right.iter().filter(|val| **val == left[i]).count();
        score += count as i32 * left[i];
    }

    println!("{score}");
}
