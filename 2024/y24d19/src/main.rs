use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let (towels, designs) = input.split_once("\n\n").unwrap();
    let towels: Vec<&str> = towels.split(", ").collect();
    let designs: Vec<&str> = designs.lines().collect();

    let mut total = 0;
    let mut cached_results = HashMap::new();

    for design in designs {
        total += try_convert(design, &towels, &mut cached_results)
    }
    dbg!(total);
}

fn try_convert<'a>(
    design: &'a str,
    towels: &[&'a str],
    cached_results: &mut HashMap<String, i64>,
) -> i64 {
    if design.is_empty() {
        return 1;
    }
    if let Some(result) = cached_results.get(design) {
        return *result;
    }
    let mut total = 0;
    for towel in towels {
        if let Some(stripped) = design.strip_prefix(towel) {
            total += try_convert(stripped, towels, cached_results);
        }
    }
    cached_results.insert(design.to_owned(), total);
    total
}
