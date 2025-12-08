use std::collections::{HashMap, HashSet};

use aoc_utils::Day;

pub struct Day8(String);

impl Day for Day8 {
    fn init(input: &str) -> Box<dyn Day>
    where
        Self: Sized,
    {
        Box::new(Self(input.into()))
    }

    fn part1(&self) -> String {
        let junction_boxes: Vec<(u64, u64, u64)> = self
            .0
            .lines()
            .map(|line| {
                let (x, right) = line.split_once(',').unwrap();
                let (y, z) = right.split_once(',').unwrap();
                (x.parse().unwrap(), y.parse().unwrap(), z.parse().unwrap())
            })
            .collect();

        let mut dists = Vec::new();

        for i in 0..(junction_boxes.len() - 1) {
            for j in (i + 1)..junction_boxes.len() {
                let left = junction_boxes[i];
                let right = junction_boxes[j];
                let dist = dist(left, right);
                dists.push((dist, (left, right)));
            }
        }
        dists.sort();

        let connections: Vec<_> = dists
            .iter()
            .take(TAKE)
            .copied()
            .map(|(_, (left, right))| (left, right))
            .collect();

        let groups = group_pairs(&connections);

        let mut sizes: Vec<u64> = groups.iter().map(|group| group.len() as u64).collect();

        sizes.sort();

        // println!("{sizes:?}");

        sizes.iter().rev().take(3).product::<u64>().to_string()
    }

    fn part2(&self) -> String {
        let junction_boxes: Vec<(u64, u64, u64)> = self
            .0
            .lines()
            .map(|line| {
                let (x, right) = line.split_once(',').unwrap();
                let (y, z) = right.split_once(',').unwrap();
                (x.parse().unwrap(), y.parse().unwrap(), z.parse().unwrap())
            })
            .collect();

        let mut dists = Vec::new();

        for i in 0..(junction_boxes.len() - 1) {
            for j in (i + 1)..junction_boxes.len() {
                let left = junction_boxes[i];
                let right = junction_boxes[j];
                let dist = dist(left, right);
                dists.push((dist, (left, right)));
            }
        }
        dists.sort();

        let mut groups = Vec::new();

        let mut result = None;

        for (_, (left, right)) in dists.iter().copied() {
            let mut set = HashSet::new();
            set.insert(left);
            set.insert(right);
            groups.push(set);
            groups = reduce_groups(groups);

            if groups
                .first()
                .is_some_and(|set| set.len() == junction_boxes.len())
            {
                result = Some((left, right));
                break;
            }
        }

        let (left, right) = result.unwrap();
        (left.0 * right.0).to_string()
    }
}

fn group_pairs(
    connections: &[((u64, u64, u64), (u64, u64, u64))],
) -> Vec<HashSet<(u64, u64, u64)>> {
    let mut groups: Vec<HashSet<(u64, u64, u64)>> = Vec::new();

    for (left, right) in connections {
        let mut found_group = false;
        for group in &mut groups {
            if group.contains(left) {
                group.insert(*right);
                found_group = true;
            } else if group.contains(right) {
                group.insert(*left);
                found_group = true;
            }
        }

        if !found_group {
            let mut set = HashSet::new();
            set.insert(*left);
            set.insert(*right);
            groups.push(set);
        }
    }

    reduce_groups(groups)
}

fn reduce_groups(mut groups: Vec<HashSet<(u64, u64, u64)>>) -> Vec<HashSet<(u64, u64, u64)>> {
    loop {
        let old_groups = groups;
        groups = old_groups.iter().fold(Vec::new(), |mut groups, set| {
            let mut found_group = false;
            for group in &mut groups {
                if group.intersection(set).next().is_some() {
                    group.extend(set);
                    found_group = true;
                    break;
                }
            }
            if !found_group {
                groups.push(set.clone());
            }
            groups
        });

        if *groups == old_groups {
            break;
        }
    }

    groups
}

#[cfg(test)]
const TAKE: usize = 10;

#[cfg(not(test))]
const TAKE: usize = 1000;

fn dist((x1, y1, z1): (u64, u64, u64), (x2, y2, z2): (u64, u64, u64)) -> u64 {
    (x1.max(x2) - x1.min(x2)).pow(2)
        + (y1.max(y2) - y1.min(y2)).pow(2)
        + (z1.max(z2) - z1.min(z2)).pow(2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            Day8::init(
                "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
"
            )
            .part1(),
            "40"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            Day8::init(
                "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
"
            )
            .part2(),
            "25272"
        );
    }
}
