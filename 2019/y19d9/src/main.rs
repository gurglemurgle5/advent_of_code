use aoc_utils::intcode::{Intcode, IntcodeState};

fn main() {
    let input = Intcode::read_to_vec("./input.txt");
    let mut intcode = Intcode::new(input);
    intcode.step_until_done();
    intcode.input(1);
    intcode.step_until_done();
    while intcode.state() != IntcodeState::Halted {
        dbg!(intcode.output());
        intcode.step_until_done();
    }
}
