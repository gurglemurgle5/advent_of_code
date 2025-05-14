use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let mut moons: Vec<Moon> = input
        .lines()
        .map(|line| &line[1..(line.len() - 1)])
        .map(|line| {
            let vec: Vec<i32> = line
                .split(", ")
                .map(|part| part[2..].parse().unwrap())
                .collect();
            Moon::new(vec.try_into().unwrap())
        })
        .collect();

    for i in 0..1000 {
        for left in 0..moons.len() {
            for right in 0..moons.len() {
                if left == right {
                    continue;
                }
                let other = moons[right];
                moons[left].gravity(other);
            }
        }
        for moon in &mut moons {
            moon.tick();
        }
    }

    dbg!(moons.iter().map(|moon| moon.energy()).sum::<i32>());
}

#[derive(Clone, Copy, Debug)]
struct Moon {
    pos: [i32; 3],
    vel: [i32; 3],
}

impl Moon {
    fn new(pos: [i32; 3]) -> Moon {
        Moon { pos, vel: [0; 3] }
    }

    fn gravity(&mut self, other: Moon) {
        for ((pos, vel), other) in self
            .pos
            .iter()
            .zip(self.vel.iter_mut())
            .zip(other.pos.iter())
        {
            if *pos > *other {
                *vel -= 1;
            } else if *pos < *other {
                *vel += 1;
            }
        }
    }

    fn tick(&mut self) {
        for (pos, vel) in self.pos.iter_mut().zip(self.vel.iter()) {
            *pos += *vel;
        }
    }

    fn energy(&self) -> i32 {
        self.pos.iter().map(|&pos| pos.abs()).sum::<i32>()
            * self.vel.iter().map(|&vel| vel.abs()).sum::<i32>()
    }
}
