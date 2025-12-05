pub mod grid;
mod input_manager;
pub mod intcode;

use std::{
    cmp::Ordering,
    ops::{Bound, RangeBounds, RangeInclusive},
};

pub use input_manager::InputManager;

#[must_use]
pub fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 { a.abs() } else { gcd(b, a % b) }
}

#[must_use]
pub fn gcd_i64(a: i64, b: i64) -> i64 {
    if b == 0 { a.abs() } else { gcd_i64(b, a % b) }
}

#[must_use]
pub fn lcm(a: i32, b: i32) -> i32 {
    if a == 0 && b == 0 {
        return 0;
    }
    let gcd = gcd(a, b);
    a.abs() * (b.abs() / gcd)
}

#[must_use]
pub fn lcm_i64(a: i64, b: i64) -> i64 {
    if a == 0 && b == 0 {
        return 0;
    }
    let gcd = gcd_i64(a, b);
    a.abs() * (b.abs() / gcd)
}

pub fn range_reduce<T: Clone + Copy + Ord>(ranges: &[RangeInclusive<T>]) -> Vec<RangeInclusive<T>> {
    let mut ranges = ranges.to_vec();
    ranges.sort_by_key(|range| *range.start());

    for left in 0..(ranges.len() - 1) {
        for right in (left + 1)..(ranges.len()) {
            while right < ranges.len() && ranges[left].contains(ranges[right].start()) {
                ranges[left] =
                    *ranges[left].start()..=*(ranges[left].end().max(ranges[right].end()));
                ranges.remove(right);
            }
        }
    }

    ranges
}

pub trait Year {
    fn day(day: u8) -> Option<DayHandle>;
}

pub trait Day
where
    Self: 'static,
{
    /// Init the struct using the provided input. Benchmark timing starts when this is called. This
    /// is a good place to do calculations that are used for both parts
    fn init(input: &str) -> Box<dyn Day>
    where
        Self: Sized;

    // Return the output for part 1.
    // Note that part 2 may be called before part 1 is, if part 1 ever gets called
    fn part1(&self) -> String {
        "Unimplimented".to_string()
    }

    // Return the output for part 2. Benchmark timing ends after this is called.
    fn part2(&self) -> String {
        "Unimplimented".to_string()
    }

    fn handle() -> DayHandle
    where
        Self: Sized,
    {
        DayHandle::new(Self::init)
    }
}

pub struct DayHandle {
    day_fn: Box<DayFn>,
}

type DayFn = dyn Fn(&str) -> Box<dyn Day>;

impl DayHandle {
    fn new<T: Fn(&str) -> Box<dyn Day> + 'static>(fun: T) -> DayHandle {
        DayHandle {
            day_fn: Box::new(fun),
        }
    }

    pub fn init_day(&self, input: &str) -> Box<dyn Day> {
        (self.day_fn)(input)
    }
}
