mod util;

use std::{error::Error, io::BufRead};
use util::{aoc::AoCDay, input::get_reader};

const ID: &str = "day02";
type Input = Vec<Vec<isize>>;
type Output = usize;

struct Day {}

impl AoCDay<Input, Output> for Day {
    fn parse_input(&self, id: &str) -> Result<Input, Box<dyn Error>> {
        get_reader(id)?
            .lines()
            .map(|line| {
                line?
                    .split_whitespace()
                    .map(|x| x.parse::<isize>())
                    .collect::<Result<_, _>>()
                    .map_err(|e| e.into())
            })
            .collect()
    }

    fn part1(&self, reports: &Input) -> Result<Output, Box<dyn Error>> {
        Ok(reports.iter().filter(|report| check_report(report)).count())
    }

    fn part2(&self, reports: &Input) -> Result<Output, Box<dyn Error>> {
        Ok(reports
            .iter()
            .filter(|report| {
                (0..=report.len())
                    .map(|i| {
                        report
                            .iter()
                            .enumerate()
                            .filter(move |(j, _)| i != *j)
                            .map(|(_, x)| *x)
                    })
                    .any(|report| check_report(&report.collect::<Vec<_>>()))
            })
            .count())
    }
}

fn check_report(report: &[isize]) -> bool {
    (report.windows(2).all(|w| w[0] < w[1]) || report.windows(2).all(|w| w[0] > w[1]))
        && report.windows(2).all(|w| {
            let diff = (w[1] - w[0]).abs();
            (1..=3).contains(&diff)
        })
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
            2
        );
    }

    #[test]
    fn test_solve_part2() {
        let day = Day {};
        assert_eq!(
            day.parse_and_solve_part2(format!("{}_test1", ID).as_str())
                .unwrap(),
            4
        );
    }
}
