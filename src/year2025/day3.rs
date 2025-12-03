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
                let left = line.chars().take(len - 1).max().unwrap();
                let left_index = line
                    .chars()
                    .enumerate()
                    .find_map(|(i, char)| if char == left { Some(i) } else { None })
                    .unwrap();

                let right = line.chars().skip(left_index + 1).max().unwrap();
                format!("{left}{right}").parse::<u32>().unwrap()
            })
            .sum::<u32>()
            .to_string()
    }

    fn part2(&self) -> String {
        self.0
            .lines()
            .map(|line| {
                let len = line.len();
                let mut chars = Vec::new();
                let mut indicies = Vec::new();

                for i in 0..12 {
                    let skip = if i == 0 { 0 } else { indicies[i - 1] + 1 };
                    let size = len - skip - (11 - i);
                    let left = line.chars().skip(skip).take(size).max().unwrap();
                    let left_index = line
                        .chars()
                        .enumerate()
                        .skip(skip)
                        .take(size)
                        .find_map(|(i, char)| if char == left { Some(i) } else { None })
                        .unwrap();
                    chars.push(left);
                    indicies.push(left_index);
                }
                chars.iter().collect::<String>().parse::<u64>().unwrap()
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
            Day3::init(
                "987654321111111
811111111111119
234234234234278
818181911112111
"
                .trim()
            )
            .part1(),
            "357"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            Day3::init(
                "987654321111111
811111111111119
234234234234278
818181911112111
"
                .trim()
            )
            .part2(),
            "3121910778619"
        );
    }
}
