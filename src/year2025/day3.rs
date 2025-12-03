use aoc_utils::Day;

pub struct Day3(String);

impl Day for Day3 {
    fn init(input: &str) -> Box<dyn Day>
    where
        Self: Sized,
    {
        Box::new(Self(input.into()))
    }

    fn part1(&self) -> String {
        self.0
            .lines()
            .map(|line| {
                let len = line.len();
                let (left_index, left) =
                    line.chars()
                        .enumerate()
                        .take(len - 1)
                        .fold(
                            (0, '\x00'),
                            |old, new| if new.1 > old.1 { new } else { old },
                        );

                let right = line.chars().skip(left_index + 1).max().unwrap();
                ((left as u8 - b'0') * 10 + (right as u8 - b'0')) as u16
            })
            .sum::<u16>()
            .to_string()
    }

    fn part2(&self) -> String {
        self.0
            .lines()
            .map(|line| {
                let len = line.len();
                let mut skip = 0;

                (0..12)
                    .map(|i| {
                        let size = len - skip - (11 - i);
                        let (index, max) = line.chars().enumerate().skip(skip).take(size).fold(
                            (0, '\x00'),
                            |old, new| if new.1 > old.1 { new } else { old },
                        );
                        skip = index + 1;
                        max
                    })
                    .fold(0, |num, char| num * 10 + (char as u8 - b'0') as u64)
            })
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
            Day3::init("987654321111111\n811111111111119\n234234234234278\n818181911112111")
                .part1(),
            "357"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            Day3::init("987654321111111\n811111111111119\n234234234234278\n818181911112111")
                .part2(),
            "3121910778619"
        );
    }
}
