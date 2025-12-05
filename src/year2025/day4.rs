use aoc_utils::Day;

pub struct Day4(String);

impl Day for Day4 {
    fn init(input: &str) -> Box<dyn Day>
    where
        Self: Sized,
    {
        Box::new(Self(input.into()))
    }

    fn part1(&self) -> String {
        let grid: Vec<Vec<char>> = self.0.lines().map(|line| line.chars().collect()).collect();

        let mut total = 0;

        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if grid[y][x] == '.' {
                    continue;
                }
                let init_x = x as i32;
                let init_y = y as i32;
                let mut nearby = 0;

                for y in (init_y - 1)..=(init_y + 1) {
                    if y < 0 || y >= grid.len() as i32 {
                        continue;
                    }
                    for x in (init_x - 1)..=(init_x + 1) {
                        if x < 0 || x >= grid[y as usize].len() as i32 {
                            continue;
                        }

                        if grid[y as usize][x as usize] == '@' {
                            nearby += 1;
                        }
                    }
                }

                if nearby < 5 {
                    total += 1;
                }
            }
        }

        total.to_string()
    }

    fn part2(&self) -> String {
        let mut grid: Vec<Vec<char>> = self.0.lines().map(|line| line.chars().collect()).collect();

        let mut total = 0;

        let mut did_something_happen = false;

        loop {
            for y in 0..grid.len() {
                for x in 0..grid[y].len() {
                    if grid[y][x] == '.' {
                        continue;
                    }
                    let init_x = x as i32;
                    let init_y = y as i32;
                    let mut nearby = 0;

                    for y in (init_y - 1)..=(init_y + 1) {
                        if y < 0 || y >= grid.len() as i32 {
                            continue;
                        }
                        for x in (init_x - 1)..=(init_x + 1) {
                            if x < 0 || x >= grid[y as usize].len() as i32 {
                                continue;
                            }

                            if grid[y as usize][x as usize] == '@' {
                                nearby += 1;
                            }
                        }
                    }

                    if nearby < 5 {
                        total += 1;
                        grid[y][x] = '.';
                        did_something_happen = true
                    }
                }
            }
            if !did_something_happen {
                break;
            } else {
                did_something_happen = false
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
            Day4::init(
                "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
"
            )
            .part1(),
            "13"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            Day4::init(
                "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
"
            )
            .part2(),
            "43"
        );
    }
}
