use aoc_utils::intcode::Intcode;
use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let mut input: Vec<i64> = input.split(',').map(|num| num.parse().unwrap()).collect();
    input[1] = 12;
    input[2] = 2;
    let mut intcode = Intcode::new(input);

    intcode.step_until_done();

    dbg!(intcode.memory()[0]);
}
