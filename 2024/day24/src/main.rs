use std::{collections::HashMap, fs};

enum Operation {
    And,
    Or,
    Xor,
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let mut wires: HashMap<&str, bool> = HashMap::new();
    let (input, gates) = input.trim().split_once("\n\n").unwrap();
    for line in input.lines() {
        let (gate, num) = line.split_once(": ").unwrap();
        wires.insert(gate, num.parse::<u32>().unwrap() == 1);
    }
    let mut gates: Vec<(&str, &str, Operation, &str)> = gates
        .lines()
        .map(|line| {
            let mut stuffs = line.split(" ");
            let a = stuffs.next().unwrap();
            let operation = match stuffs.next().unwrap() {
                "AND" => Operation::And,
                "OR" => Operation::Or,
                "XOR" => Operation::Xor,
                _ => panic!("bad"),
            };
            let b = stuffs.next().unwrap();
            stuffs.next().unwrap();
            let out = stuffs.next().unwrap();
            (a, b, operation, out)
        })
        .collect();
    while !gates.is_empty() {
        gates.retain(|(a, b, operation, out)| {
            let a = match wires.get(a) {
                Some(a) => *a,
                None => return true,
            };
            let b = match wires.get(b) {
                Some(b) => *b,
                None => return true,
            };
            wires.insert(
                out,
                match operation {
                    Operation::And => a && b,
                    Operation::Or => a || b,
                    Operation::Xor => a ^ b,
                },
            );

            false
        });
    }

    let mut i = 0;
    let mut out = 0;
    while let Some(&bit) = wires.get(format!("z{i:0>2}").as_str()) {
        out |= (bit as i64) << i;
        i += 1;
    }
    dbg!(out);
}
