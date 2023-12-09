mod util;

use std::{error::Error, io::BufRead};
use util::{aoc::AoCDay, input::get_reader};

const ID: &str = "day09";
type Input = Vec<Vec<isize>>;
type Output = isize;

struct Day {}

impl AoCDay<Input, Output> for Day {
    fn parse_input(&self, id: &str) -> Result<Input, Box<dyn Error>> {
        let reader = get_reader(id)?;

        reader
            .lines()
            .map(|line| {
                let line = line?;
                let numbers = line
                    .split_ascii_whitespace()
                    .map(|number_str| number_str.parse::<isize>())
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(numbers)
            })
            .collect::<Result<Input, Box<dyn Error>>>()
    }

    fn part1(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        Ok(input
            .iter()
            .map(|numbers| {
                get_differences(numbers)
                    .iter()
                    .rev()
                    .fold(0, |acc, numbers| acc + numbers.last().unwrap_or(&0))
            })
            .sum())
    }

    fn part2(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        Ok(input
            .iter()
            .map(|numbers| {
                get_differences(numbers)
                    .iter()
                    .rev()
                    .fold(0, |acc, numbers| numbers.first().unwrap_or(&0) - acc)
            })
            .sum())
    }
}

fn get_differences(numbers: &[isize]) -> Vec<Vec<isize>> {
    let mut differences: Vec<Vec<isize>> = vec![numbers.to_vec()];

    while differences.last().unwrap().iter().any(|n| *n != 0) {
        differences.push(
            differences
                .last()
                .unwrap()
                .windows(2)
                .map(|chunk| chunk[1] - chunk[0])
                .collect(),
        );
    }

    differences
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
            114
        );
    }

    #[test]
    fn test_solve_part2() {
        let day = Day {};
        assert_eq!(
            day.parse_and_solve_part2(format!("{}_test1", ID).as_str())
                .unwrap(),
            2
        );
    }
}
