use std::{
    fs,
    ops::{Add, Mul},
};

use regex::Regex;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Point {
    x: i128,
    y: i128,
}

impl Point {
    fn new(x: i128, y: i128) -> Point {
        Point { x, y }
    }
}

impl Mul<i128> for Point {
    type Output = Point;
    fn mul(self, rhs: i128) -> Self::Output {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

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

            let a = Point::new(a_x.parse().unwrap(), a_y.parse().unwrap());
            let b = Point::new(b_x.parse().unwrap(), b_y.parse().unwrap());
            let p = Point::new(prize_x.parse().unwrap(), prize_y.parse().unwrap())
                + Point::new(10000000000000, 10000000000000);

            let y = (p.y * a.x - p.x * a.y) / (b.y * a.x - b.x * a.y);
            let x = (p.x - y * b.x) / a.x;

            if a * x + b * y == p {
                total += x * 3 + y;
            }
        });
    dbg!(total);
}
