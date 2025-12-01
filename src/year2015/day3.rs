use std::collections::HashSet;

use aoc_utils::Day;

pub struct Day3(String);

impl Day for Day3 {
    fn init(input: String) -> Box<dyn Day>
    where
        Self: Sized,
    {
        Box::new(Self(input))
    }

    fn part1(&self) -> String {
        let mut visited = HashSet::new();

        let mut x = 0;
        let mut y = 0;

        visited.insert((x, y));

        for char in self.0.chars() {
            match char {
                '^' => y += 1,
                'v' => y -= 1,
                '>' => x += 1,
                '<' => x -= 1,
                _ => panic!(),
            }
            visited.insert((x, y));
        }

        visited.len().to_string()
    }

    fn part2(&self) -> String {
        let mut visited = HashSet::new();

        let mut x1 = 0;
        let mut y1 = 0;
        let mut x2 = 0;
        let mut y2 = 0;

        visited.insert((0, 0));

        for (i, char) in self.0.chars().enumerate() {
            if i % 2 == 0 {
                match char {
                    '^' => y1 += 1,
                    'v' => y1 -= 1,
                    '>' => x1 += 1,
                    '<' => x1 -= 1,
                    _ => panic!(),
                }
                visited.insert((x1, y1));
            } else {
                match char {
                    '^' => y2 += 1,
                    'v' => y2 -= 1,
                    '>' => x2 += 1,
                    '<' => x2 -= 1,
                    _ => panic!(),
                }
                visited.insert((x2, y2));
            }
        }

        visited.len().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(Day3::init(">".into()).part1(), "2");
        assert_eq!(Day3::init("^>v<".into()).part1(), "4");
        assert_eq!(Day3::init("^v^v^v^v^v".into()).part1(), "2");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day3::init("^v".into()).part2(), "3");
        assert_eq!(Day3::init("^>v<".into()).part2(), "3");
        assert_eq!(Day3::init("^v^v^v^v^v".into()).part2(), "11");
    }
}
