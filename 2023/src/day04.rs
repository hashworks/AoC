mod util;

use std::{error::Error, io::BufRead};
use util::{aoc::AoCDay, input::get_reader};

const ID: &str = "day04";
type Input = Vec<(usize, usize)>;
type Output = usize;

struct Day {}

impl AoCDay<Input, Output> for Day {
    fn parse_input(&self, id: &str) -> Result<Input, Box<dyn Error>> {
        let reader = get_reader(id)?;

        let mut cards = Vec::new();

        for line in reader.lines() {
            let line = line?;

            let mut parts = line.split(':');

            let mut number_strs = parts
                .nth(1)
                .ok_or("Parse Error: malformed input")?
                .split(" | ");

            let winning_numbers = number_strs
                .next()
                .ok_or("Parse Error: malformed input")?
                .split(' ')
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>();

            let matching_numbers = number_strs
                .next()
                .ok_or("Parse Error: malformed input")?
                .split(' ')
                .filter(|s| winning_numbers.contains(s))
                .count();

            cards.push((1, matching_numbers));
        }

        Ok(cards)
    }

    fn part1(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        Ok(input
            .iter()
            .map(|(_, matching_numbers)| 2_usize.pow(*matching_numbers as u32) / 2)
            .sum())
    }

    fn part2(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        let mut input = input.clone();

        for id in 0..input.len() {
            let (won_copies, matching_numbers) = input[id];
            for (copies, _) in input.iter_mut().skip(id + 1).take(matching_numbers) {
                *copies += won_copies;
            }
        }

        Ok(input.iter().map(|(copies, _)| copies).sum())
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
            13
        );
    }

    #[test]
    fn test_solve_part2() {
        let day = Day {};
        assert_eq!(
            day.parse_and_solve_part2(format!("{}_test1", ID).as_str())
                .unwrap(),
            30
        );
    }
}
