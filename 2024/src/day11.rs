mod util;

use std::{collections::HashMap, error::Error, io::BufRead};
use util::{aoc::AoCDay, input::get_reader};

const ID: &str = "day11";
type Input = Vec<usize>;
type Output = usize;

// Marginally faster than reasonable alternatives. This is just for fun, please don't do this in production.
fn split_number(n: usize) -> Option<(usize, usize)> {
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

fn map_number(x: usize) -> (usize, Option<usize>) {
    if x == 0 {
        return (1, None);
    }

    match split_number(x) {
        Some((a, b)) => (a, Some(b)),
        None => (x * 2024, None),
    }
}

fn len_after_blinks(
    number: usize,
    blinks_left: usize,
    seen: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if blinks_left == 0 {
        return 1;
    }

    let key = (number, blinks_left);
    if let Some(&known_len) = seen.get(&key) {
        return known_len;
    }

    let (a, b_opt) = map_number(number);
    let len = len_after_blinks(a, blinks_left - 1, seen)
        + b_opt.map_or(0, |b| len_after_blinks(b, blinks_left - 1, seen));

    seen.insert(key, len);
    len
}

struct Day {}

impl AoCDay<Input, Output> for Day {
    fn parse_input(&self, id: &str) -> Result<Input, Box<dyn Error>> {
        get_reader(id)?
            .lines()
            .next()
            .ok_or("Invalid input")??
            .split(' ')
            .map(|x| x.parse().map_err(Into::into))
            .collect()
    }

    fn part1(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        let seen = &mut HashMap::new();
        Ok(input.iter().map(|x| len_after_blinks(*x, 25, seen)).sum())
    }

    fn part2(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        let seen = &mut HashMap::new();
        Ok(input.iter().map(|x| len_after_blinks(*x, 75, seen)).sum())
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
