use std::{fs, i32, thread::sleep, time::Duration};

fn main() {
    const WIDTH: i32 = 101;
    const HEIGHT: i32 = 103;
    const STEPS: i32 = 100;
    let input = fs::read_to_string("./input.txt").unwrap();
    let mut bots: Vec<(i32, i32, i32, i32)> = input
        .lines()
        .map(|line| {
            let (pos, vel) = line.split_once(" ").unwrap();
            let pos = &pos[2..];
            let (pos_x, pos_y) = pos.split_once(",").unwrap();
            let pos_x = pos_x.parse().unwrap();
            let pos_y = pos_y.parse().unwrap();
            let vel = &vel[2..];
            let (vel_x, vel_y) = vel.split_once(",").unwrap();
            let vel_x = vel_x.parse().unwrap();
            let vel_y = vel_y.parse().unwrap();
            (pos_x, pos_y, vel_x, vel_y)
        })
        .collect();
    let mut t = 0;
    loop {
        let mut grid = [[0i32; WIDTH as usize]; HEIGHT as usize];
        for bot in &bots {
            grid[bot.1 as usize][bot.0 as usize] += 1;
        }
        println!("t = {t}");
        for line in grid {
            for num in line {
                if num == 0 {
                    print!(" ");
                } else {
                    print!("{num}");
                }
            }
            println!();
        }

        if t % 1000 == 0 {
            eprintln!("t = {t}");
        }

        // sleep(Duration::from_secs(1));
        t += 1;
        for bot in bots.iter_mut() {
            bot.0 += bot.2;
            bot.0 %= WIDTH;
            if bot.0 < 0 {
                bot.0 += WIDTH;
            }
            bot.1 += bot.3;
            bot.1 %= HEIGHT;
            if bot.1 < 0 {
                bot.1 += HEIGHT;
            }
        }
    }

    for bot in bots.iter_mut() {
        bot.0 += bot.2 * STEPS;
        bot.0 %= WIDTH;
        if bot.0 < 0 {
            bot.0 += WIDTH;
        }
        bot.1 += bot.3 * STEPS;
        bot.1 %= HEIGHT;
        if bot.1 < 0 {
            bot.1 += HEIGHT;
        }
    }

    let mut top_left = 0;
    let mut top_right = 0;
    let mut bottom_left = 0;
    let mut bottom_right = 0;

    for bot in bots {
        if bot.0 < WIDTH / 2 {
            if bot.1 < HEIGHT / 2 {
                top_left += 1;
            } else if bot.1 > HEIGHT / 2 {
                bottom_left += 1;
            }
        } else if bot.0 > WIDTH / 2 {
            if bot.1 < HEIGHT / 2 {
                top_right += 1;
            } else if bot.1 > HEIGHT / 2 {
                bottom_right += 1;
            }
        }
    }
    dbg!(top_left, top_right, bottom_left, bottom_right);
    dbg!(top_left * top_right * bottom_left * bottom_right);
}
