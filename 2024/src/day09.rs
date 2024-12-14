mod util;

use rayon::iter::ParallelBridge;
use std::{error::Error, fmt::Debug, io::BufRead, ops::Range};
use util::{aoc::AoCDay, input::get_reader};

const ID: &str = "day09";

#[derive(Clone, PartialEq)]
enum State {
    Id(usize),
    FreeSpace,
}

impl Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            State::Id(id) => write!(f, "{}", id),
            State::FreeSpace => write!(f, "."),
        }
    }
}

type Input = Vec<State>;
type Output = usize;

struct Day {}

impl AoCDay<Input, Output> for Day {
    fn parse_input(&self, id: &str) -> Result<Input, Box<dyn Error>> {
        let mut free_disk_space = false;
        let mut file_id = 0;
        let mut states = vec![];

        for char in get_reader(id)?
            .lines()
            .next()
            .ok_or("Invalid input")??
            .chars()
        {
            let length = char.to_digit(10).ok_or("Invalid input")? as usize;
            for _ in 0..length {
                if free_disk_space {
                    states.push(State::FreeSpace);
                } else {
                    states.push(State::Id(file_id));
                }
            }
            if free_disk_space {
                file_id += 1;
            }
            free_disk_space = !free_disk_space;
        }

        Ok(states)
    }

    fn part1(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        let mut compacted_disk = input.clone();

        let mut min_j = 0;

        input
            .iter()
            .enumerate()
            .rev()
            .filter(|(_, state)| state != &&State::FreeSpace)
            .for_each(|(i, data)| {
                if i < min_j {
                    return;
                }
                if let Some((j, free_disk_space)) = compacted_disk
                    .iter_mut()
                    .enumerate()
                    .skip(min_j)
                    .take(i - min_j)
                    .find(|(_, s)| s == &&State::FreeSpace)
                {
                    *free_disk_space = data.clone();
                    compacted_disk[i] = State::FreeSpace;
                    min_j = j;
                }
            });

        Ok(checksum(&compacted_disk))
    }

    fn part2(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        let mut compacted_disk = input.clone();

        let mut free_spaces: Vec<Range<usize>> = vec![];
        let mut data_ranges: Vec<(usize, Range<usize>)> = vec![];

        input.iter().enumerate().for_each(|(i, state)| match state {
            State::Id(id) => {
                if let Some(last_data_range) = data_ranges.last_mut() {
                    if last_data_range.0 == *id {
                        last_data_range.1.end = i;
                        return;
                    }
                }
                data_ranges.push((*id, i..i));
            }
            State::FreeSpace => {
                if let Some(last_free_space) = free_spaces.last_mut() {
                    if last_free_space.end == i - 1 {
                        last_free_space.end = i;
                        return;
                    }
                }
                free_spaces.push(i..i);
            }
        });

        data_ranges.iter().rev().for_each(|(id, data_range)| {
            if let Some(free_space) = free_spaces
                .iter_mut()
                .filter(|free_space| free_space.start <= free_space.end)
                .filter(|free_space| free_space.end < data_range.start)
                .find(|free_range| free_range.len() >= data_range.len())
            {
                compacted_disk
                    .iter_mut()
                    .take(free_space.start + data_range.len() + 1)
                    .skip(free_space.start)
                    .for_each(|state| {
                        *state = State::Id(*id);
                    });

                compacted_disk[data_range.start..=data_range.end]
                    .iter_mut()
                    .for_each(|state| {
                        *state = State::FreeSpace;
                    });

                free_space.start += data_range.len() + 1;
            }
        });

        Ok(checksum(&compacted_disk))
    }
}

fn checksum(disk: &[State]) -> usize {
    disk.iter()
        .enumerate()
        .fold(0, |acc, (i, state)| match state {
            State::Id(id) => acc + i * id,
            State::FreeSpace => acc,
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
            1928
        );
    }

    #[test]
    fn test_solve_part2() {
        let day = Day {};
        assert_eq!(
            day.parse_and_solve_part2(format!("{}_test1", ID).as_str())
                .unwrap(),
            2858
        );
    }
}
