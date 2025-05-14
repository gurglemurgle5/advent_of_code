use std::collections::HashMap;

use aoc_utils::intcode::{Intcode, IntcodeState};

fn main() {
    let input = Intcode::read_to_vec("./input.txt");
    let mut intcode = Intcode::new(input);
    intcode.step_until_done();

    let mut pos = (0, 0);
    let mut tiles: HashMap<(i64, i64), bool> = HashMap::new();
    tiles.insert((0, 0), true);
    let mut dir = (0, 1);

    while intcode.state() != IntcodeState::Halted {
        intcode.input(*tiles.get(&pos).unwrap_or(&false) as i64);
        intcode.step_until_done();
        tiles.insert(pos, intcode.output() == 1);
        intcode.step_until_done();
        if intcode.output() == 0 {
            // turn left
            dir = match dir {
                (0, 1) => (-1, 0),
                (1, 0) => (0, 1),
                (0, -1) => (1, 0),
                (-1, 0) => (0, -1),
                _ => panic!("bad dir"),
            }
        } else {
            // turn right
            dir = match dir {
                (0, 1) => (1, 0),
                (1, 0) => (0, -1),
                (0, -1) => (-1, 0),
                (-1, 0) => (0, 1),
                _ => panic!("bad dir"),
            }
        }
        intcode.step_until_done();

        pos.0 += dir.0;
        pos.1 += dir.1;
    }

    let mut furthest_left = 0;
    let mut furthest_up = 0;
    let mut furthest_right = 0;
    let mut furthest_down = 0;

    for (&(x, y), &color) in &tiles {
        if color {
            if x < furthest_left {
                furthest_left = x;
            }
            if x > furthest_right {
                furthest_right = x;
            }
            if y < furthest_down {
                furthest_down = y;
            }
            if y > furthest_up {
                furthest_up = y;
            }
        }
    }

    dbg!(furthest_up, furthest_down, furthest_left, furthest_right);

    for y in (furthest_down..=furthest_up).rev() {
        for x in furthest_left..=furthest_right {
            if *tiles.get(&(x, y)).unwrap_or(&false) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}
