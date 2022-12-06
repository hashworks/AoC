mod util;

use std::{collections::HashSet, error::Error, io::BufRead};
use util::{aoc::AoCDay, input::get_reader};

const ID: &str = "day06";
type Input = Vec<u8>;
type Output = usize;

struct Day {}

impl AoCDay<Input, Output> for Day {
    fn parse_input(&self, id: &str) -> Result<Input, Box<dyn Error>> {
        let reader = get_reader(id)?;

        Ok(reader
            .lines()
            .next()
            .ok_or("bad input: no first line")??
            .bytes()
            .collect())
    }

    fn part1(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        Ok(first_distinct_index(input, 4))
    }

    fn part2(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        Ok(first_distinct_index(input, 14))
    }
}

fn first_distinct_index(input: &[u8], windowsize: usize) -> usize {
    input
        .windows(windowsize)
        .enumerate()
        .find(|(_, window)| window.iter().collect::<HashSet<&u8>>().len() == windowsize)
        .map(|(idx, _)| idx + windowsize)
        .unwrap()
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
            7
        );
    }

    #[test]
    fn test_solve_part2() {
        let day = Day {};
        assert_eq!(
            day.parse_and_solve_part2(format!("{}_test1", ID).as_str())
                .unwrap(),
            19
        );
    }
}

fn main() {
    Day {}.run(ID);
}
