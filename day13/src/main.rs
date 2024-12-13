use std::fs;

use regex::Regex;

fn main() {
    let re = Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)
Button B: X\+(\d+), Y\+(\d+)
Prize: X=(\d+), Y=(\d+)",
    )
    .unwrap();
    let mut total = 0;
    fs::read_to_string("./input.txt")
        .unwrap()
        .split("\n\n")
        .for_each(|input| {
            let thing = re.captures(input).unwrap();
            let (_, [a_x, a_y, b_x, b_y, prize_x, prize_y]) = thing.extract::<6>();
            let a_x: i32 = a_x.parse().unwrap();
            let a_y: i32 = a_y.parse().unwrap();
            let b_x: i32 = b_x.parse().unwrap();
            let b_y: i32 = b_y.parse().unwrap();
            let prize_x: i32 = prize_x.parse().unwrap();
            let prize_y: i32 = prize_y.parse().unwrap();
            let mut lowest_cost: Option<i32> = None;

            todo!();

            if let Some(cost) = lowest_cost {
                total += cost;
            }
        });
}
