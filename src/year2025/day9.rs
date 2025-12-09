use aoc_utils::Day;

pub struct Day9(String);

impl Day for Day9 {
    fn init(input: &str) -> Box<dyn Day>
    where
        Self: Sized,
    {
        Box::new(Self(input.into()))
    }

    fn part1(&self) -> String {
        let tiles: Vec<(u64, u64)> = self
            .0
            .lines()
            .map(|line| {
                let (x, y) = line.split_once(',').unwrap();
                (x.parse().unwrap(), y.parse().unwrap())
            })
            .collect();

        let mut sizes = Vec::new();

        for i in 0..(tiles.len() - 1) {
            for j in (i + 1)..tiles.len() {
                let left = tiles[i];
                let right = tiles[j];
                let size = (left.0.max(right.0) - left.0.min(right.0) + 1)
                    * (left.1.max(right.1) - left.1.min(right.1) + 1);
                sizes.push((size, (left, right)));
            }
        }

        sizes.iter().max().unwrap().0.to_string()
    }

    fn part2(&self) -> String {
        let tiles: Vec<(u64, u64)> = self
            .0
            .lines()
            .map(|line| {
                let (x, y) = line.split_once(',').unwrap();
                (x.parse().unwrap(), y.parse().unwrap())
            })
            .collect();

        let mut lines = Vec::new();
        for i in 0..tiles.len() {
            if i == 0 {
                lines.push((tiles[tiles.len() - 1], tiles[0]));
            } else {
                lines.push((tiles[i - 1], tiles[i]));
            }
        }

        let mut sizes = Vec::new();

        for i in 0..(tiles.len() - 1) {
            'finding_sizes: for j in (i + 1)..tiles.len() {
                let left = tiles[i];
                let right = tiles[j];
                let x_range = (left.0.min(right.0) + 1)..=(left.0.max(right.0) - 1);
                let y_range = (left.1.min(right.1) + 1)..=(left.1.max(right.1) - 1);

                for &(x, y) in &tiles {
                    if x_range.contains(&x) && y_range.contains(&y) {
                        continue 'finding_sizes;
                    }
                }

                let raycast_x = (left.0 + right.0) / 2;
                let raycast_y = (left.1 + right.1) / 2;

                let mut inside = false;

                for line in &lines {
                    if line.0.0 == line.1.0 {
                        // vertical line
                        if x_range.contains(&line.0.0)
                            && line.0.1.min(line.1.1) < *y_range.start()
                            && line.0.1.max(line.1.1) > *y_range.end()
                        {
                            continue 'finding_sizes;
                        }

                        if line.0.0 > raycast_x
                            && ((line.0.1.min(line.1.1))..=(line.0.1.max(line.1.1)))
                                .contains(&raycast_y)
                        {
                            inside = !inside;
                        }
                    } else {
                        // horizontal line
                        if y_range.contains(&line.0.1)
                            && line.0.0.min(line.1.0) < *x_range.start()
                            && line.0.0.max(line.1.0) > *x_range.end()
                        {
                            continue 'finding_sizes;
                        }
                    }
                }

                if !inside {
                    continue 'finding_sizes;
                }

                let size = (left.0.max(right.0) - left.0.min(right.0) + 1)
                    * (left.1.max(right.1) - left.1.min(right.1) + 1);
                sizes.push((size, (left, right)));
            }
        }

        sizes.iter().max().unwrap().0.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            Day9::init(
                "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
"
            )
            .part1(),
            "50"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            Day9::init(
                "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
"
            )
            .part2(),
            "24"
        );
    }
}
