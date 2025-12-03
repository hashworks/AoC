mod util;

use std::{error::Error, io::BufRead};
use util::{aoc::AoCDay, input::get_reader};

const ID: &str = "day03";
type Input = Vec<Vec<u32>>;
type Output = usize;

struct Day {}

impl AoCDay<Input, Output> for Day {
    fn parse_input(&self, id: &str) -> Result<Input, Box<dyn Error>> {
        get_reader(id)?
            .lines()
            .into_iter()
            .map(|l| {
                l?.chars()
                    .map(|c| {
                        c.to_digit(10).ok_or_else(|| {
                            Box::<dyn Error>::from("Input char is not a valid digit")
                        })
                    })
                    .collect()
            })
            .collect()
    }

    fn part1(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        Ok(joltage_maximizer(input, 2))
    }

    fn part2(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        Ok(joltage_maximizer(input, 12))
    }
}

fn joltage_maximizer(banks: &Input, digits: usize) -> Output {
    banks
        .iter()
        .map(|bank| {
            let max_skips = bank.len() - (digits - 1);
            (0..digits)
                .fold((0, 0), |(bank_sum, skips), pos| {
                    let mut pos_max_i = 0;
                    let mut pos_max = 0;
                    for (current_pos_n_i, pos_n) in bank
                        .iter()
                        .skip(pos + skips)
                        .take(max_skips - skips)
                        .enumerate()
                    {
                        if *pos_n > pos_max {
                            pos_max_i = current_pos_n_i;
                            pos_max = *pos_n;
                        }
                    }

                    (
                        bank_sum + 10usize.pow(digits as u32 - 1 - pos as u32) * pos_max as usize,
                        skips + pos_max_i,
                    )
                })
                .0
        })
        .sum()
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
            357
        );
    }

    #[test]
    fn test_solve_part2() {
        let day = Day {};
        assert_eq!(
            day.parse_and_solve_part2(format!("{}_test1", ID).as_str())
                .unwrap(),
            3121910778619
        );
    }
}
