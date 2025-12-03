use aoc_utils::Day;

pub struct Day_(String);

impl Day for Day_ {
    fn init(input: &str) -> Box<dyn Day>
    where
        Self: Sized,
    {
        Box::new(Self(input.into()))
    }

    fn part1(&self) -> String {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(Day_::init("TODO").part1(), "TODO");
    }
}
