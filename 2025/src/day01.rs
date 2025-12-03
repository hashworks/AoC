mod util;

use std::{error::Error, io::BufRead};
use util::{aoc::AoCDay, input::get_reader};

const ID: &str = "day01";
type Input = Vec<isize>;
type Output = usize;

struct Day {}

impl AoCDay<Input, Output> for Day {
    fn parse_input(&self, id: &str) -> Result<Input, Box<dyn Error>> {
        get_reader(id)?
            .lines()
            .into_iter()
            .map(|l| {
                let l = l?;
                let number: isize = l.chars().skip(1).collect::<String>().parse()?;

                match l.chars().next() {
                    Some('L') => Ok(-number),
                    Some('R') => Ok(number),
                    _ => Err("foo")?,
                }
            })
            .collect()
    }

    fn part1(&self, directions: &Input) -> Result<Output, Box<dyn Error>> {
        Ok(directions
            .iter()
            .fold((0, 50), |(counter, pos), dir| {
                let pos = (pos + dir).rem_euclid(100);
                match pos {
                    0 => (counter + 1, pos),
                    _ => (counter, pos),
                }
            })
            .0)
    }

    fn part2(&self, directions: &Input) -> Result<Output, Box<dyn Error>> {
        Ok(directions
            .iter()
            .fold((0, 50), |(counter, pos), &dir| {
                let range = if dir < 0 { dir..0 } else { 0..dir };
                range.fold((counter, pos), |(counter, pos), step| {
                    let pos = if step < 0 { pos - 1 } else { pos + 1 };
                    match pos {
                        -1 => (counter, 99),
                        100 => (counter + 1, 0),
                        0 => (counter + 1, pos),
                        _ => (counter, pos),
                    }
                })
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
            3
        );
    }

    #[test]
    fn test_solve_part2() {
        let day = Day {};
        assert_eq!(
            day.parse_and_solve_part2(format!("{}_test1", ID).as_str())
                .unwrap(),
            6
        );
    }
}
