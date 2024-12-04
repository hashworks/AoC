mod util;

use regex::Regex;
use std::{error::Error, io::BufRead};
use util::{aoc::AoCDay, input::get_reader};

const ID: &str = "day03";
type Input = Vec<String>;
type Output = usize;

struct Day {}

impl AoCDay<Input, Output> for Day {
    fn parse_input(&self, id: &str) -> Result<Input, Box<dyn Error>> {
        get_reader(id)?.lines().map(|line| Ok(line?)).collect()
    }

    fn part1(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)")?;

        Ok(input
            .iter()
            .map(|line| {
                re.captures_iter(line)
                    .flat_map(|cap| {
                        let (xs, ys) = (cap.get(1)?, cap.get(2)?);
                        let x = xs.as_str().parse::<usize>().ok()?;
                        let y = ys.as_str().parse::<usize>().ok()?;
                        Some(x * y)
                    })
                    .sum::<usize>()
            })
            .sum())
    }

    fn part2(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        let re = Regex::new(r"(?mU)(.*)mul\(([0-9]{1,3}),([0-9]{1,3})\)")?;

        Ok(re
            .captures_iter(&input.join("\n"))
            .fold((0, true), |(acc, enabled), cap| {
                if let (Some(control), Some(xs), Some(ys)) = (cap.get(1), cap.get(2), cap.get(3)) {
                    // This only works because some edge cases don't appear in the input
                    let enabled = if enabled && control.as_str().contains("don't()") {
                        false
                    } else if !enabled && control.as_str().contains("do()") {
                        true
                    } else {
                        enabled
                    };
                    if enabled {
                        if let (Some(x), Some(y)) = (
                            xs.as_str().parse::<usize>().ok(),
                            ys.as_str().parse::<usize>().ok(),
                        ) {
                            (acc + x * y, enabled)
                        } else {
                            (acc, enabled)
                        }
                    } else {
                        (acc, enabled)
                    }
                } else {
                    (acc, enabled)
                }
            })
            .0)
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
            161
        );
    }

    #[test]
    fn test_solve_part2() {
        let day = Day {};
        assert_eq!(
            day.parse_and_solve_part2(format!("{}_test2", ID).as_str())
                .unwrap(),
            48
        );
    }
}
