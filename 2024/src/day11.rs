mod util;

use std::{collections::HashMap, error::Error, io::BufRead};
use util::{aoc::AoCDay, input::get_reader};

const ID: &str = "day11";
type Input = Vec<usize>;
type Output = usize;

fn split_number(n: usize) -> Option<(usize, usize)> {
    let mut num_digits = 0;
    let mut temp = n;
    while temp > 0 {
        num_digits += 1;
        temp /= 10;
    }

    if num_digits % 2 != 0 {
        return None;
    }

    let half_digits = num_digits / 2;
    let divisor = 10usize.pow(half_digits as u32);

    let left_part = n / divisor;
    let right_part = n % divisor;

    Some((left_part, right_part))
}

fn map_number(x: usize) -> (usize, Option<usize>) {
    if x == 0 {
        (1, None)
    } else if let Some((a, b)) = split_number(x) {
        (a, Some(b))
    } else {
        (x * 2024, None)
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
    if let Some(known_len) = seen.get(&key) {
        return *known_len;
    }

    let (a, b) = map_number(number);
    let len = len_after_blinks(a, blinks_left - 1, seen)
        + b.map_or(0, |x| len_after_blinks(x, blinks_left - 1, seen));

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
