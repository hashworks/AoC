mod util;

use std::{error::Error, io::BufRead};
use util::{aoc::AoCDay, input::get_reader};

const ID: &str = "day04";
type Input = Vec<Vec<char>>;
type Output = usize;

struct Day {}

impl AoCDay<Input, Output> for Day {
    fn parse_input(&self, id: &str) -> Result<Input, Box<dyn Error>> {
        get_reader(id)?
            .lines()
            .map(|s| Ok(s?.chars().collect()))
            .collect()
    }

    fn part1(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        Ok([
            input,
            &transpose_vertical(input),
            &transpose_diagonal_left(input),
            &transpose_diagonal_right(input),
        ]
        .iter()
        .map(|variant| {
            variant.iter().fold(0, |acc, line| {
                acc + line
                    .windows(4)
                    .filter(|w| w == &['X', 'M', 'A', 'S'] || w == &['S', 'A', 'M', 'X'])
                    .count()
            })
        })
        .sum::<usize>())
    }

    fn part2(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        Ok((1..input.len() - 1)
            .flat_map(|i| {
                (1..input[0].len() - 1)
                    .map(|j| {
                        if input[i][j] == 'A'
                            && (input[i - 1][j - 1] == 'M' && input[i + 1][j + 1] == 'S'
                                || input[i - 1][j - 1] == 'S' && input[i + 1][j + 1] == 'M')
                            && (input[i - 1][j + 1] == 'M' && input[i + 1][j - 1] == 'S'
                                || input[i - 1][j + 1] == 'S' && input[i + 1][j - 1] == 'M')
                        {
                            Some(())
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .flatten()
            .count())
    }
}

fn transpose_vertical(input: &Input) -> Vec<Vec<char>> {
    (0..input[0].len())
        .map(|i| (0..input.len()).map(|j| input[j][i]).collect())
        .collect()
}

fn transpose_diagonal_left(input: &Input) -> Vec<Vec<char>> {
    let mut result = vec![vec![]; input.len() + input[0].len() - 1];

    for i in 0..input.len() {
        for j in 0..input[0].len() {
            result[i + j].push(input[i][j]);
        }
    }

    result
}

fn transpose_diagonal_right(input: &Input) -> Vec<Vec<char>> {
    let mut result = vec![vec![]; input.len() + input[0].len() - 1];

    for i in 0..input.len() {
        for j in 0..input[0].len() {
            result[i + input[0].len() - j - 1].push(input[i][j]);
        }
    }

    result
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
            18
        );
    }

    #[test]
    fn test_solve_part2() {
        let day = Day {};
        assert_eq!(
            day.parse_and_solve_part2(format!("{}_test1", ID).as_str())
                .unwrap(),
            9
        );
    }
}
