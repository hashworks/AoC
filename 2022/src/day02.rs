mod util;

use std::{error::Error, io::BufRead};
use util::{aoc::AoCDay, input::get_reader};

const ID: &str = "day02";
type Input = Vec<(char, char)>;
type Output = usize;

const OPPONENT_ROCK: char = 'A';
const OPPONENT_PAPER: char = 'B';
const OPPONENT_SCISSORS: char = 'C';

const PART1_ROCK: char = 'X';
const PART1_PAPER: char = 'Y';
const PART1_SCISSORS: char = 'Z';

const PART2_LOOSE: char = 'X';
const PART2_DRAW: char = 'Y';
const PART2_WIN: char = 'Z';

struct Day {}

impl AoCDay<Input, Output> for Day {
    fn parse_input(&self, id: &str) -> Result<Input, Box<dyn Error>> {
        get_reader(id)?
            .lines()
            .map(|l| {
                let line = l?;
                let mut chars = line.chars();
                let left = chars.next();
                chars.next();
                let right = chars.next();
                if left.is_none() || right.is_none() {
                    return Err("Invalid input".into());
                }
                Ok((left.unwrap(), right.unwrap()))
            })
            .collect()
    }

    fn part1(&self, input: &Input) -> Output {
        input
            .iter()
            .map(|(left, right)| match (*right, *left) {
                (PART1_ROCK, OPPONENT_SCISSORS) => 1 + 6,
                (PART1_PAPER, OPPONENT_ROCK) => 2 + 6,
                (PART1_SCISSORS, OPPONENT_PAPER) => 3 + 6,
                (PART1_ROCK, OPPONENT_ROCK) => 1 + 3,
                (PART1_PAPER, OPPONENT_PAPER) => 2 + 3,
                (PART1_SCISSORS, OPPONENT_SCISSORS) => 3 + 3,
                (PART1_ROCK, OPPONENT_PAPER) => 1,
                (PART1_PAPER, OPPONENT_SCISSORS) => 2,
                (PART1_SCISSORS, OPPONENT_ROCK) => 3,
                _ => 0,
            })
            .sum()
    }

    fn part2(&self, input: &Input) -> Output {
        input
            .iter()
            .map(|(left, right)| match (*right, *left) {
                (PART2_WIN, OPPONENT_SCISSORS) => 1 + 6,
                (PART2_WIN, OPPONENT_ROCK) => 2 + 6,
                (PART2_WIN, OPPONENT_PAPER) => 3 + 6,
                (PART2_DRAW, OPPONENT_ROCK) => 1 + 3,
                (PART2_DRAW, OPPONENT_PAPER) => 2 + 3,
                (PART2_DRAW, OPPONENT_SCISSORS) => 3 + 3,
                (PART2_LOOSE, OPPONENT_PAPER) => 1,
                (PART2_LOOSE, OPPONENT_SCISSORS) => 2,
                (PART2_LOOSE, OPPONENT_ROCK) => 3,
                _ => 0,
            })
            .sum()
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
            15
        );
    }

    #[test]
    fn test_solve_part2() {
        let day = Day {};
        assert_eq!(
            day.parse_and_solve_part2(format!("{}_test1", ID).as_str())
                .unwrap(),
            12
        );
    }
}

fn main() {
    Day {}.run(ID);
}
