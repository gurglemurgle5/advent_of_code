use std::{fs, time::Instant};

fn main() {
    let now = Instant::now();

    let mut bitfields = [0u128; 100];

    let input = fs::read_to_string("./input.txt").unwrap();
    let (rules, updates) = input.split_once("\n\n").unwrap();

    rules.lines().for_each(|line| {
        let line = line.as_bytes();
        let left = (line[0] & 0b1111) * 10 + (line[1] & 0b1111);
        let right = (line[3] & 0b1111) * 10 + (line[4] & 0b1111);
        bitfields[right as usize] |= 1 << left;
    });
    dbg!(now.elapsed());

    let mut sum_valid = 0;
    let mut sum_invalid = 0;
    updates.lines().for_each(|line| {
        let mut update: Vec<u8> = line
            .split(',')
            .map(|num| {
                let num = num.as_bytes();
                (num[0] & 0b1111) * 10 + (num[1] & 0b1111)
            })
            .collect();
        let mut is_valid = true;

        // so it turns out incorrectly ordered pages are always next to each other, allowing for
        // more optimization
        let mut left = 0;
        while left < (update.len() - 1) {
            if bitfields[update[left] as usize] >> (update[left + 1]) & 1 == 1 {
                is_valid = false;
                (update[left], update[left + 1]) = (update[left + 1], update[left]);
                if left != 0 {
                    // We need to go back to check if the previous order is invalid
                    left -= 1
                } else {
                    left += 1
                }
            } else {
                left += 1;
            }
        }

        if is_valid {
            sum_valid += update[update.len() / 2] as u16;
        } else {
            sum_invalid += update[update.len() / 2] as u16;
        }
    });

    dbg!(now.elapsed());

    println!("{sum_valid}");
    println!("{sum_invalid}");

    dbg!(now.elapsed());
}
