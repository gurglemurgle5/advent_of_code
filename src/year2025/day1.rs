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
        let mut num = 50;
        let mut total = 0;

        for line in self.0.lines() {
            let dir = line.chars().next().unwrap();
            let dist: i32 = line[1..].parse().unwrap();
            match dir {
                'L' => num -= dist,
                'R' => num += dist,
                _ => panic!(),
            }
            num %= 100;
            if num < 0 {
                num += 100;
            }
            if num == 0 {
                total += 1;
            }
        }
        total.to_string()
    }

    fn part2(&self) -> String {
        let mut num = 50;
        let mut total = 0;

        for line in self.0.lines() {
            let dist: i32 = line[1..].parse().unwrap();
            let dir = match line.chars().next().unwrap() {
                'L' => -1,
                'R' => 1,
                _ => panic!(),
            };
            let mut times_passed = 0;
            for _ in 0..dist {
                num += dir;
                num %= 100;
                if num < 0 {
                    num += 100;
                }
                if num == 0 {
                    times_passed += 1;
                }
            }
            total += times_passed;
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
            Day1::init("L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82".to_string()).part1(),
            "3"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            Day1::init("L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82".to_string()).part2(),
            "6"
        );
    }
}
