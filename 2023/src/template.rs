mod util;

use std::error::Error;
use util::{aoc::AoCDay, input::get_reader};

const ID: &str = "day00";
type Input = usize;
type Output = usize;

struct Day {}

impl AoCDay<Input, Output> for Day {
    fn parse_input(&self, id: &str) -> Result<Input, Box<dyn Error>> {
        let reader = get_reader(id)?;

        Ok(42)
    }

    fn part1(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        Ok(*input)
    }

    fn part2(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        Ok(*input)
    }
}

fn main() {
    Day {}.run(ID);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        let day = Day {};
        assert_eq!(
            day.parse_and_solve_part1(format!("{}_test1", ID).as_str())
                .unwrap(),
            42
        );
    }

    #[test]
    fn test_solve_part2() {
        let day = Day {};
        assert_eq!(
            day.parse_and_solve_part2(format!("{}_test1", ID).as_str())
                .unwrap(),
            42
        );
    }
}
