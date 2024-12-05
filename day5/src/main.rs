use std::{collections::HashMap, fs, time::Instant};

fn main() {
    let now = Instant::now();

    let input = fs::read_to_string("./input.txt").unwrap();
    let (rules, updates) = input.split_once("\n\n").unwrap();

    let rules: Vec<(u8, u8)> = rules
        .lines()
        .map(|line| {
            let rule = line.split_once('|').unwrap();
            (rule.0.parse().unwrap(), rule.1.parse().unwrap())
        })
        .collect();
    let updates: Vec<Vec<u8>> = updates
        .lines()
        .map(|line| line.split(',').map(|num| num.parse().unwrap()).collect())
        .collect();

    let mut hashmap: HashMap<u8, Vec<u8>> = HashMap::new();

    for rule in rules {
        hashmap.entry(rule.1).or_default().push(rule.0);
    }

    dbg!(now.elapsed());

    let mut sum_valid = 0;
    let mut sum_invalid = 0;
    for mut update in updates {
        let mut is_valid = true;

        for i in 0..(update.len() - 1) {
            let mut j = i + 1;
            while j < update.len() {
                if hashmap
                    .get(&update[i])
                    .map(|vec| vec.contains(&update[j]))
                    .unwrap_or(false)
                {
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
