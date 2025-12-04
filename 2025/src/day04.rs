mod util;

use std::{collections::HashSet, error::Error, io::BufRead};
use util::{aoc::AoCDay, input::get_reader};

const ID: &str = "day04";
type Input = Vec<Vec<bool>>;
type Output = usize;

struct Day {}

impl AoCDay<Input, Output> for Day {
    fn parse_input(&self, id: &str) -> Result<Input, Box<dyn Error>> {
        get_reader(id)?
            .lines()
            .map(|l| {
                l?.chars()
                    .map(|c| match c {
                        '.' => Ok(false),
                        '@' => Ok(true),
                        _ => Err(Box::<dyn Error>::from("Invalid char")),
                    })
                    .collect()
            })
            .collect()
    }

    fn part1(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        Ok(input
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, is_tp)| **is_tp)
                    .filter(|(x, _)| {
                        let y = y as isize;
                        let x = *x as isize;
                        get_surroundings(y, x)
                            .iter()
                            .filter(|(y, x)| *y >= 0 && *x >= 0)
                            .filter(|(y, x)| match input.get(*y as usize) {
                                Some(row) => *row.get(*x as usize).unwrap_or(&false),
                                None => false,
                            })
                            .take(4)
                            .count()
                            < 4
                    })
                    .count()
            })
            .sum())
    }

    fn part2(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        Ok(part2_rec(input, HashSet::new()))
    }
}

fn get_surroundings(y: isize, x: isize) -> [(isize, isize); 8] {
    [
        (y - 1, x - 1),
        (y - 1, x),
        (y - 1, x + 1),
        (y, x - 1),
        (y, x + 1),
        (y + 1, x - 1),
        (y + 1, x),
        (y + 1, x + 1),
    ]
}

fn part2_rec(input: &Vec<Vec<bool>>, mut removed: HashSet<(usize, usize)>) -> usize {
    match input
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, is_tp)| **is_tp)
                .filter(|(x, _)| {
                    if removed.contains(&(y, *x)) {
                        return false;
                    }
                    let y = y as isize;
                    let x = *x as isize;
                    if get_surroundings(y, x)
                        .iter()
                        .filter(|(y, x)| *y >= 0 && *x >= 0)
                        .filter(|(y, x)| match input.get(*y as usize) {
                            Some(row) => {
                                row.get(*x as usize) == Some(&true)
                                    && !removed.contains(&(*y as usize, *x as usize))
                            }
                            None => false,
                        })
                        .take(4)
                        .count()
                        < 4
                    {
                        removed.insert((y as usize, x as usize));
                        true
                    } else {
                        false
                    }
                })
                .count()
        })
        .sum::<usize>()
    {
        0 => 0,
        x => x + part2_rec(input, removed),
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
            13
        );
    }

    #[test]
    fn test_solve_part2() {
        let day = Day {};
        assert_eq!(
            day.parse_and_solve_part2(format!("{}_test1", ID).as_str())
                .unwrap(),
            43
        );
    }
}
