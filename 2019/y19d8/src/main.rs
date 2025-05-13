use std::fs;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let input: Vec<char> = input.chars().collect();
    let mut input: &[char] = &input;
    let mut layers = Vec::new();
    while !input.is_empty() {
        layers.push(&input[0..(WIDTH * HEIGHT)]);
        input = &input[(WIDTH * HEIGHT)..];
    }
    layers.reverse();

    let mut image = ['0'; WIDTH * HEIGHT];

    for layer in layers {
        for (i, &char) in layer.iter().enumerate() {
            if char != '2' {
                image[i] = char;
            }
        }
    }

    for (i, char) in image.into_iter().enumerate() {
        if i % WIDTH == 0 && i != 0 {
            println!();
        }
        if char == '1' {
            print!("█");
        } else {
            print!(" ");
        }
    }
    println!();
}
