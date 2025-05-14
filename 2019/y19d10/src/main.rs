use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut best_coords = (0, 0);
    let mut best = 0;

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] != '#' {
                continue;
            }
            let visible = count_visible(&grid, x, y);
            if visible > best {
                best = visible;
                best_coords = (x, y);
            }
        }
    }

    dbg!(best, best_coords);
}

fn count_visible(grid: &[Vec<char>], x: usize, y: usize) -> i32 {
    let mut total = 0;
    for other_y in 0..(grid.len()) {
        'da_loop: for other_x in 0..(grid[other_y].len()) {
            if grid[other_y][other_x] != '#' || (x == other_x && y == other_y) {
                continue;
            }
            let mut x = x as i32;
            let mut y = y as i32;
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

            total += 1;
        }
    }

    total
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 { a.abs() } else { gcd(b, a % b) }
}
