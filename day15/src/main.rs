use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let (map, instructions) = input.split_once("\n\n").unwrap();
    let (mut bot_row, mut bot_col) = (0, 0);
    let mut map: Vec<Vec<char>> = map.lines().map(|line| line.chars().collect()).collect();
    for (row, row_data) in map.iter_mut().enumerate() {
        for (col, char) in row_data.iter_mut().enumerate() {
            if *char == '@' {
                *char = '.';
                (bot_row, bot_col) = (row, col);
            }
        }
    }

    instructions
        .chars()
        .filter(|char| *char != '\n')
        .for_each(|dir| {
            for row in map.iter() {
                for char in row {
                    print!("{char}");
                }
                println!()
            }
            let (mut row, mut col) = (bot_row, bot_col);
            let mut stack_length = 0;
            match dir {
                '^' => loop {
                    row -= 1;
                    match map[row][col] {
                        '#' => break,
                        'O' => stack_length += 1,
                        '.' => {
                            bot_row -= 1;
                            map[bot_row][bot_col] = '.'; // clear any boxes that may be there
                            for i in 0..stack_length {
                                map[bot_row - 1 - i][bot_col] = 'O'; // push the boxes
                            }
                            break;
                        }
                        _ => panic!("bad map char!"),
                    }
                },
                '>' => loop {
                    col += 1;
                    match map[row][col] {
                        '#' => break,
                        'O' => stack_length += 1,
                        '.' => {
                            bot_col += 1;
                            map[bot_row][bot_col] = '.'; // clear any boxes that may be there
                            for i in 0..stack_length {
                                map[bot_row][bot_col + 1 + i] = 'O'; // push the boxes
                            }
                            break;
                        }
                        _ => panic!("bad map char!"),
                    }
                },
                'v' => loop {
                    row += 1;
                    match map[row][col] {
                        '#' => break,
                        'O' => stack_length += 1,
                        '.' => {
                            bot_row += 1;
                            map[bot_row][bot_col] = '.'; // clear any boxes that may be there
                            for i in 0..stack_length {
                                map[bot_row + 1 + i][bot_col] = 'O'; // push the boxes
                            }
                            break;
                        }
                        _ => panic!("bad map char!"),
                    }
                },
                '<' => loop {
                    col -= 1;
                    match map[row][col] {
                        '#' => break,
                        'O' => stack_length += 1,
                        '.' => {
                            bot_col -= 1;
                            map[bot_row][bot_col] = '.'; // clear any boxes that may be there
                            for i in 0..stack_length {
                                map[bot_row][bot_col - 1 - i] = 'O'; // push the boxes
                            }
                            break;
                        }
                        _ => panic!("bad map char!"),
                    }
                },
                a => panic!("bad instruction char {a}!"),
            }
        });
    for row in map.iter() {
        for char in row {
            print!("{char}");
        }
        println!()
    }

    let mut total = 0;

    for (row, row_data) in map.iter().enumerate() {
        for (col, char) in row_data.iter().enumerate() {
            if *char == 'O' {
                total += 100 * row + col;
            }
        }
    }

    dbg!(total);
}
