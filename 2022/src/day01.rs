mod util;

use std::{error::Error, io::BufRead};
use util::{aoc::AoCDay, input::get_reader};

const ID: &str = "day01";
type Input = Vec<Vec<usize>>;
type Output = usize;

struct Day {}

impl AoCDay<Input, Output> for Day {
    fn parse_input(&self, id: &str) -> Result<Input, Box<dyn Error>> {
        let reader = get_reader(id)?;

        let mut result = Vec::new();
        let mut temp = Vec::new();

        for line in reader.lines() {
            let line = line?;
            if line.is_empty() {
                result.push(temp);
                temp = Vec::new();
            } else {
                temp.push(line.parse()?);
            }
        }
        if !temp.is_empty() {
            result.push(temp);
        }

        Ok(result)
    }

    fn part1(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        input
            .iter()
            .map(|v| v.iter().sum())
            .max()
            .ok_or("no max".into())
    }

    fn part2(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        let mut vec: Vec<_> = input.iter().map(|v| v.iter().sum()).collect();
        vec.sort();

        Ok(vec.iter().rev().take(3).sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        assert_eq!(
            Day {}
                .parse_and_solve_part1(format!("{}_test1", ID).as_str())
                .unwrap(),
            24000
        );
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(
            Day {}
                .parse_and_solve_part2(format!("{}_test1", ID).as_str())
                .unwrap(),
            45000
        );
    }
}

fn main() {
    Day {}.run(ID);
}
