use std::{collections::HashMap, fs};

fn main() {
    let input: Vec<u8> = fs::read_to_string("./input.txt")
        .unwrap()
        .trim()
        .chars()
        .map(|num| num as u8 & 0x0f)
        .collect();
    let mut filesystem = Vec::new();
    let mut metadata = HashMap::new();
    let mut is_file = true;
    let mut current_id = 0;

    for num in input {
        if is_file {
            metadata.insert(current_id, (filesystem.len(), num as usize));
        }
        for _ in 0..num {
            if is_file {
                filesystem.push(Some(current_id));
            } else {
                filesystem.push(None);
            }
        }
        if is_file {
            current_id += 1;
        }
        is_file = !is_file;
    }

    // let mut left = 0;
    // let mut right = filesystem.len() - 1;
    // while left < right {
    //     if filesystem[left].is_some() {
    //         left += 1;
    //     } else if filesystem[right].is_none() {
    //         right -= 1;
    //     } else {
    //         filesystem[left] = filesystem[right].take();
    //     }
    // }

    for id in (0..current_id).rev() {
        let (pos, len) = metadata.get(&id).unwrap();
        for i in 0..*pos {
            let mut is_valid_spot = true;
            for j in i..(i + len) {
                if filesystem[j].is_some() {
                    is_valid_spot = false;
                    break;
                }
            }
            if is_valid_spot {
                for j in 0..*len {
                    filesystem[i + j] = filesystem[pos + j].take();
                }
            }
        }
    }

    let mut sum = 0;
    for (pos, num) in filesystem
        .into_iter()
        .enumerate()
        .filter_map(|(pos, num)| num.map(|num| (pos, num)))
    {
        sum += pos as i64 * num;
    }
    println!("{sum}");
}
