use std::ops::RangeInclusive;

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
        let ranges: Box<[RangeInclusive<u64>]> = self
            .0
            .trim()
            .split(',')
            .map(|range| {
                let (left, right) = range.split_once('-').unwrap();
                let left = left.parse().unwrap();
                let right = right.parse().unwrap();
                left..=right
            })
            .collect();

        let mut invalid: u64 = 0;
        for range in ranges {
            for num in range {
                let num = num.to_string();
                let len = num.len();
                if len.is_multiple_of(2) && num[0..(len / 2)] == num[(len / 2)..len] {
                    invalid += num.parse::<u64>().unwrap();
                }
            }
        }
        invalid.to_string()
    }

    fn part2(&self) -> String {
        let ranges: Box<[RangeInclusive<u64>]> = self
            .0
            .trim()
            .split(',')
            .map(|range| {
                let (left, right) = range.split_once('-').unwrap();
                let left = left.parse().unwrap();
                let right = right.parse().unwrap();
                left..=right
            })
            .collect();

        let mut invalid: u64 = 0;
        for range in ranges {
            for num in range {
                let num = num.to_string();
                let len = num.len();

                let mut is_invalid = None;
                for multiple in 1..len {
                    if len.is_multiple_of(multiple) {
                        is_invalid = Some(true);
                        let test = &num[0..multiple];
                        for i in 1..(len / multiple) {
                            if &num[(i * multiple)..(multiple + (i * multiple))] != test {
                                is_invalid = Some(false);
                            }
                        }
                    }
                    if is_invalid.is_some_and(|bool| bool) {
                        invalid += num.parse::<u64>().unwrap();
                        break;
                    }
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
