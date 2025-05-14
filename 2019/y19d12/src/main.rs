use aoc_utils::lcm_i64;
use std::{cmp::Ordering, fs};

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let mut moons: Vec<Moon> = input
        .lines()
        .map(|line| &line[1..(line.len() - 1)])
        .map(|line| {
            let vec: Vec<i64> = line
                .split(", ")
                .map(|part| part[2..].parse().unwrap())
                .collect();
            Moon::new(vec.try_into().unwrap())
        })
        .collect();

    let initial = moons.clone();
    let mut x = None;
    let mut y = None;
    let mut z = None;
    let mut i: i64 = 0;
    loop {
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

        i += 1;

        if x.is_none() {
            let mut is_x = true;
            for (i, moon) in moons.iter().enumerate() {
                if moon.vel[0] != 0 || moon.pos[0] != initial[i].pos[0] {
                    is_x = false;
                }
            }
            if is_x {
                x = Some(i);
            }
        }
        if y.is_none() {
            let mut is_y = true;
            for (i, moon) in moons.iter().enumerate() {
                if moon.vel[1] != 0 || moon.pos[1] != initial[i].pos[1] {
                    is_y = false;
                    break;
                }
            }
            if is_y {
                y = Some(i);
            }
        }
        if z.is_none() {
            let mut is_z = true;
            for (i, moon) in moons.iter().enumerate() {
                if moon.vel[2] != 0 || moon.pos[2] != initial[i].pos[2] {
                    is_z = false;
                    break;
                }
            }
            if is_z {
                z = Some(i);
            }
        }

        if i % 10_000_000 == 0 {
            dbg!(i);
        }
        if x.is_some() && y.is_some() && z.is_some() {
            break;
        }
    }

    dbg!(x, y, z);
    dbg!(lcm_i64(x.unwrap(), lcm_i64(y.unwrap(), z.unwrap())));
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Moon {
    pos: [i64; 3],
    vel: [i64; 3],
}

impl Moon {
    fn new(pos: [i64; 3]) -> Moon {
        Moon { pos, vel: [0; 3] }
    }

    fn gravity(&mut self, other: Moon) {
        for ((pos, vel), other) in self
            .pos
            .iter()
            .zip(self.vel.iter_mut())
            .zip(other.pos.iter())
        {
            match pos.cmp(other) {
                Ordering::Greater => *vel -= 1,
                Ordering::Less => *vel += 1,
                Ordering::Equal => (),
            }
        }
    }

    fn tick(&mut self) {
        for (pos, vel) in self.pos.iter_mut().zip(self.vel.iter()) {
            *pos += *vel;
        }
    }
}
