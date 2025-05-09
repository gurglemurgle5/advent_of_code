use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let (register_data, instructions) = input.split_once("\n\n").unwrap();
    let mut a: i64 = 0;
    let mut b: i64 = 0;
    let mut c: i64 = 0;
    for (num, line) in register_data.lines().enumerate() {
        let data = line.split_once(": ").unwrap().1.parse().unwrap();
        match num {
            0 => a = data,
            1 => b = data,
            2 => c = data,
            _ => panic!("whoops"),
        }
    }
    // this takes only the first line because i wanna add comments after it
    let instructions: Vec<u8> = instructions
        .lines()
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split(",")
        .map(|num| num.parse().unwrap())
        .collect();
    let mut a: i64 = 0;

    if let Some(full_a) = generate_a(a, &instructions) {
        a = full_a;
        dbg!(a);
    }

    let mut instruction_pointer = 0;
    let mut output = Vec::new();

    while instruction_pointer != instructions.len() {
        let opcode = instructions[instruction_pointer];
        let literal_operand = instructions[instruction_pointer + 1] as i64;
        instruction_pointer += 2;
        let combo_operand = match literal_operand {
            0..=3 => literal_operand,
            4 => a,
            5 => b,
            6 => c,
            7 => 7, // to prevent the code from crashing
            _ => panic!("invalid number!"),
        };
        match opcode {
            0 => a >>= combo_operand,
            1 => b ^= literal_operand,
            2 => b = combo_operand & 0b0111,
            3 => {
                if a != 0 {
                    instruction_pointer = literal_operand as usize;
                }
            }
            4 => b ^= c,
            5 => output.push(combo_operand as u8 & 0b0111),
            6 => b = a >> combo_operand,
            7 => c = a >> combo_operand,
            _ => panic!("invalid number!"),
        }
    }
    if output == instructions {
        println!("it works!");
    } else {
        println!("it was wrong ):");
    }
}

fn generate_a(mut a: i64, instructions: &[u8]) -> Option<i64> {
    if instructions.is_empty() {
        return Some(a >> 3);
    }
    let expected = instructions[instructions.len() - 1];
    for test_bits in 0..=0b111 {
        a = (a & (!0b111)) | test_bits;

        // this code is based off of my input
        let mut b = a & 0b0111;
        b ^= 0b011;
        let c = a >> b;
        b ^= c;
        b ^= 0b101;

        let output = b & 0b0111;
        if output as u8 == expected {
            if let Some(a) = generate_a(a << 3, &instructions[0..(instructions.len() - 1)]) {
                return Some(a);
            }
        }
    }
    None
}
