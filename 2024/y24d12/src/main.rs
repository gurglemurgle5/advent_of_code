use std::{collections::HashSet, fs};

fn main() {
    let input: Vec<Vec<char>> = fs::read_to_string("./input.txt")
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut checked_spots: HashSet<(usize, usize)> = HashSet::new();

    let mut cost = 0;

    for row in 0..input.len() {
        for col in 0..input[row].len() {
            if checked_spots.contains(&(row, col)) {
                continue;
            }

            let mut set = HashSet::new();
            get_region(input[row][col], row, col, &input, &mut set);

            let area = set.len();
            let mut top_fences = HashSet::new();
            let mut bottom_fences = HashSet::new();
            let mut left_fences = HashSet::new();
            let mut right_fences = HashSet::new();
            let mut perimeter = 0;
            for (row, col) in &set {
                let (row, col) = (*row, *col);
                if !(row > 0 && set.contains(&(row - 1, col))) {
                    top_fences.insert((row, col));
                }
                if !(col > 0 && set.contains(&(row, col - 1))) {
                    left_fences.insert((row, col));
                }
                if !set.contains(&(row + 1, col)) {
                    bottom_fences.insert((row, col));
                }
                if !set.contains(&(row, col + 1)) {
                    right_fences.insert((row, col));
                }
            }
            while !top_fences.is_empty() {
                perimeter += 1;
                let (row, mut col) = top_fences.iter().next().unwrap().to_owned();
                while col != 0 && top_fences.contains(&(row, col - 1)) {
                    col -= 1;
                }
                while top_fences.remove(&(row, col)) {
                    col += 1;
                }
            }
            while !bottom_fences.is_empty() {
                perimeter += 1;
                let (row, mut col) = bottom_fences.iter().next().unwrap().to_owned();
                while col != 0 && bottom_fences.contains(&(row, col - 1)) {
                    col -= 1;
                }
                while bottom_fences.remove(&(row, col)) {
                    col += 1;
                }
            }
            while !left_fences.is_empty() {
                perimeter += 1;
                let (mut row, col) = left_fences.iter().next().unwrap().to_owned();
                while row != 0 && left_fences.contains(&(row - 1, col)) {
                    row -= 1;
                }
                while left_fences.remove(&(row, col)) {
                    row += 1;
                }
            }
            while !right_fences.is_empty() {
                perimeter += 1;
                let (mut row, col) = right_fences.iter().next().unwrap().to_owned();
                while row != 0 && right_fences.contains(&(row - 1, col)) {
                    row -= 1;
                }
                while right_fences.remove(&(row, col)) {
                    row += 1;
                }
            }

            checked_spots.extend(&set);

            cost += perimeter * area;

            dbg!(input[row][col], area, perimeter);
        }
    }
    dbg!(cost);
}

fn get_region(
    target: char,
    row: usize,
    col: usize,
    map: &Vec<Vec<char>>,
    set: &mut HashSet<(usize, usize)>,
) {
    if target != map[row][col] {
        return;
    }
    if !set.insert((row, col)) {
        return;
    }
    if row > 0 {
        get_region(target, row - 1, col, map, set);
    }
    if col > 0 {
        get_region(target, row, col - 1, map, set);
    }
    if row + 1 < map.len() {
        get_region(target, row + 1, col, map, set);
    }
    if col + 1 < map[row].len() {
        get_region(target, row, col + 1, map, set);
    }
}
