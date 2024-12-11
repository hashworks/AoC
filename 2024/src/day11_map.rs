mod util;

use hashbrown::HashMap;
use std::{error::Error, io::BufRead};
use util::{aoc::AoCDay, input::get_reader};

const ID: &str = "day11";
type Input = HashMap<usize, usize>;
type Output = usize;

// Marginally faster than reasonable alternatives. This is just for fun, please don't do this in production.
fn try_to_split_number(n: usize) -> Option<(usize, usize)> {
    if let Some(divisor) = match n {
        0..=9 => None,
        10..=99 => Some(10),
        100..=999 => None,
        1000..=9999 => Some(100),
        10000..=99999 => None,
        100000..=999999 => Some(1_000),
        1000000..=9999999 => None,
        10000000..=99999999 => Some(10_000),
        100000000..=999999999 => None,
        1000000000..=9999999999 => Some(100_000),
        10000000000..=99999999999 => None,
        100000000000..=999999999999 => Some(1_000_000),
        1000000000000..=9999999999999 => None,
        10000000000000..=99999999999999 => Some(10_000_000),
        100000000000000..=999999999999999 => None,
        1000000000000000..=9999999999999999 => Some(100_000_000),
        10000000000000000..=99999999999999999 => None,
        100000000000000000..=999999999999999999 => Some(1_000_000_000),
        1000000000000000000..=9999999999999999999 => None,
        _ => Some(10_000_000_000),
    } {
        let left = n / divisor;
        let right = n % divisor;

        Some((left, right))
    } else {
        None
    }
}

// Idea from AxlLind with some modifications
// https://github.com/AxlLind/AdventOfCode2024/blob/main/src/bin/11.rs
fn blink(stones: &mut HashMap<usize, usize>) {
    stones
        .clone()
        .into_iter()
        .filter(|(_, count)| count > &0)
        .for_each(|(number, count)| {
            stones.entry(number).and_modify(|x| *x -= count);
            match number {
                0 => {
                    *stones.entry(1).or_default() += count;
                }
                _ => match try_to_split_number(number) {
                    Some((a, b)) => {
                        *stones.entry(a).or_default() += count;
                        *stones.entry(b).or_default() += count;
                    }
                    None => {
                        *stones.entry(number * 2024).or_default() += count;
                    }
                },
            }
        });
}

struct Day {}

impl AoCDay<Input, Output> for Day {
    fn parse_input(&self, id: &str) -> Result<Input, Box<dyn Error>> {
        get_reader(id)?
            .lines()
            .next()
            .ok_or("Invalid input")??
            .split(' ')
            .map(|x| Ok((x.parse()?, 1)))
            .collect()
    }

    // ~215µs
    fn part1(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        let mut stones = input.clone();
        for _ in 0..25 {
            blink(&mut stones);
        }
        Ok(stones.values().sum())
    }

    // ~2.3ms
    fn part2(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        let mut stones = input.clone();
        for _ in 0..75 {
            blink(&mut stones);
        }
        Ok(stones.values().sum())
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
            55312
        );
    }

    #[test]
    fn test_solve_part2() {
        let day = Day {};
        assert_eq!(
            day.parse_and_solve_part2(format!("{}_test1", ID).as_str())
                .unwrap(),
            65601038650482
        );
    }
}
