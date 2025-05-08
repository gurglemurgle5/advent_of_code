use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let mut locks: Vec<[u8; 5]> = Vec::new();
    let mut keys: Vec<[u8; 5]> = Vec::new();

    input.split("\n\n").for_each(|object| {
        if object.starts_with('#') {
            let mut lock = [0u8; 5];
            for (len, line) in object.lines().skip(1).take(5).enumerate() {
                for (index, char) in line.chars().enumerate() {
                    if char == '#' {
                        lock[index] = len as u8 + 1;
                    }
                }
            }
            locks.push(lock);
        } else {
            // key
            let mut key = [0u8; 5];
            for (len, line) in object.lines().rev().skip(1).take(5).enumerate() {
                for (index, char) in line.chars().enumerate() {
                    if char == '#' {
                        key[index] = len as u8 + 1;
                    }
                }
            }
            keys.push(key);
        }
    });
    let mut total = 0;
    for lock in locks {
        for key in &keys {
            let mut is_valid = true;
            for i in 0..5 {
                if lock[i] + key[i] > 5 {
                    is_valid = false;
                }
            }
            if is_valid {
                total += 1;
            }
        }
    }
    dbg!(total);
}
