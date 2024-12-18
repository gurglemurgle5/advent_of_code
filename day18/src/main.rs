use std::{
    collections::{HashMap, HashSet},
    fs,
};

const WIDTH: i32 = 71;
const HEIGHT: i32 = 71;
const LIMIT: usize = 1024;

fn main() {
    let input: Vec<(i32, i32)> = fs::read_to_string("./input.txt")
        .unwrap()
        .lines()
        .map(|line| line.split_once(",").unwrap())
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .collect();
    let mut map = HashSet::new();

    for byte in input {
        println!("testing {byte:?}");
        map.insert(byte);
        let path = a_star(&map, (0, 0), (WIDTH - 1, HEIGHT - 1), |(x, y)| {
            (WIDTH - x - 1).abs() + (HEIGHT - y - 1).abs()
        });
    }

    // let path = a_star(input, (0, 0), (WIDTH - 1, HEIGHT - 1), |(x, y)| {
    //     (WIDTH - x - 1).abs() + (HEIGHT - y - 1).abs()
    // });
    // dbg!(path.len());
}

fn reconstruct_path(
    came_from: HashMap<(i32, i32), (i32, i32)>,
    mut current: (i32, i32),
) -> Vec<(i32, i32)> {
    let mut total_path = Vec::new();
    // let mut score = 0;
    while let Some(other) = came_from.get(&current) {
        // score += d(current, *other);
        current = *other;
        total_path.push(current);
    }
    total_path.reverse();
    // score
    total_path
}

fn a_star<T: FnMut((i32, i32)) -> i32>(
    input: &HashSet<(i32, i32)>,
    start: (i32, i32),
    goal: (i32, i32),
    mut h: T,
) -> Vec<(i32, i32)> {
    let mut open_set: HashSet<(i32, i32)> = HashSet::new();
    open_set.insert(start);
    let mut came_from: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
    let mut g_score: HashMap<(i32, i32), i32> = HashMap::new();
    g_score.insert(start, 0);
    let mut f_score: HashMap<(i32, i32), i32> = HashMap::new();
    f_score.insert(start, h(start));

    while !open_set.is_empty() {
        let current = *open_set
            .iter()
            .min_by_key(|node| f_score.get(node).unwrap_or(&i32::MAX))
            .unwrap();
        if current == goal {
            return reconstruct_path(came_from, current);
        }
        open_set.remove(&current);
        for neighbor in generate_neighbors(&input, current) {
            let tentative_g_score = g_score.get(&current).unwrap() + d(current, neighbor);
            if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&i32::MAX) {
                came_from.insert(neighbor, current);
                g_score.insert(neighbor, tentative_g_score);
                f_score.insert(neighbor, tentative_g_score + h(neighbor));
                open_set.insert(neighbor);
            }
        }
    }
    panic!("failed to reach goal!");
}

/// PRECONDITION: current and neighbor are direct neighbors
fn d(current: (i32, i32), neighbor: (i32, i32)) -> i32 {
    1
}

fn generate_neighbors(input: &HashSet<(i32, i32)>, (x, y): (i32, i32)) -> Vec<(i32, i32)> {
    let mut neighbors = Vec::new();

    if !input.contains(&(x + 1, y)) && x + 1 < WIDTH {
        neighbors.push((x + 1, y));
    }
    if !input.contains(&(x - 1, y)) && x > 0 {
        neighbors.push((x - 1, y));
    }
    if !input.contains(&(x, y + 1)) && y + 1 < HEIGHT {
        neighbors.push((x, y + 1));
    }
    if !input.contains(&(x, y - 1)) && y > 0 {
        neighbors.push((x, y - 1));
    }

    neighbors
}
