use std::collections::HashMap;

use aoc_utils::Day;

pub struct Day11(String);

impl Day for Day11 {
    fn init(input: &str) -> Box<dyn Day>
    where
        Self: Sized,
    {
        Box::new(Self(input.into()))
    }

    fn part1(&self) -> String {
        let mut devices = HashMap::new();
        for line in self.0.lines() {
            let (device, others) = line.split_once(": ").unwrap();
            let others: Box<[&str]> = others.split(' ').collect();
            devices.insert(device, others);
        }

        get_paths_to_out(&devices, "you").to_string()
    }

    fn part2(&self) -> String {
        let mut devices = HashMap::new();
        let mut results = HashMap::new();
        for line in self.0.lines() {
            let (device, others) = line.split_once(": ").unwrap();
            let others: Box<[&str]> = others.split(' ').collect();
            devices.insert(device, others);
        }

        get_paths_to_out_filtered(&devices, "svr", false, false, &mut results).to_string()
    }
}

fn get_paths_to_out(devices: &HashMap<&str, Box<[&str]>>, device: &str) -> u64 {
    if device == "out" {
        1
    } else {
        devices
            .get(device)
            .unwrap()
            .iter()
            .map(|device| get_paths_to_out(devices, device))
            .sum()
    }
}

fn get_paths_to_out_filtered<'a>(
    devices: &'a HashMap<&str, Box<[&str]>>,
    device: &'a str,
    dac: bool,
    fft: bool,
    results: &mut HashMap<(&'a str, bool, bool), u64>,
) -> u64 {
    if let Some(result) = results.get(&(device, dac, fft)) {
        *result
    } else if device == "out" {
        let result = if dac && fft { 1 } else { 0 };
        results.insert((device, dac, fft), result);
        result
    } else {
        let new_dac = (device == "dac") || dac;
        let new_fft = (device == "fft") || fft;
        let result = devices
            .get(device)
            .unwrap()
            .iter()
            .map(|device| get_paths_to_out_filtered(devices, device, new_dac, new_fft, results))
            .sum();

        results.insert((device, dac, fft), result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            Day11::init(
                "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
"
            )
            .part1(),
            "5"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            Day11::init(
                "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
"
            )
            .part2(),
            "2"
        );
    }
}
