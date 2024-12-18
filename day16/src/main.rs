use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Node {
    row: usize,
    col: usize,
    dir: Direction,
}

fn main() {
    let (mut start_row, mut start_col) = (0, 0);
    let (mut end_row, mut end_col) = (0, 0);
    let map: Vec<Vec<char>> = fs::read_to_string("./input.txt")
        .unwrap()
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, char)| match char {
                    'S' => {
                        (start_row, start_col) = (row, col);
                        '.'
                    }
                    'E' => {
                        (end_row, end_col) = (row, col);
                        '.'
                    }
                    _ => char,
                })
                .collect()
        })
        .collect();

    let result = a_star(
        map,
        Node {
            row: start_row,
            col: start_col,
            dir: Direction::Right,
        },
        Node {
            row: end_row,
            col: end_col,
            dir: Direction::Up,
        },
        |node| {
            (node.row as i32 - end_row as i32).unsigned_abs() as usize
                + (node.col as i32 - end_col as i32).unsigned_abs() as usize
        },
    );
    dbg!(result);
}

fn reconstruct_path(came_from: HashMap<Node, Node>, mut current: Node) -> usize {
    // let mut total_path = Vec::new();
    let mut score = 0;
    while let Some(other) = came_from.get(&current) {
        score += d(current, *other);
        current = *other;
        // total_path.push(current);
    }
    // total_path.reverse();
    score
}

fn a_star<T: FnMut(Node) -> usize>(
    map: Vec<Vec<char>>,
    start: Node,
    goal: Node,
    mut h: T,
) -> usize {
    let mut open_set: HashSet<Node> = HashSet::new();
    open_set.insert(start);
    let mut came_from: HashMap<Node, Node> = HashMap::new();
    let mut g_score: HashMap<Node, usize> = HashMap::new();
    g_score.insert(start, 0);
    let mut f_score: HashMap<Node, usize> = HashMap::new();
    f_score.insert(start, h(start));

    while !open_set.is_empty() {
        let current = *open_set
            .iter()
            .min_by_key(|node| f_score.get(node).unwrap_or(&usize::MAX))
            .unwrap();
        // We don't use direct equality, as we don't care about direction once we reach goal.
        if current.row == goal.row && current.col == goal.col {
            return reconstruct_path(came_from, current);
        }
        open_set.remove(&current);
        for neighbor in generate_neighbors(&map, current) {
            let tentative_g_score = g_score.get(&current).unwrap() + d(current, neighbor);
            if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&usize::MAX) {
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
fn d(current: Node, neighbor: Node) -> usize {
    if current.dir == neighbor.dir {
        // direction is the same, so we're moving 1 tile forwards
        1
    } else {
        // direction is different, so we are staying in the same spot
        1000
    }
}

fn generate_neighbors(map: &[Vec<char>], node: Node) -> Vec<Node> {
    let mut neighbors = vec![
        Node {
            row: node.row,
            col: node.col,
            dir: Direction::Up,
        },
        Node {
            row: node.row,
            col: node.col,
            dir: Direction::Down,
        },
        Node {
            row: node.row,
            col: node.col,
            dir: Direction::Left,
        },
        Node {
            row: node.row,
            col: node.col,
            dir: Direction::Right,
        },
    ];
    match node.dir {
        Direction::Up => {
            if map[node.row - 1][node.col] == '.' {
                neighbors.push(Node {
                    row: node.row - 1,
                    col: node.col,
                    dir: node.dir,
                });
            }
        }
        Direction::Down => {
            if map[node.row + 1][node.col] == '.' {
                neighbors.push(Node {
                    row: node.row + 1,
                    col: node.col,
                    dir: node.dir,
                });
            }
        }
        Direction::Left => {
            if map[node.row][node.col - 1] == '.' {
                neighbors.push(Node {
                    row: node.row,
                    col: node.col - 1,
                    dir: node.dir,
                });
            }
        }
        Direction::Right => {
            if map[node.row][node.col + 1] == '.' {
                neighbors.push(Node {
                    row: node.row,
                    col: node.col + 1,
                    dir: node.dir,
                });
            }
        }
    }
    neighbors
}
