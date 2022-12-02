mod util;

use std::error::Error;
use util::{aoc::AoCDay, input::get_reader};

const ID: &str = "day00";
type Input = usize;
type Output = usize;

struct Day {}

impl AoCDay<Input, Output> for Day {
    fn parse_input(&self, _id: &str) -> Result<Input, Box<dyn Error>> {
        let _reader = get_reader(_id)?;

        Ok(42)
    }

    fn part1(&self, input: &Input) -> Output {
        *input
    }

    fn part2(&self, input: &Input) -> Output {
        *input
    }
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

fn main() {
    Day {}.run(ID);
}
