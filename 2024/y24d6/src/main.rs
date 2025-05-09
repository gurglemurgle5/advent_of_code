use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut row: i32 = 0;
    let mut col: i32 = 0;
    let mut dir = '\0';

    for (r, line) in grid.iter_mut().enumerate() {
        for (c, char) in line.iter_mut().enumerate() {
            match char {
                '^' => {
                    row = r as i32;
                    col = c as i32;
                    dir = '^';
                    *char = '.';
                }
                _ => (),
            }
        }
    }
    let mut solutions = 0;
    for obj_row in 0..grid.len() {
        println!("doing row {obj_row}");
        'main_loop: for obj_col in 0..grid[obj_row].len() {
            if grid[obj_row][obj_col] == '#' || (obj_row as i32 == row && obj_col as i32 == col) {
                continue 'main_loop;
            }

            grid[obj_row][obj_col] = '#';

            let mut row = row;
            let mut col = col;
            let mut dir = dir;

            let mut steps = Vec::new();

            loop {
                let pos = (row, col, dir);
                if steps.contains(&pos) {
                    solutions += 1;
                    grid[obj_row][obj_col] = '.';
                    continue 'main_loop;
                }
                steps.push(pos);
                match dir {
                    '^' => {
                        if grid[row as usize - 1][col as usize] != '#' {
                            row -= 1;
                        } else {
                            dir = '>'
                        }
                    }
                    '>' => {
                        if grid[row as usize][col as usize + 1] != '#' {
                            col += 1;
                        } else {
                            dir = 'V'
                        }
                    }
                    'V' => {
                        if grid[row as usize + 1][col as usize] != '#' {
                            row += 1;
                        } else {
                            dir = '<'
                        }
                    }
                    '<' => {
                        if grid[row as usize][col as usize - 1] != '#' {
                            col -= 1;
                        } else {
                            dir = '^'
                        }
                    }
                    _ => panic!(),
                }
                // grid[row as usize][col as usize] = 'X';
                // if row >= grid.len() as i32 || row < 0 || col >= grid[row as usize].len() as i32 || col < 0
                if (dir == '^' && row == 0)
                    || (dir == '>' && col == grid[row as usize].len() as i32 - 1)
                    || (dir == 'V' && row == grid.len() as i32 - 1)
                    || (dir == '<' && col == 0)
                {
                    grid[obj_row][obj_col] = '.';
                    continue 'main_loop;
                }
            }
        }
    }
    dbg!(solutions);
    // grid[row as usize][col as usize] = 'X';

    // let mut count = 0;
    // for row in grid {
    //     for char in row {
    //         if char == 'X' {
    //             count += 1;
    //         }
    //     }
    // }
    // dbg!(count);
}
