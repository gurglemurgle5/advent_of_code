use aoc_utils::{Day, grid::Grid};

pub struct Day6(String);

impl Day for Day6 {
    fn init(input: &str) -> Box<dyn Day>
    where
        Self: Sized,
    {
        Box::new(Self(input.into()))
    }

    fn part1(&self) -> String {
        let mut lists: Vec<Vec<u64>> = Vec::new();

        for line in self.0.lines() {
            for (i, num) in line.split(' ').filter(|num| !num.is_empty()).enumerate() {
                if num == "*" || num == "+" {
                    break;
                }
                if i >= lists.len() {
                    lists.push(Vec::new());
                }
                lists[i].push(num.parse().unwrap());
            }
        }

        let ops: Vec<Ops> = self
            .0
            .lines()
            .last()
            .unwrap()
            .chars()
            .filter_map(|char| match char {
                '*' => Some(Ops::Multiply),
                '+' => Some(Ops::Add),
                _ => None,
            })
            .collect();

        let mut total = 0;
        for i in 0..(ops.len()) {
            let op = &ops[i];
            total += match op {
                Ops::Add => lists[i].iter().sum::<u64>(),
                Ops::Multiply => lists[i].iter().product(),
            };
        }
        total.to_string()
    }

    fn part2(&self) -> String {
        let mut lists: Vec<Vec<u64>> = vec![vec![]];

        let lines: Grid<char> = self.0.parse().unwrap();
        let mut col = 0;
        let mut relative_i = 0;
        for i in 0..lines.width() {
            let mut was_all_whitespace = true;
            for j in 0..lines.height() {
                let char = lines[(i, j)];

                if char == ' ' || char == '*' || char == '+' {
                    continue;
                }
                was_all_whitespace = false;

                if lists[col].len() <= relative_i {
                    lists[col].resize(relative_i + 1, 0);
                }

                lists[col][relative_i] *= 10;
                lists[col][relative_i] += (char as u8 - b'0') as u64;
            }
            if was_all_whitespace {
                col += 1;
                relative_i = 0;
                lists.push(Vec::new());
            } else {
                relative_i += 1;
            }
        }

        let ops: Vec<Ops> = self
            .0
            .lines()
            .last()
            .unwrap()
            .chars()
            .filter_map(|char| match char {
                '*' => Some(Ops::Multiply),
                '+' => Some(Ops::Add),
                _ => None,
            })
            .collect();

        let mut total = 0;
        for i in 0..(ops.len()) {
            let op = &ops[i];
            total += match op {
                Ops::Add => lists[i].iter().sum::<u64>(),
                Ops::Multiply => lists[i].iter().product(),
            };
        }
        total.to_string()
    }
}

enum Ops {
    Add,
    Multiply,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            Day6::init(
                "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
"
            )
            .part1(),
            "4277556"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            Day6::init(
                "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
"
            )
            .part2(),
            "3263827"
        );
    }
}
