use aoc_utils::Day;

pub struct Day1(String);

impl Day for Day1 {
    fn init(input: String) -> Box<dyn Day>
    where
        Self: Sized,
    {
        Box::new(Self(input))
    }

    fn part1(&self) -> String {
        let mut total = 0;
        for char in self.0.chars() {
            if char == '(' {
                total += 1;
            } else {
                total -= 1;
            }
        }
        total.to_string()
    }

    fn part2(&self) -> String {
        let mut total = 0;
        for (i, char) in self.0.chars().enumerate() {
            if char == '(' {
                total += 1;
            } else {
                total -= 1;
            }
            if total < 0 {
                return (i + 1).to_string();
            }
        }
        panic!("input did not go into basement");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero() {
        assert_eq!(Day1::init("(())".into()).part1(), "0");
        assert_eq!(Day1::init("()()".into()).part1(), "0");
    }

    #[test]
    fn test_three() {
        assert_eq!(Day1::init("(((".into()).part1(), "3");
        assert_eq!(Day1::init("(()(()(".into()).part1(), "3");
        assert_eq!(Day1::init("))(((((".into()).part1(), "3");
    }

    #[test]
    fn test_negative_one() {
        assert_eq!(Day1::init("())".into()).part1(), "-1");
        assert_eq!(Day1::init("))(".into()).part1(), "-1");
    }

    #[test]
    fn test_negative_three() {
        assert_eq!(Day1::init(")))".into()).part1(), "-3");
        assert_eq!(Day1::init(")())())".into()).part1(), "-3");
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Day1::init(")".into()).part2(), "1");
        assert_eq!(Day1::init("()())".into()).part2(), "5");
    }
}
