use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let mut sum = 0;
    for line in input.lines() {
        let (value, nums) = line.split_once(": ").unwrap();
        let value: i64 = value.parse().unwrap();
        let nums: Vec<i64> = nums.split(" ").map(|num| num.parse().unwrap()).collect();
        let mut is_valid = false;

        let mut ops = vec![0; nums.len() - 1];

        'good_name_here: loop {
            let mut j = 0;
            let num = nums.iter().map(|num| *num).reduce(|left, right| {
                let num = match ops[j] {
                    0 => left * right,
                    1 => left + right,
                    2 => (left.to_string() + &right.to_string()).parse().unwrap(),
                    _ => unreachable!("the values in ops should never be greater than 2"),
                };
                j += 1;
                num
            });
            if num.unwrap() == value {
                is_valid = true;
                break;
            } else {
                let mut i = 0;
                loop {
                    if i == ops.len() {
                        break 'good_name_here;
                    }
                    if ops[i] < 2 {
                        ops[i] += 1;
                        break;
                    } else {
                        ops[i] = 0;
                        i += 1;
                    }
                }
            }
        }

        println!("{value}: {nums:?} ({is_valid})");

        if is_valid {
            sum += value;
        }
    }
    println!("{sum}");
}
