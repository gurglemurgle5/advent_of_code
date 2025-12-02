use std::ops::RangeInclusive;

use aoc_utils::Day;

const FACTORS: &[&[u32]] = &[
    &[],        // 0
    &[],        // 1
    &[1],       // 2
    &[1],       // 3
    &[1, 2],    // 4
    &[1],       // 5
    &[1, 2, 3], // 6
    &[1],       // 7
    &[1, 2, 4], // 8
    &[1, 3],    // 9
    &[1, 2, 5], // 10
];

const POWS: &[u64] = &[
    1,
    10,
    100,
    1_000,
    10_000,
    100_000,
    1_000_000,
    10_000_000,
    100_000_000,
    1_000_000_000,
];

pub struct Day2(Box<[RangeInclusive<u64>]>);

impl Day for Day2 {
    fn init(input: &str) -> Box<dyn Day>
    where
        Self: Sized,
    {
        Box::new(Self(
            input
                .trim()
                .split(',')
                .map(|range| {
                    let (left, right) = range.split_once('-').unwrap();
                    let left = left.parse().unwrap();
                    let right = right.parse().unwrap();
                    left..=right
                })
                .collect(),
        ))
    }

    fn part1(&self) -> String {
        let mut invalid = 0;
        for range in &self.0 {
            for num in range.clone() {
                let len = num.ilog10() + 1;
                if len.is_multiple_of(2) {
                    let multiple = POWS[(len / 2) as usize];
                    let test = num % multiple;
                    let other = (num / multiple) % multiple;

                    if other != test {
                        continue;
                    }

                    invalid += num;
                }
            }
        }
        invalid.to_string()
    }

    fn part2(&self) -> String {
        let mut invalid = 0;
        for range in &self.0 {
            for num in range.clone() {
                let len = num.ilog10() + 1;

                'multiples: for &multiple in FACTORS[len as usize] {
                    let chunk = POWS[multiple as usize];
                    let test = num % chunk;

                    for i in 1..(len / multiple) {
                        let other = (num / POWS[multiple as usize * i as usize]) % chunk;
                        if other != test {
                            continue 'multiples;
                        }
                    }

                    invalid += num;
                    break;
                }
            }
        }
        invalid.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(Day2::init("11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124").part1(), "1227775554");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day2::init("11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124").part2(), "4174379265");
    }
}
