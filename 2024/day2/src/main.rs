use std::fs;

fn main() {
    let input: Vec<Vec<i32>> = fs::read_to_string("./input.txt")
        .unwrap()
        .lines()
        .map(|line| line.split(' ').map(|num| num.parse().unwrap()).collect())
        .collect();

    let mut safe_reports = 0;
    for report in input {
        let mut is_safe = is_report_safe(&report);

        if !is_safe {
            // this is a bad way of doing it but it's the easiest way
            for i in 0..(report.len()) {
                let mut modified_report = report.clone();
                modified_report.remove(i);
                if is_report_safe(&modified_report) {
                    is_safe = true;
                    break;
                }
            }
        }

        if is_safe {
            safe_reports += 1;
        }

        // println!("{report:?}: {is_safe}");
    }
    println!("{safe_reports}");
}

fn is_report_safe(report: &[i32]) -> bool {
    let mut is_safe = true;
    let is_increasing = report[1] > report[0];

    for i in 0..(report.len() - 1) {
        if !(1..4).contains(&(report[i] - report[i + 1]).abs()) {
            is_safe = false;
        }
        if (report[i + 1] > report[i]) != is_increasing {
            is_safe = false;
        }
    }
    is_safe
}
