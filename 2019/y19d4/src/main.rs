use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let input = input.split_once('-').unwrap();
    let range: std::ops::RangeInclusive<i32> =
        (input.0.parse().unwrap())..=(input.1.parse().unwrap());

    let mut total_valid = 0;
    'passes: for pass in range {
        let pass = pass.to_string();
        let chars: Vec<char> = pass.chars().collect();
        let mut adjacent = false;
        for i in 0..(chars.len() - 1) {
            if chars[i + 1] < chars[i] {
                continue 'passes;
            }
            if chars[i] == chars[i + 1] {
                adjacent = true;
            }
        }
        if adjacent {
            total_valid += 1;
        }
    }
    dbg!(total_valid);
}
