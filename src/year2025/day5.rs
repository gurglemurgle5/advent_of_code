use std::ops::RangeInclusive;

use aoc_utils::Day;

pub struct Day5(String);

impl Day for Day5 {
    fn init(input: &str) -> Box<dyn Day>
    where
        Self: Sized,
    {
        Box::new(Self(input.into()))
    }

    fn part1(&self) -> String {
        let (ranges, ids) = self.0.split_once("\n\n").unwrap();
        let ranges: Vec<RangeInclusive<u64>> = ranges
            .lines()
            .map(|line| {
                let (start, end) = line.split_once('-').unwrap();
                (start.parse().unwrap())..=(end.parse().unwrap())
            })
            .collect();
        let ids: Vec<u64> = ids.lines().map(|line| line.parse().unwrap()).collect();
        let safe = ids
            .iter()
            .filter(|id| {
                ranges
                    .iter()
                    .fold(false, |safe, range| range.contains(id) || safe)
            })
            .count();
        safe.to_string()
    }

    fn part2(&self) -> String {
        let (ranges, _) = self.0.split_once("\n\n").unwrap();
        let mut ranges: Vec<RangeInclusive<u64>> = ranges
            .lines()
            .map(|line| {
                let (start, end) = line.split_once('-').unwrap();
                (start.parse().unwrap())..=(end.parse().unwrap())
            })
            .collect();

        'reduce: loop {
            for first in 0..(ranges.len() - 1) {
                for second in (first + 1)..ranges.len() {
                    let first_range = &ranges[first].clone();
                    let second_range = &ranges[second].clone();

                    if first_range.contains(second_range.start())
                        || first_range.contains(second_range.end())
                        || second_range.contains(first_range.start())
                        || second_range.contains(first_range.end())
                    {
                        let new_range = (*first_range.start().min(second_range.start()))
                            ..=(*first_range.end().max(second_range.end()));
                        ranges.remove(second);
                        ranges.remove(first);
                        ranges.push(new_range);
                        continue 'reduce;
                    }
                }
            }
            break;
        }
        ranges
            .into_iter()
            .map(|range| range.end() - range.start() + 1)
            .sum::<u64>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            Day5::init(
                "3-5
10-14
16-20
12-18

1
5
8
11
17
32
"
            )
            .part1(),
            "3"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            Day5::init(
                "3-5
10-14
16-20
12-18

1
5
8
11
17
32
"
            )
            .part2(),
            "14"
        );
    }
}
