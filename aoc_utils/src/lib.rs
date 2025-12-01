mod input_manager;
pub mod intcode;

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

pub trait Year {
    fn day(day: u8) -> Option<DayHandle>;
}

pub trait Day
where
    Self: 'static,
{
    /// Init the struct using the provided input. Benchmark timing starts when this is called. This
    /// is a good place to do calculations that are used for both parts
    fn init(input: String) -> Box<dyn Day>
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
    day_fn: Box<dyn Fn(String) -> Box<dyn Day>>,
}

impl DayHandle {
    fn new<T: Fn(String) -> Box<dyn Day> + 'static>(fun: T) -> DayHandle {
        DayHandle {
            day_fn: Box::new(fun),
        }
    }

    pub fn init_day(&self, input: String) -> Box<dyn Day> {
        (self.day_fn)(input)
    }
}
