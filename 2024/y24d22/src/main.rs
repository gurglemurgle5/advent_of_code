use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let input: Vec<i32> = input
        .trim()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    let mut max_bananas = 0;
    for a in -9..=9 {
        for b in -9..=9 {
            dbg!((a, b));
            for c in -9..=9 {
                for d in -9..=9 {
                    let test_changes = [a, b, c, d];
                    if !verify_changes(&test_changes) {
                        continue;
                    }
                    let mut bananas = 0;
                    'sellers: for num in &input {
                        let mut num = *num;
                        let mut changes = [-10, -10, -10, -10]; // using -10 as placeholder value
                        for _ in 0..2000 {
                            let old = num % 10;

                            num ^= (num << 6) & 0xffffff;
                            num ^= num >> 5;
                            num ^= (num << 11) & 0xffffff;

                            changes.rotate_left(1);
                            changes[3] = (num % 10) - old;
                            if changes == test_changes {
                                bananas += num % 10;
                                continue 'sellers;
                            }
                        }
                    }
                    if bananas > max_bananas {
                        max_bananas = bananas;
                    }
                }
            }
        }
    }

    dbg!(max_bananas);
}

fn verify_changes(changes: &[i32; 4]) -> bool {
    let mut lower_range = 0;
    let mut upper_range = 0;
    let mut current = 0;
    for &num in changes {
        current += num;
        if current < lower_range {
            lower_range = current;
        }
        if current > upper_range {
            upper_range = current;
        }
    }
    let range = upper_range - lower_range;

    // if range == 9 && current.is_negative() {
    //     // 0 bananas ):
    //     return false;
    // }
    // range <= 9

    // arbitrary threashold to keep bad changes out
    // might not give a correct result tho
    current > 0 && range <= 9
}
