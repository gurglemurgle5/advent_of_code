use std::{collections::HashSet, fs};

fn main() {
    let input: Vec<Vec<u8>> = fs::read_to_string("./input.txt")
        .unwrap()
        .lines()
        .map(|line| line.chars().map(|num| num as u8 & 0x0f).collect())
        .collect();
    let mut score = 0;
    let mut rating = 0;
    for row in 0..input.len() {
        for col in 0..input[row].len() {
            score += calc_score(row, col, 0, &input).len();
            rating += calc_rating(row, col, 0, &input);
        }
    }
    println!("{score}");
    println!("{rating}");
}

fn calc_score(row: usize, col: usize, height: u8, map: &Vec<Vec<u8>>) -> HashSet<(usize, usize)> {
    let mut set = HashSet::new();
    if map[row][col] != height {
        return set;
    }
    if height == 9 {
        set.insert((row, col));
        return set;
    }
    if row > 0 {
        set.extend(calc_score(row - 1, col, height + 1, map));
    }
    if col > 0 {
        set.extend(calc_score(row, col - 1, height + 1, map));
    }
    if row + 1 < map.len() {
        set.extend(calc_score(row + 1, col, height + 1, map));
    }
    if col + 1 < map[row].len() {
        set.extend(calc_score(row, col + 1, height + 1, map));
    }
    set
}

fn calc_rating(row: usize, col: usize, height: u8, map: &Vec<Vec<u8>>) -> i32 {
    if map[row][col] != height {
        return 0;
    }
    if height == 9 {
        return 1;
    }
    let mut rating = 0;
    if row > 0 {
        rating += calc_rating(row - 1, col, height + 1, map);
    }
    if col > 0 {
        rating += calc_rating(row, col - 1, height + 1, map);
    }
    if row + 1 < map.len() {
        rating += calc_rating(row + 1, col, height + 1, map);
    }
    if col + 1 < map[row].len() {
        rating += calc_rating(row, col + 1, height + 1, map);
    }
    rating
}
