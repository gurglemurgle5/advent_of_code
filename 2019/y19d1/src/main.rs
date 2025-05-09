use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let total: i32 = input
        .lines()
        .map(|line| {
            let mut fuel = line.parse::<i32>().unwrap() / 3 - 2;

            let mut remaining = fuel;

            loop {
                let required = remaining / 3 - 2;
                if required <= 0 {
                    break;
                }
                fuel += required;
                remaining = required;
            }
            fuel
        })
        .sum();

    dbg!(total);
}
