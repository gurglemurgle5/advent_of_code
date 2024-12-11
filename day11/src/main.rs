use std::{collections::HashMap, fs};

fn main() {
    let input: Vec<u64> = fs::read_to_string("./input.txt")
        .unwrap()
        .split(' ')
        .map(|num| num.parse().unwrap())
        .collect();

    let mut stones: HashMap<u64, u64> = HashMap::new();

    for num in input {
        *stones.entry(num).or_default() += 1;
    }

    let mut i = 0;

    loop {
        println!(
            "blinked {i} times, there are {} stones",
            stones.iter().fold(0, |total, (num, count)| total + count)
        );
        let mut new = HashMap::new();
        for (num, count) in stones {
            if num == 0 {
                *new.entry(1).or_default() += count;
            } else if num.to_string().len() % 2 == 0 {
                let num = num.to_string();
                *new.entry(num[0..(num.len() / 2)].parse().unwrap())
                    .or_default() += count;
                *new.entry(num[(num.len() / 2)..].parse().unwrap())
                    .or_default() += count;
            } else {
                *new.entry(num * 2024).or_default() += count;
            }
        }
        stones = new;
        i += 1;
    }
}
