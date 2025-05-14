use std::{f64, fs};

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut best_coords = (0, 0);
    let mut best = 0;

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] != '#' {
                continue;
            }
            let visible = count_visible(&grid, x, y);
            if visible.len() > best {
                best = visible.len();
                best_coords = (x, y);
            }
        }
    }

    dbg!(best_coords, best);

    let mut destroyed: usize = 0;
    let mut list;

    loop {
        list = count_visible(&grid, best_coords.0, best_coords.1);
        if list.len() + destroyed < 200 {
            destroyed += list.len();
            for &(x, y) in &list {
                grid[y][x] = '.';
            }
        } else {
            break;
        }
    }

    let mut list: Vec<(i32, i32)> = list
        .into_iter()
        .map(|(x, y)| {
            (
                x as i32 - best_coords.0 as i32,
                y as i32 - best_coords.1 as i32,
            )
        })
        .collect();

    list.sort_by(|left, right| {
        let mut left_angle = f64::atan2(left.1 as f64, left.0 as f64);
        left_angle += f64::consts::FRAC_PI_2;
        while left_angle < 0.0 {
            left_angle += f64::consts::TAU;
        }
        left_angle %= f64::consts::TAU;
        let mut right_angle = f64::atan2(right.1 as f64, right.0 as f64);
        right_angle += f64::consts::FRAC_PI_2;
        while right_angle < 0.0 {
            right_angle += f64::consts::TAU;
        }
        left_angle %= f64::consts::TAU;
        left_angle.total_cmp(&right_angle)
    });

    let mut n200 = list[199 - destroyed];
    n200.0 += best_coords.0 as i32;
    n200.1 += best_coords.1 as i32;

    dbg!(n200);
}

fn count_visible(grid: &[Vec<char>], orig_x: usize, orig_y: usize) -> Vec<(usize, usize)> {
    let mut total = Vec::new();
    for other_y in 0..(grid.len()) {
        'da_loop: for other_x in 0..(grid[other_y].len()) {
            if grid[other_y][other_x] != '#' || (orig_x == other_x && orig_y == other_y) {
                continue;
            }
            let mut x = orig_x as i32;
            let mut y = orig_y as i32;
            let dx = other_x as i32 - x;
            let dy = other_y as i32 - y;
            let gcd = gcd(dx, dy);
            let dx = dx / gcd;
            let dy = dy / gcd;

            x += dx;
            y += dy;

            while !(x == other_x as i32 && y == other_y as i32) {
                if grid[y as usize][x as usize] == '#' {
                    continue 'da_loop;
                }
                x += dx;
                y += dy;
            }

            total.push((other_x, other_y));
        }
    }

    total
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 { a.abs() } else { gcd(b, a % b) }
}
