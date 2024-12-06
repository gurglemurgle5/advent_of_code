use std::{fs, time::Instant};

fn main() {
    let now = Instant::now();

    let mut bitfields = [0u8; 256 * 32];

    let input = fs::read_to_string("./input.txt").unwrap();
    let (rules, updates) = input.split_once("\n\n").unwrap();

    rules.lines().for_each(|line| {
        let line = line.as_bytes();
        let left = (line[0] & 0xf) << 4 | (line[1] & 0xf);
        let right = (line[3] & 0xf) << 4 | (line[4] & 0xf);
        bitfields[right as usize * 32 + ((left as usize & 0b1111_1000) >> 3)] |=
            1 << (left & 0b0111);
    });
    dbg!(now.elapsed());

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
        let mut left = 0;
        while left < (update.len() - 1) {
            if bitfields
                [update[left] as usize * 32 + ((update[left + 1] as usize & 0b1111_1000) >> 3)]
                >> (update[left + 1] & 0b0111)
                & 1
                == 1
            {
                is_valid = false;
                (update[left], update[left + 1]) = (update[left + 1], update[left]);
                if left != 0 {
                    // We need to go back to check if the previous order is invalid
                    // hmm maybe this could be sped up with better backtracking logic
                    left -= 1
                } else {
                    left += 1
                }
            } else {
                left += 1;
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

    dbg!(now.elapsed());

    println!("{sum_valid}");
    println!("{sum_invalid}");

    dbg!(now.elapsed());
}
