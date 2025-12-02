use aoc_utils::Day;

pub struct Day1(Box<[i16]>);

impl Day for Day1 {
    fn init(input: &str) -> Box<dyn Day>
    where
        Self: Sized,
    {
        Box::new(Self(
            input
                .lines()
                .map(|line| {
                    (match line.chars().next().unwrap() {
                        'L' => -1,
                        'R' => 1,
                        _ => panic!(),
                    }) * line[1..].parse::<i16>().unwrap()
                })
                .collect(),
        ))
    }

    fn part1(&self) -> String {
        let mut num: i32 = 50;
        let mut total = 0;

        for travel in &self.0 {
            num += *travel as i32;
            num = num.rem_euclid(100);
            if num == 0 {
                total += 1;
            }
        }
        total.to_string()
    }

    fn part2(&self) -> String {
        let mut num: i32 = 50;
        let mut total = 0;

        for &travel in &self.0 {
            for _ in 0..(travel.abs()) {
                num += travel.signum() as i32;
                num = num.rem_euclid(100);
                if num == 0 {
                    total += 1;
                }
            }
        }
        total.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            Day1::init("L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82").part1(),
            "3"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            Day1::init("L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82").part2(),
            "6"
        );
    }
}
