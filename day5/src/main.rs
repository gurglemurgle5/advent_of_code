use std::{fs, time::Instant};

fn main() {
    let now = Instant::now();

    let mut bitfields = [0u128; 100];

    let input = fs::read_to_string("./input.txt").unwrap();
    let (rules, updates) = input.split_once("\n\n").unwrap();

    rules.lines().for_each(|line| {
        let line = line.as_bytes();
        let left = (line[0] as u8 & 0b1111) * 10 + (line[1] as u8 & 0b1111);
        let right = (line[3] as u8 & 0b1111) * 10 + (line[4] as u8 & 0b1111);
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
                (num[0] as u8 & 0b1111) * 10 + (num[1] as u8 & 0b1111)
            })
            .collect();
        let mut is_valid = true;

        for i in 0..(update.len() - 1) {
            let mut j = i + 1;
            while j < update.len() {
                if bitfields[update[i] as usize] & (1 << update[j]) == (1 << update[j]) {
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
    });

    dbg!(now.elapsed());

    println!("{sum_valid}");
    println!("{sum_invalid}");
    dbg!(now.elapsed());
}
