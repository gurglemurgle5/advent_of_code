use aoc_utils::{DayHandle, Year};

include! { concat!(env!("OUT_DIR"), "/generated.rs") }

pub fn get_day(year: u16, day: u8) -> Option<DayHandle> {
    match year {
        2015 => year2015::Year2015::day(day),
        2025 => year2025::Year2025::day(day),
        _ => None,
    }
}
