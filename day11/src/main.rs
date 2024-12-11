use num_bigint::BigUint;
use std::{collections::HashMap, fs};

fn main() {
    let input: Vec<BigUint> = fs::read_to_string("./input.txt")
        .unwrap()
        .split(' ')
        .map(|num| num.parse().unwrap())
        .collect();

    let mut stones: HashMap<BigUint, BigUint> = HashMap::new();

    for num in input {
        *stones.entry(num).or_default() += BigUint::from(1u32);
    }

    let mut i = 0;

    loop {
        println!(
            "blinked {i} times, there are {} stones",
            stones
                .iter()
                .fold(0u32.into(), |total: BigUint, (num, count)| total + count)
        );
        let mut new = HashMap::new();
        for (num, count) in stones {
            if num == 0u32.into() {
                *new.entry(1u32.into()).or_default() += count;
            } else if num.to_string().len() % 2 == 0 {
                let num = num.to_string();
                *new.entry(num[0..(num.len() / 2)].parse().unwrap())
                    .or_default() += count.clone();
                *new.entry(num[(num.len() / 2)..].parse().unwrap())
                    .or_default() += count;
            } else {
                *new.entry(num * BigUint::from(2024u32)).or_default() += count;
            }
        }
        stones = new;
        i += 1;
    }
}
