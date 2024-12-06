use std::{fs, time::Instant};

fn main() {
    let now = Instant::now();

    let mut bitflags = [0u128; 100];

    let input = fs::read_to_string("./input.txt").unwrap();
    let (rules, updates) = input.split_once("\n\n").unwrap();

    rules.lines().for_each(|line| {
        let rule = line.split_once('|').unwrap();
        bitflags[rule.1.parse::<usize>().unwrap()] |= 1 << rule.0.parse::<u128>().unwrap();
    });
    let updates: Vec<Vec<u8>> = updates
        .lines()
        .map(|line| line.split(',').map(|num| num.parse().unwrap()).collect())
        .collect();

    dbg!(now.elapsed());

    let mut sum_valid = 0;
    let mut sum_invalid = 0;
    for mut update in updates {
        let mut is_valid = true;

        for i in 0..(update.len() - 1) {
            let mut j = i + 1;
            while j < update.len() {
                if bitflags[update[i] as usize] & (1 << update[j]) != 0 {
                    is_valid = false;
                    (update[i], update[j]) = (update[j], update[i]);
                    j = i + 1;
                } else {
                    j += 1;
                }
            }
        }

        if is_valid {
            sum_valid += update[update.len() / 2] as u16;
        } else {
            sum_invalid += update[update.len() / 2] as u16;
        }
    }
    dbg!(now.elapsed());

    println!("{sum_valid}");
    println!("{sum_invalid}");
    dbg!(now.elapsed());
}
