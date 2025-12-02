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
        self.0
            .lines()
            .filter(|line| {
                let vowels = line.chars().filter(|char| "aeiou".contains(*char)).count();
                if vowels < 3 {
                    return false;
                }
                if line.contains("ab")
                    || line.contains("cd")
                    || line.contains("pq")
                    || line.contains("xy")
                {
                    return false;
                }
                let mut chars = line.chars();
                let mut prev = chars.next().unwrap();
                for char in chars {
                    if prev == char {
                        return true;
                    }
                    prev = char;
                }
                false
            })
            .count()
            .to_string()
    }

    // fn part2(&self) -> String {
    //     self.0
    //         .lines()
    //         .filter(|line| {
    //             todo!();
    //         })
    //         .count()
    //         .to_string()
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(Day5::init("ugknbfddgicrmopn").part1(), "1");
        assert_eq!(Day5::init("aaa").part1(), "1");
        assert_eq!(Day5::init("jchzalrnumimnmhp").part1(), "0");
        assert_eq!(Day5::init("haegwjzuvuyypxyu").part1(), "0");
        assert_eq!(Day5::init("dvszwmarrgswjxmb").part1(), "0");
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(Day5::init("qjhvhtzxzqqjkmpb".to_string()).part2(), "1");
    //     assert_eq!(Day5::init("xxyxx".to_string()).part2(), "1");
    //     assert_eq!(Day5::init("uurcxstgmygtbstg".to_string()).part2(), "0");
    //     assert_eq!(Day5::init("ieodomkazucvgmuy".to_string()).part2(), "0");
    // }
}
