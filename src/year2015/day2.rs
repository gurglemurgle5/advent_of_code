use std::str::FromStr;

use aoc_utils::Day;

pub struct Day2(String);

impl Day for Day2 {
    fn init(input: &str) -> Box<dyn Day>
    where
        Self: Sized,
    {
        Box::new(Self(input.into()))
    }

    fn part1(&self) -> String {
        let presents: Vec<Present> = self.0.lines().map(|line| line.parse().unwrap()).collect();

        let total: u32 = presents
            .iter()
            .map(|present| present.surface_area_plus_slack())
            .sum();

        total.to_string()
    }

    fn part2(&self) -> String {
        let presents: Vec<Present> = self.0.lines().map(|line| line.parse().unwrap()).collect();

        let total: u32 = presents.iter().map(|present| present.ribbon()).sum();

        total.to_string()
    }
}

struct Present {
    l: u32,
    w: u32,
    h: u32,
}

impl Present {
    fn surface_area_plus_slack(&self) -> u32 {
        let lw = self.l * self.w;
        let wh = self.w * self.h;
        let hl = self.h * self.l;
        lw * 2 + wh * 2 + hl * 2 + lw.min(wh).min(hl)
    }

    fn volume(&self) -> u32 {
        self.l * self.w * self.h
    }

    fn ribbon(&self) -> u32 {
        let lw = (self.l + self.w) * 2;
        let wh = (self.w + self.h) * 2;
        let hl = (self.h + self.l) * 2;
        lw.min(wh).min(hl) + self.volume()
    }
}

impl FromStr for Present {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dims: Vec<u32> = s.split('x').map(|num| num.parse().unwrap()).collect();
        Ok(Present {
            l: dims[0],
            w: dims[1],
            h: dims[2],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2x3x4() {
        assert_eq!(Day2::init("2x3x4").part1(), "58");
        assert_eq!(Day2::init("2x3x4").part2(), "34");
    }

    #[test]
    fn test_1x1x10() {
        assert_eq!(Day2::init("1x1x10").part1(), "43");
        assert_eq!(Day2::init("1x1x10").part2(), "14");
    }
}
