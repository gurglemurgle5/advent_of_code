use std::fs;

fn main() {
    let input: Vec<Vec<char>> = fs::read_to_string("./input.txt")
        .unwrap()
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    const WORD: &[char] = &[/*'X',*/ 'M', 'A', 'S'];
    const LEN: usize = WORD.len();

    let mut count = 0;

    // for y in 0..input.len() {
    //     for x in 0..input[y].len() {
    //         // left
    //         if x >= LEN - 1 {
    //             let mut is_word = true;
    //             for i in 0..LEN {
    //                 if input[y][x - i] != WORD[i] {
    //                     is_word = false;
    //                 }
    //             }
    //             count += is_word as i32; // less lines than if statement
    //         }
    //         // up
    //         if y >= LEN - 1 {
    //             let mut is_word = true;
    //             for i in 0..LEN {
    //                 if input[y - i][x] != WORD[i] {
    //                     is_word = false;
    //                 }
    //             }
    //             count += is_word as i32; // less lines than if statement
    //         }
    //         // right
    //         if x <= input[y].len() - LEN {
    //             let mut is_word = true;
    //             for i in 0..LEN {
    //                 if input[y][x + i] != WORD[i] {
    //                     is_word = false;
    //                 }
    //             }
    //             count += is_word as i32; // less lines than if statement
    //         }
    //         // down
    //         if y <= input.len() - LEN {
    //             let mut is_word = true;
    //             for i in 0..LEN {
    //                 if input[y + i][x] != WORD[i] {
    //                     is_word = false;
    //                 }
    //             }
    //             count += is_word as i32; // less lines than if statement
    //         }
    //         // up-left
    //         if y >= LEN - 1 && x >= LEN - 1 {
    //             let mut is_word = true;
    //             for i in 0..LEN {
    //                 if input[y - i][x - i] != WORD[i] {
    //                     is_word = false;
    //                 }
    //             }
    //             count += is_word as i32; // less lines than if statement
    //         }
    //         // up-right
    //         if y >= LEN - 1 && x <= input[y].len() - LEN {
    //             let mut is_word = true;
    //             for i in 0..LEN {
    //                 if input[y - i][x + i] != WORD[i] {
    //                     is_word = false;
    //                 }
    //             }
    //             count += is_word as i32; // less lines than if statement
    //         }
    //         // down-left
    //         if y <= input.len() - LEN && x >= LEN - 1 {
    //             let mut is_word = true;
    //             for i in 0..LEN {
    //                 if input[y + i][x - i] != WORD[i] {
    //                     is_word = false;
    //                 }
    //             }
    //             count += is_word as i32; // less lines than if statement
    //         }
    //         // down-right
    //         if y <= input.len() - LEN && x <= input[y].len() - LEN {
    //             let mut is_word = true;
    //             for i in 0..LEN {
    //                 if input[y + i][x + i] != WORD[i] {
    //                     is_word = false;
    //                 }
    //             }
    //             count += is_word as i32; // less lines than if statement
    //         }
    //     }
    // }
    // println!("{count}");

    for y in 1..(input.len() - 1) {
        for x in 1..(input[y].len() - 1) {
            let mut up_left = true;
            let mut up_right = true;
            let mut down_left = true;
            let mut down_right = true;
            // up-left

            for i in 0..LEN {
                if input[y + 1 - i][x + 1 - i] != WORD[i] {
                    up_left = false;
                }
            }

            // up-right

            for i in 0..LEN {
                if input[y + 1 - i][x + i - 1] != WORD[i] {
                    up_right = false;
                }
            }

            // down-left

            for i in 0..LEN {
                if input[y + i - 1][x + 1 - i] != WORD[i] {
                    down_left = false;
                }
            }

            // down-right

            for i in 0..LEN {
                if input[y + i - 1][x + i - 1] != WORD[i] {
                    down_right = false;
                }
            }

            if (up_left || down_right) && (up_right || down_left) {
                count += 1;
            }
        }
    }

    println!("{count}");
}
