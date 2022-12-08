mod util;

use std::{error::Error, io::BufRead};
use take_until::TakeUntilExt;
use util::{aoc::AoCDay, input::get_reader};

const ID: &str = "day08";
type Input = Vec<Vec<u8>>;
type Output = usize;

struct Day {}

impl AoCDay<Input, Output> for Day {
    fn parse_input(&self, id: &str) -> Result<Input, Box<dyn Error>> {
        get_reader(id)?
            .lines()
            .map(|line| Ok(line?.into_bytes()))
            .collect()
    }

    fn part1(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        Ok(input
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter(|&(x, &tree)| {
                        row[0..x].iter().all(|&left_tree| left_tree < tree)
                            || row[x + 1..row.len()]
                                .iter()
                                .all(|&right_tree| right_tree < tree)
                            || input[0..y]
                                .iter()
                                .rev()
                                .map(|row| row[x])
                                .all(|top_tree| top_tree < tree)
                            || input[y + 1..input.len()]
                                .iter()
                                .map(|row| row[x])
                                .all(|bottom_tree| bottom_tree < tree)
                    })
                    .count()
            })
            .sum())
    }

    fn part2(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        input
            .iter()
            .enumerate()
            .skip(1)
            .take(input.len() - 1)
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .skip(1)
                    .take(row.len() - 1)
                    .map(|(x, &tree)| {
                        row[0..x]
                            .iter()
                            .rev()
                            .take_until(|&&left_tree| left_tree >= tree)
                            .count()
                            * row[x + 1..row.len()]
                                .iter()
                                .take_until(|&&left_tree| left_tree >= tree)
                                .count()
                            * input[0..y]
                                .iter()
                                .rev()
                                .map(|row| row[x])
                                .take_until(|&top_tree| top_tree >= tree)
                                .count()
                            * input[y + 1..input.len()]
                                .iter()
                                .map(|row| row[x])
                                .take_until(|&bottom_tree| bottom_tree >= tree)
                                .count()
                    })
                    .max()
                    .unwrap_or(0)
            })
            .max()
            .ok_or("bad input: no max-vis tree found".into())
    }
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
            21
        );
    }

    #[test]
    fn test_solve_part2() {
        let day = Day {};
        assert_eq!(
            day.parse_and_solve_part2(format!("{}_test1", ID).as_str())
                .unwrap(),
            8
        );
    }
}

fn main() {
    Day {}.run(ID);
}
