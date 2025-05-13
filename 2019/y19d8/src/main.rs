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
    let mut layer_least_zeros = layers[0];
    let mut least_zeros = usize::MAX;

    for &layer in &layers {
        let zeros = layer.iter().filter(|&&num| num == '0').count();
        if zeros < least_zeros {
            least_zeros = zeros;
            layer_least_zeros = layer;
        }
    }

    let ones = layer_least_zeros.iter().filter(|&&num| num == '1').count();
    let twos = layer_least_zeros.iter().filter(|&&num| num == '2').count();

    dbg!(ones * twos);
}
