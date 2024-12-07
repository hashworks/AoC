mod util;

use std::{error::Error, io::BufRead};
use util::{aoc::AoCDay, input::get_reader};

const ID: &str = "day07";
type Input = Vec<(usize, Vec<usize>)>;
type Output = usize;

struct Day {}

impl AoCDay<Input, Output> for Day {
    fn parse_input(&self, id: &str) -> Result<Input, Box<dyn Error>> {
        get_reader(id)?
            .lines()
            .map_while(Result::ok)
            .map(|l| {
                let (total_s, parts_s) = l.split_once(": ").ok_or("Invalid input")?;

                let total = total_s.parse::<usize>()?;

                let parts = parts_s
                    .split(' ')
                    .map(|p| p.parse::<usize>())
                    .collect::<Result<Vec<_>, _>>()?;

                Ok((total, parts))
            })
            .collect()
    }

    fn part1(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        Ok(input
            .iter()
            .filter(|(_, parts)| !parts.is_empty())
            .filter(|(total, parts)| {
                parts
                    .iter()
                    .skip(1)
                    .fold(
                        vec![*parts.first().expect("Bug in quantum realm")],
                        |previous, part| {
                            let mut acc = Vec::with_capacity(previous.len() * 2);
                            previous.iter().filter(|&p| p <= total).for_each(|&p| {
                                acc.push(p + part);
                                acc.push(p * part);
                            });
                            acc
                        },
                    )
                    .iter()
                    .any(|p| p == total)
            })
            .map(|(total, _)| total)
            .sum())
    }

    fn part2(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        Ok(input
            .iter()
            .filter(|(_, parts)| !parts.is_empty())
            .filter(|(total, parts)| {
                parts
                    .iter()
                    .skip(1)
                    .fold(
                        vec![*parts.first().expect("Bug in quantum realm")],
                        |previous, part| {
                            let mut acc = Vec::with_capacity(previous.len() * 3);
                            previous.iter().filter(|&p| p <= total).for_each(|&p| {
                                acc.push(p + part);
                                acc.push(p * part);
                                acc.push(concat(p, *part));
                            });
                            acc
                        },
                    )
                    .iter()
                    .any(|p| p == total)
            })
            .map(|(total, _)| total)
            .sum())
    }
}

fn concat(a: usize, b: usize) -> usize {
    let shift = (1..)
        .map(|i| 10_usize.pow(i))
        .find(|&i| i > b)
        .expect("Integer overflow");
    a * shift + b
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
            3749
        );
    }

    #[test]
    fn test_solve_part2() {
        let day = Day {};
        assert_eq!(
            day.parse_and_solve_part2(format!("{}_test1", ID).as_str())
                .unwrap(),
            11387
        );
    }
}
