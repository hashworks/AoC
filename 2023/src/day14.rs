mod util;

use pathfinding::directed::cycle_detection::floyd;
use std::{error::Error, io::BufRead};
use util::{aoc::AoCDay, input::get_reader};

const ID: &str = "day14";
type Input = Vec<Vec<Option<State>>>;
type Output = usize;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum State {
    Ball,
    Rock,
}

struct Day {}

impl AoCDay<Input, Output> for Day {
    fn parse_input(&self, id: &str) -> Result<Input, Box<dyn Error>> {
        get_reader(id)?
            .lines()
            .map(|l| {
                let l = l?;
                l.chars()
                    .map(|c| match c {
                        '.' => Ok(None),
                        '#' => Ok(Some(State::Rock)),
                        'O' => Ok(Some(State::Ball)),
                        _ => Err("Invalid character".into()),
                    })
                    .collect()
            })
            .collect()
    }

    fn part1(&self, map: &Input) -> Result<Output, Box<dyn Error>> {
        Ok(calculate_load(tilt_north(&mut map.clone())))
    }

    fn part2(&self, map: &Input) -> Result<Output, Box<dyn Error>> {
        let (cycle_size, map, index_of_first_element) = floyd(map.clone(), cycle);

        let remaining_cycles = (1000000000 - index_of_first_element) % cycle_size;

        Ok(calculate_load(
            &(0..remaining_cycles).fold(map.clone(), |map, _| cycle(map)),
        ))
    }
}

fn cycle(map: Input) -> Input {
    let mut map = map.clone();

    map = tilt_north(&mut map).to_vec();
    map = tilt_west(&mut map).to_vec();
    map = tilt_south(&mut map).to_vec();
    map = tilt_east(&mut map).to_vec();

    map
}

fn calculate_load(map: &Input) -> usize {
    map.iter()
        .map(|column| column.iter().filter(|c| c == &&Some(State::Ball)).count())
        .enumerate()
        .map(|(y, balls)| (map.len() - y) * balls)
        .sum()
}

fn tilt_north(map: &mut Input) -> &Input {
    for y in 1..map.len() {
        for x in 0..map[0].len() {
            if let Some(State::Ball) = map[y][x] {
                let target_y = (0..y)
                    .rev()
                    .find(|y| map[*y][x].is_some())
                    .map(|y| y + 1)
                    .unwrap_or(0);
                if target_y != y {
                    map[y][x] = None;
                    map[target_y][x] = Some(State::Ball);
                }
            }
        }
    }

    map
}

fn tilt_west(map: &mut Input) -> &Input {
    for y in 0..map.len() {
        for x in 1..map[0].len() {
            if let Some(State::Ball) = map[y][x] {
                let target_x = (0..x)
                    .rev()
                    .find(|x| map[y][*x].is_some())
                    .map(|x| x + 1)
                    .unwrap_or(0);
                if target_x != x {
                    map[y][x] = None;
                    map[y][target_x] = Some(State::Ball);
                }
            }
        }
    }

    map
}

fn tilt_south(map: &mut Input) -> &Input {
    for y in (0..map.len() - 1).rev() {
        for x in 0..map[0].len() {
            if let Some(State::Ball) = map[y][x] {
                let target_y = (y + 1..map.len())
                    .find(|y| map[*y][x].is_some())
                    .map(|y| y - 1)
                    .unwrap_or(map.len() - 1);
                if target_y != y {
                    map[y][x] = None;
                    map[target_y][x] = Some(State::Ball);
                }
            }
        }
    }

    map
}

fn tilt_east(map: &mut Input) -> &Input {
    let max_x = map[0].len() - 1;
    for y in 0..map.len() {
        for x in (0..max_x).rev() {
            if let Some(State::Ball) = map[y][x] {
                let target_x = (x + 1..map[0].len())
                    .find(|x| map[y][*x].is_some())
                    .map(|x| x - 1)
                    .unwrap_or(max_x);
                if target_x != x {
                    map[y][x] = None;
                    map[y][target_x] = Some(State::Ball);
                }
            }
        }
    }

    map
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
            136
        );
    }

    #[test]
    fn test_solve_part2() {
        let day = Day {};
        assert_eq!(
            day.parse_and_solve_part2(format!("{}_test1", ID).as_str())
                .unwrap(),
            64
        );
    }
}
