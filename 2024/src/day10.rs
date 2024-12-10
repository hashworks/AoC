mod util;

use std::{
    collections::{HashMap, HashSet},
    error::Error,
    io::BufRead,
};
use util::{aoc::AoCDay, input::get_reader};

const ID: &str = "day10";
type Input = Vec<Vec<u32>>;
type Output = usize;

fn dynamic_part2(
    map: &Input,
    position: (usize, usize),
    dynamic: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if let Some(result) = dynamic.get(&position) {
        return *result;
    }

    let current_height = map[position.0][position.1];

    let result = [(1, 0), (0, 1), (-1, 0), (0, -1)]
        .iter()
        .map(|(dy, dx)| {
            let new_position = (position.0 as isize + dy, position.1 as isize + dx);

            if new_position.0 < 0 || new_position.1 < 0 {
                return 0;
            }

            let new_position = (new_position.0 as usize, new_position.1 as usize);

            if new_position.0 >= map.len() || new_position.1 >= map[0].len() {
                return 0;
            }

            let new_height = map[new_position.0][new_position.1];

            if new_height == current_height + 1 {
                if new_height == 9 {
                    return 1;
                }
                dynamic_part2(map, new_position, dynamic)
            } else {
                0
            }
        })
        .sum();

    dynamic.insert(position, result);

    result
}

fn recursive_part1(map: &Input, position: (usize, usize)) -> Vec<(usize, usize)> {
    let current_height = map[position.0][position.1];

    [(1, 0), (0, 1), (-1, 0), (0, -1)]
        .iter()
        .filter_map(|(dy, dx)| {
            let new_position = (position.0 as isize + dy, position.1 as isize + dx);

            if new_position.0 < 0 || new_position.1 < 0 {
                return None;
            }

            let new_position = (new_position.0 as usize, new_position.1 as usize);

            if new_position.0 >= map.len() || new_position.1 >= map[0].len() {
                return None;
            }

            let new_height = map[new_position.0][new_position.1];

            if new_height == current_height + 1 {
                if new_height == 9 {
                    return Some(vec![(new_position.0, new_position.1)]);
                }
                Some(recursive_part1(map, new_position))
            } else {
                None
            }
        })
        .flatten()
        .collect::<Vec<_>>()
}

struct Day {}

impl AoCDay<Input, Output> for Day {
    fn parse_input(&self, id: &str) -> Result<Input, Box<dyn Error>> {
        get_reader(id)?
            .lines()
            .map(|line| {
                line?
                    .chars()
                    .map(|x| x.to_digit(10).ok_or("Invalid input"))
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(|e| e.into())
            })
            .collect()
    }

    fn part1(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        Ok(input
            .iter()
            .enumerate()
            .map(|(y, xs)| {
                xs.iter()
                    .enumerate()
                    .filter(|(_, &h)| h == 0)
                    .map(|(x, _)| {
                        recursive_part1(input, (y, x))
                            .iter()
                            .collect::<HashSet<_>>()
                            .len()
                    })
                    .sum::<usize>()
            })
            .sum())
    }

    fn part2(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        Ok(input
            .iter()
            .enumerate()
            .map(|(y, xs)| {
                xs.iter()
                    .enumerate()
                    .filter(|(_, &h)| h == 0)
                    .map(|(x, _)| dynamic_part2(input, (y, x), &mut HashMap::new()))
                    .sum::<usize>()
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
            36
        );
    }

    #[test]
    fn test_solve_part2() {
        let day = Day {};
        assert_eq!(
            day.parse_and_solve_part2(format!("{}_test1", ID).as_str())
                .unwrap(),
            81
        );
    }
}
