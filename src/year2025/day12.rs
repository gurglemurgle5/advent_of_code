use aoc_utils::Day;

pub struct Day12(String);

impl Day for Day12 {
    fn init(input: &str) -> Box<dyn Day>
    where
        Self: Sized,
    {
        Box::new(Self(input.into()))
    }

    fn part1(&self) -> String {
        let shapes = self.0.lines().take(30);
        let mut shape_sizes: Vec<usize> = vec![0];
        for line in shapes {
            if line.is_empty() {
                shape_sizes.push(0);
            } else {
                *shape_sizes.last_mut().unwrap() +=
                    line.chars().filter(|&char| char == '#').count();
            }
        }

        let regions: Box<[(usize, usize, Box<[usize]>)]> = self
            .0
            .lines()
            .skip(30)
            .map(|line| {
                let mut parts = line.split(' ');
                let width_height = parts.next().unwrap().strip_suffix(':').unwrap();
                let (width, height) = width_height.split_once('x').unwrap();
                let width = width.parse().unwrap();
                let height = height.parse().unwrap();
                let counts = parts.map(|num| num.parse().unwrap()).collect();
                (width, height, counts)
            })
            .collect();

        let mut total_valid = 0;
        for (width, height, counts) in regions {
            let area = width * height;
            let shape_area: usize = counts
                .iter()
                .enumerate()
                .map(|(i, count)| shape_sizes[i] * count)
                .sum();

            if shape_area > area {
                continue;
            }

            if ((width / 3) * 3) * ((height / 3) * 3) >= (counts.iter().copied().sum::<usize>() * 9)
            {
                total_valid += 1;
                continue;
            }

            // Somehow this actually works, and this panic is never reached
            // not doing it on example though
            panic!("whoops")
        }

        total_valid.to_string()
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
}
