use aoc_utils::intcode::Intcode;
use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let input: Vec<i64> = input.split(',').map(|num| num.parse().unwrap()).collect();

    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut input = input.clone();
            input[1] = noun;
            input[2] = verb;
            let mut intcode = Intcode::new(input);

            intcode.step_until_done();
            if intcode.memory()[0] == 19690720 {
                dbg!(format!("{noun}{verb}"));
            }
        }
    }
}
