use std::{fs, time::Instant};

fn main() {
    let now = Instant::now();

    let input = fs::read_to_string("./input.txt").unwrap();
    let (rules, updates) = input.trim().split_once("\n\n").unwrap();

    let rules: Vec<(i32, i32)> = rules
        .lines()
        .map(|line| {
            let rule = line.split_once('|').unwrap();
            (rule.0.parse().unwrap(), rule.1.parse().unwrap())
        })
        .collect();

    let updates: Vec<Vec<i32>> = updates
        .lines()
        .map(|line| line.split(',').map(|num| num.parse().unwrap()).collect())
        .collect();

    dbg!(now.elapsed());

    let mut sum_valid = 0;
    let mut sum_invalid = 0;
    for mut update in updates {
        let mut is_valid = true;
        let mut should_loop = true;

        // ok dumb idea, let's try looping over all the rules and swapping the placement of invalid pages
        while should_loop {
            should_loop = false;
            for rule in rules.iter() {
                let indices = update
                    .iter()
                    .position(|page| *page == rule.0)
                    .zip(update.iter().position(|page| *page == rule.1));
                if let Some((pos0, pos1)) = indices {
                    if pos0 > pos1 {
                        is_valid = false;
                        (update[pos0], update[pos1]) = (update[pos1], update[pos0]);
                        should_loop = true;
                    }
                }
            }
        }
        if is_valid {
            sum_valid += update[update.len() / 2];
        } else {
            sum_invalid += update[update.len() / 2];
        }
    }

    dbg!(now.elapsed());

    println!("{sum_valid}");
    println!("{sum_invalid}");
}
