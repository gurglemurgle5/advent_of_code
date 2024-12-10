use std::{collections::HashMap, fs};

fn main() {
    let mut map: Vec<Vec<char>> = fs::read_to_string("./input.txt")
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    // search for antennas
    for (y, row) in map.iter().enumerate() {
        for (x, char) in row.iter().enumerate() {
            if *char != '.' {
                antennas
                    .entry(*char)
                    .or_default()
                    .push((y as i32, x as i32));
            }
        }
    }
    // mark antinodes

    for (_, antennas) in antennas {
        for left in antennas.iter() {
            for right in antennas.iter() {
                if left == right {
                    continue;
                }
                // extend from left to right
                let drow = right.0 - left.0;
                let dcol = right.1 - left.1;
                let (drow, dcol) = reduce(drow, dcol);
                let mut row = right.0;
                let mut col = right.1;
                while row >= 0
                    && row < map.len() as i32
                    && col >= 0
                    && col < map[row as usize].len() as i32
                {
                    map[row as usize][col as usize] = '#';
                    row += drow;
                    col += dcol;
                }
            }
        }
    }

    // find antinodes
    let mut total = 0;
    for row in map {
        for char in row {
            if char == '#' {
                total += 1;
            }
        }
    }
    println!("{total}");
}

fn gcf(mut a: i32, mut b: i32) -> i32 {
    let mut t;
    while b != 0 {
        t = b;
        b = a % b;
        a = t;
    }
    a.abs()
}

fn reduce(a: i32, b: i32) -> (i32, i32) {
    let gcf = gcf(a, b);
    (a / gcf, b / gcf)
}
