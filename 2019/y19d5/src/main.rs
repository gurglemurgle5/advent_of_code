use aoc_utils::intcode::{Intcode, IntcodeState};

fn main() {
    let input = Intcode::read_to_vec("./input.txt");
    let mut intcode = Intcode::new(input);

    intcode.step_until_done();
    dbg!(intcode.state());
    intcode.input(5);
    while intcode.step_until_done() != IntcodeState::Halted {
        println!("{}", intcode.output());
    }
}
