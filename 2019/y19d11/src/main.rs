use std::collections::HashMap;

use aoc_utils::intcode::{Intcode, IntcodeState};

fn main() {
    let input = Intcode::read_to_vec("./input.txt");
    let mut intcode = Intcode::new(input);
    intcode.step_until_done();

    let mut pos = (0, 0);
    let mut tiles: HashMap<(i64, i64), bool> = HashMap::new();
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

    dbg!(tiles.len());
}
