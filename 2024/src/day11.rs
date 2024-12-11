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

fn map_number(x: usize) -> Vec<usize> {
    if x == 0 {
        vec![1]
    } else if let Some((a, b)) = split_number(x) {
        vec![a, b]
    } else {
        vec![x * 2024]
    }
}

fn len_after_blinks(
    numbers: Vec<usize>,
    blinks_left: usize,
    seen: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if blinks_left == 0 {
        return numbers.len();
    }

    numbers
        .iter()
        .map(|x| {
            if let Some(known_len) = seen.get(&(*x, blinks_left)) {
                return *known_len;
            }

            let len = len_after_blinks(map_number(*x), blinks_left - 1, seen);

            seen.insert((*x, blinks_left), len);

            len
        })
        .sum()
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
        Ok(len_after_blinks(input.clone(), 25, &mut HashMap::new()))
    }

    fn part2(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        Ok(len_after_blinks(input.clone(), 75, &mut HashMap::new()))
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
