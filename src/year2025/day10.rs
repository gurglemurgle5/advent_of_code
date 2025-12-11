use aoc_utils::Day;

pub struct Day10(String);

impl Day for Day10 {
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
                let mut parts = line.split(' ');
                let display = parts.next().unwrap();
                let _joltage = parts.next_back().unwrap();
                let buttons = parts;

                let display: Box<[bool]> = display
                    .chars()
                    .take(display.len() - 1)
                    .skip(1)
                    .map(|char| char == '#')
                    .collect();
                let buttons: Box<[Box<[usize]>]> = buttons
                    .map(|button| {
                        button[1..(button.len() - 1)]
                            .split(',')
                            .map(|light| light.parse().unwrap())
                            .collect()
                    })
                    .collect();

                (0..2i32.pow(buttons.len() as u32))
                    .filter_map(|i| {
                        let mut strat = vec![false; buttons.len()].into_boxed_slice();
                        for j in 0..strat.len() {
                            strat[j] = i & (1 << j) > 0;
                        }

                        let mut attempt = display.clone();
                        for (index, _) in strat.iter().enumerate().filter(|(_, bool)| **bool) {
                            for &light in &buttons[index] {
                                attempt[light] = !attempt[light];
                            }
                        }
                        if !attempt.contains(&true) {
                            Some(strat.iter().filter(|bool| **bool).count())
                        } else {
                            None
                        }
                    })
                    .min()
                    .unwrap()

                // to undo the display, we need to set all the lights to off (false)
                // this is the same as starting from all off and changing it to the display
            })
            .sum::<usize>()
            .to_string()
    }

    fn part2(&self) -> String {
        self.0
            .lines()
            .map(|line| {
                let mut parts = line.split(' ');
                let _display = parts.next().unwrap();
                let joltage = parts.next_back().unwrap();
                let buttons = parts;

                let buttons: Box<[Box<[usize]>]> = buttons
                    .map(|button| {
                        button[1..(button.len() - 1)]
                            .split(',')
                            .map(|light| light.parse().unwrap())
                            .collect()
                    })
                    .collect();

                let joltage: Box<[u8]> = joltage[1..(joltage.len() - 1)]
                    .split(',')
                    .map(|num| num.parse().unwrap())
                    .collect();

                todo!("yeah idk");
                0
            })
            .sum::<usize>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            Day10::init(
                "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
"
            )
            .part1(),
            "7"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            Day10::init(
                "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
"
            )
            .part2(),
            "33"
        );
    }
}
