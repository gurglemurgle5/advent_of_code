use std::fs;

use regex::Regex;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    let yes: Vec<usize> = Regex::new(r"do\(\)")
        .unwrap()
        .find_iter(&input)
        .map(|mat| mat.start())
        .collect();

    let no: Vec<usize> = Regex::new(r"don't\(\)")
        .unwrap()
        .find_iter(&input)
        .map(|mat| mat.start())
        .collect();
    let mut total = 0;

    for mul in re.captures_iter(&input) {
        let index = mul.get(0).unwrap().start();
        let last_yes = yes.iter().filter(|yes| **yes < index).last().unwrap_or(&0);
        let last_no = no.iter().filter(|no| **no < index).last().unwrap_or(&0);

        if last_no > last_yes {
            continue;
        }

        let first: i32 = mul[1].parse().unwrap();
        let second: i32 = mul[2].parse().unwrap();
        total += first * second;
    }
    println!("{total}");
}
