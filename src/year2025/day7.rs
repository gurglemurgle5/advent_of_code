use std::collections::{HashMap, HashSet};

use aoc_utils::{Day, grid::Grid};

pub struct Day7(String);

impl Day for Day7 {
    fn init(input: &str) -> Box<dyn Day>
    where
        Self: Sized,
    {
        Box::new(Self(input.into()))
    }

    fn part1(&self) -> String {
        let grid: Grid<char> = self.0.parse().unwrap();
        let mut beams: HashSet<usize> = HashSet::new();
        let mut splits = 0;
        beams.insert(
            grid.row(0)
                .iter()
                .enumerate()
                .find(|(_, char)| **char == 'S')
                .unwrap()
                .0,
        );
        for row in grid.rows() {
            let mut new_beams = HashSet::new();
            for beam in beams {
                if row[beam] == '^' {
                    splits += 1;
                    new_beams.insert(beam - 1);
                    new_beams.insert(beam + 1);
                } else {
                    new_beams.insert(beam);
                }
            }
            beams = new_beams;
        }
        splits.to_string()
    }

    fn part2(&self) -> String {
        let grid: Grid<char> = self.0.parse().unwrap();
        let beam = grid
            .row(0)
            .iter()
            .enumerate()
            .find(|(_, char)| **char == 'S')
            .unwrap()
            .0;
        let mut results = HashMap::new();
        get_timelines(beam, &grid, 0, &mut results).to_string()
    }
}

fn get_timelines(
    beam: usize,
    grid: &Grid<char>,
    row: usize,
    results: &mut HashMap<(usize, usize), i64>,
) -> i64 {
    let result = results.get(&(row, beam)).copied();
    match result {
        Some(num) => num,
        None => {
            let result = if grid.height() == row {
                1
            } else if grid[(beam, row)] == '^' {
                get_timelines(beam - 1, grid, row + 1, results)
                    + get_timelines(beam + 1, grid, row + 1, results)
            } else {
                get_timelines(beam, grid, row + 1, results)
            };
            results.insert((row, beam), result);
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            Day7::init(
                ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
"
            )
            .part1(),
            "21"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            Day7::init(
                ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
"
            )
            .part2(),
            "40"
        );
    }
}
