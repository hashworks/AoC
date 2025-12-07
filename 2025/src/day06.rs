mod util;

use std::{error::Error, io::BufRead};
use util::{aoc::AoCDay, input::get_reader};

const ID: &str = "day06";
type Input = Vec<(Problem, Vec<Vec<char>>)>;
type Output = usize;

#[derive(Debug, Clone, Copy)]
enum Problem {
    SUM,
    PRODUCT,
}

struct Day {}

impl AoCDay<Input, Output> for Day {
    fn parse_input(&self, id: &str) -> Result<Input, Box<dyn Error>> {
        let lines = get_reader(id)?.lines().collect::<Result<Vec<_>, _>>()?;

        let problem_pos = lines
            .last()
            .ok_or("No problem line found")?
            .chars()
            .rev()
            .enumerate()
            .filter_map(|(pos, c)| match c {
                '+' => Some((pos, Problem::SUM)),
                '*' => Some((pos, Problem::PRODUCT)),
                _ => None,
            })
            .collect::<Vec<_>>();

        let mut previous_pos = 0;
        Ok(problem_pos
            .iter()
            .map(|(problem_pos, problem)| {
                let from = previous_pos;
                let to = problem_pos - previous_pos + 1;
                previous_pos = *problem_pos + 2;
                (
                    *problem,
                    lines
                        .iter()
                        .take(lines.len() - 1)
                        .map(|l| l.chars().rev().skip(from).take(to).collect())
                        .collect(),
                )
            })
            .collect())
    }

    fn part1(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        Ok(input
            .iter()
            .map(|(problem, number_strs)| {
                let numbers = number_strs.iter().map(|n_str| {
                    n_str
                        .iter()
                        .rev()
                        .filter(|c| c.is_digit(10))
                        .collect::<String>()
                        .parse()
                        .unwrap_or(0)
                });
                match problem {
                    &Problem::SUM => numbers.sum::<usize>(),
                    &Problem::PRODUCT => numbers.product(),
                }
            })
            .sum())
    }

    fn part2(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        Ok(input
            .iter()
            .map(|(problem, number_strs)| {
                let len = number_strs.first().map(|s| s.len()).unwrap_or(0);
                let numbers = (0..len).map(|i| {
                    number_strs
                        .iter()
                        .map(move |s| s.get(i).unwrap_or(&' '))
                        .filter(|c| c.is_digit(10))
                        .collect::<String>()
                        .parse()
                        .unwrap_or(0)
                });
                match problem {
                    &Problem::SUM => numbers.sum::<usize>(),
                    &Problem::PRODUCT => numbers.product(),
                }
            })
            .sum())
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
            4277556
        );
    }

    #[test]
    fn test_solve_part2() {
        let day = Day {};
        assert_eq!(
            day.parse_and_solve_part2(format!("{}_test1", ID).as_str())
                .unwrap(),
            3263827
        );
    }
}
