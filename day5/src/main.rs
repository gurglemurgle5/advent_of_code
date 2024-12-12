use std::{fs, time::Instant};

fn main() {
    let now = Instant::now();

    // good luck figuring out how this works
    let mut rule_data = [0u8; 256 * 32];

    let input = fs::read_to_string("./input.txt").unwrap();
    let (rules, updates) = input.split_once("\n\n").unwrap();

    rules.lines().for_each(|line| {
        let line = line.as_bytes();
        let left = (line[0] & 0xf) << 4 | (line[1] & 0xf);
        let right = (line[3] & 0xf) << 4 | (line[4] & 0xf);
        rule_data[right as usize * 32 + ((left as usize & 0b1111_1000) >> 3)] |=
            1 << (left & 0b0111);
    });

    let mut sum_valid = 0;
    let mut sum_invalid = 0;
    updates.lines().for_each(|line| {
        let mut update: Vec<u8> = line
            .split(',')
            .map(|num| {
                let num = num.as_bytes();
                (num[0] & 0xf) << 4 | (num[1] & 0xf)
            })
            .collect();
        let mut is_valid = true;

        // so it turns out incorrectly ordered pages are always next to each other, allowing for
        // more optimization
        let mut i = 0;
        while i < (update.len() - 1) {
            if rule_data[update[i] as usize * 32 + ((update[i + 1] as usize & 0b1111_1000) >> 3)]
                >> (update[i + 1] & 0b0111)
                & 1
                == 1
            {
                is_valid = false;
                (update[i], update[i + 1]) = (update[i + 1], update[i]);
                if i != 0 {
                    // We need to go back to check if the previous order is invalid
                    let mut j = i - 1;
                    while rule_data
                        [update[j] as usize * 32 + ((update[j + 1] as usize & 0b1111_1000) >> 3)]
                        >> (update[j + 1] & 0b0111)
                        & 1
                        == 1
                    {
                        (update[j], update[j + 1]) = (update[j + 1], update[j]);
                        if j != 0 {
                            j -= 1;
                        } else {
                            break;
                        }
                    }
                } else {
                    i += 1;
                }
            } else {
                i += 1;
            }
        }

        if is_valid {
            let num = update[update.len() / 2];
            sum_valid += ((num >> 4) * 10 + (num & 0xf)) as u16;
        } else {
            let num = update[update.len() / 2];
            sum_invalid += ((num >> 4) * 10 + (num & 0xf)) as u16;
        }
    });

    // dbg!(now.elapsed());

    println!("{sum_valid}");
    println!("{sum_invalid}");

    dbg!(now.elapsed());
}
