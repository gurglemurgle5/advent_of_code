use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let (map, instructions) = input.split_once("\n\n").unwrap();
    let (mut bot_row, mut bot_col) = (0, 0);
    let input: Vec<Vec<(char, char)>> = map
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| match char {
                    '.' => ('.', '.'),
                    '#' => ('#', '#'),
                    '@' => ('@', '.'),
                    'O' => ('[', ']'),
                    _ => panic!("unknown char"),
                })
                .collect()
        })
        .collect();
    let mut map = Vec::new();
    for line in input {
        let mut vec = Vec::new();
        for (a, b) in line {
            vec.push(a);
            vec.push(b);
        }
        map.push(vec);
    }
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
            // for row in map.iter() {
            //     for char in row {
            //         print!("{char}");
            //     }
            //     println!();
            // }
            // println!();
            let (mut row, mut col) = (bot_row, bot_col);
            let mut stack_length = 0;
            match dir {
                '^' => match map[row - 1][col] {
                    '#' => (),
                    '[' | ']' => {
                        let mut new = map.clone();
                        if try_push_box_up(&mut new, row - 1, col).is_ok() {
                            map = new;
                            bot_row -= 1;
                        }
                    }
                    '.' => {
                        bot_row -= 1;
                        map[bot_row][bot_col] = '.';
                    }
                    _ => panic!("bad map char!"),
                },
                '>' => loop {
                    col += 1;
                    match map[row][col] {
                        '#' => break,
                        '[' | ']' => stack_length += 1,
                        '.' => {
                            bot_col += 1;
                            map[bot_row][bot_col] = '.'; // clear any boxes that may be there
                            for i in 0..stack_length {
                                map[bot_row][bot_col + 1 + i] = if i % 2 == 0 { '[' } else { ']' };
                            }
                            break;
                        }
                        _ => panic!("bad map char!"),
                    }
                },
                'v' => {
                    match map[row + 1][col] {
                        '#' => (),
                        '[' | ']' => {
                            let mut new = map.clone();
                            if try_push_box_down(&mut new, row + 1, col).is_ok() {
                                map = new;
                                bot_row += 1;
                            }
                        }
                        '.' => {
                            bot_row += 1;
                            map[bot_row][bot_col] = '.'; // clear any boxes that may be there
                        }
                        _ => panic!("bad map char!"),
                    }
                }
                '<' => loop {
                    col -= 1;
                    match map[row][col] {
                        '#' => break,
                        '[' | ']' => stack_length += 1,
                        '.' => {
                            bot_col -= 1;
                            map[bot_row][bot_col] = '.'; // clear any boxes that may be there
                            for i in 0..stack_length {
                                map[bot_row][bot_col - 1 - i] = if i % 2 == 0 { ']' } else { '[' };
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
        println!();
    }
    println!();

    let mut total = 0;

    for (row, row_data) in map.iter().enumerate() {
        for (col, char) in row_data.iter().enumerate() {
            if *char == '[' {
                total += 100 * row + col;
            }
        }
    }

    dbg!(total);
}

fn try_push_box_up(map: &mut Vec<Vec<char>>, row: usize, mut col: usize) -> Result<(), ()> {
    if map[row][col] == ']' {
        col -= 1;
    }
    match map[row - 1][col] {
        '.' => (),
        '#' => return Err(()),
        '[' | ']' => try_push_box_up(map, row - 1, col)?,
        _ => panic!("bad map char!"),
    }
    match map[row - 1][col + 1] {
        '.' => (),
        '#' => return Err(()),
        '[' | ']' => try_push_box_up(map, row - 1, col + 1)?,
        _ => panic!("bad map char!"),
    }
    map[row][col] = '.';
    map[row][col + 1] = '.';
    map[row - 1][col] = '[';
    map[row - 1][col + 1] = ']';
    Ok(())
}

fn try_push_box_down(map: &mut Vec<Vec<char>>, row: usize, mut col: usize) -> Result<(), ()> {
    if map[row][col] == ']' {
        col -= 1;
    }
    match map[row + 1][col] {
        '.' => (),
        '#' => return Err(()),
        '[' | ']' => try_push_box_down(map, row + 1, col)?,
        _ => panic!("bad map char!"),
    }
    match map[row + 1][col + 1] {
        '.' => (),
        '#' => return Err(()),
        '[' | ']' => try_push_box_down(map, row + 1, col + 1)?,
        _ => panic!("bad map char!"),
    }
    map[row][col] = '.';
    map[row][col + 1] = '.';
    map[row + 1][col] = '[';
    map[row + 1][col + 1] = ']';
    Ok(())
}
