mod util;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::{collections::HashSet, error::Error, hash::Hash, io::BufRead};
use util::{aoc::AoCDay, input::get_reader};

const ID: &str = "day06";

#[derive(PartialEq, Eq, Hash)]
enum Direction {
    N,
    E,
    S,
    W,
}

impl Clone for Direction {
    fn clone(&self) -> Self {
        match self {
            Direction::N => Direction::N,
            Direction::E => Direction::E,
            Direction::S => Direction::S,
            Direction::W => Direction::W,
        }
    }
}

impl Direction {
    fn turn_right(&self) -> Self {
        match self {
            Direction::N => Direction::E,
            Direction::E => Direction::S,
            Direction::S => Direction::W,
            Direction::W => Direction::N,
        }
    }

    fn move_forward(&self, position: (isize, isize)) -> (isize, isize) {
        match self {
            Direction::N => (position.0 - 1, position.1),
            Direction::E => (position.0, position.1 + 1),
            Direction::S => (position.0 + 1, position.1),
            Direction::W => (position.0, position.1 - 1),
        }
    }
}

struct Map {
    start_direction: Direction,
    start_position: (isize, isize),
    obstacles: Vec<Vec<bool>>,
}

impl Map {
    fn is_out_of_bounds(&self, position: (isize, isize)) -> bool {
        position.0 < 0
            || position.1 < 0
            || position.0 >= self.obstacles[0].len() as isize
            || position.1 >= self.obstacles.len() as isize
    }

    fn is_obstacle(&self, position: (isize, isize)) -> bool {
        self.obstacles[position.0 as usize][position.1 as usize]
    }

    fn move_until_out_of_bounds_or_loop(
        &self,
        additional_obstacle: Option<(isize, isize)>,
    ) -> (bool, HashSet<(isize, isize, Direction)>) {
        let mut possition = self.start_position;
        let mut direction = self.start_direction.clone();
        let mut visited_positions = HashSet::new();
        visited_positions.insert((possition.0, possition.1, direction.clone()));

        loop {
            let mut next_position;
            loop {
                next_position = direction.move_forward(possition);
                if self.is_out_of_bounds(next_position) {
                    return (false, visited_positions);
                }
                if self.is_obstacle(next_position) || Some(next_position) == additional_obstacle {
                    direction = direction.turn_right();
                } else {
                    break;
                }
            }

            possition = next_position;

            // Loop Detection
            if visited_positions.contains(&(possition.0, possition.1, direction.clone())) {
                return (true, visited_positions);
            }

            visited_positions.insert((possition.0, possition.1, direction.clone()));
        }
    }
}

type Input = Map;
type Output = usize;

struct Day {}

impl AoCDay<Input, Output> for Day {
    fn parse_input(&self, id: &str) -> Result<Input, Box<dyn Error>> {
        let mut start_position = (0, 0);

        let obstacles = get_reader(id)?
            .lines()
            .map_while(Result::ok)
            .enumerate()
            .map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        '#' => Ok(true),
                        '.' => Ok(false),
                        '^' => {
                            start_position = (y as isize, x as isize);
                            Ok(false)
                        }
                        _ => Err("Invalid input"),
                    })
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<std::result::Result<_, _>>()?;

        Ok(Map {
            start_direction: Direction::N,
            start_position,
            obstacles,
        })
    }

    fn part1(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        let (_, visited_positions_and_directions) = input.move_until_out_of_bounds_or_loop(None);
        Ok(visited_positions_and_directions
            .iter()
            .map(|(y, x, _)| (y, x))
            .collect::<HashSet<_>>()
            .len())
    }

    fn part2(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        let (_, visited_positions_and_directions) = input.move_until_out_of_bounds_or_loop(None);
        let visited_positions = visited_positions_and_directions
            .iter()
            .map(|(y, x, _)| (y, x))
            .collect::<HashSet<_>>();

        Ok(visited_positions
            .par_iter()
            .filter(|(&y, &x)| {
                let (contains_loop, _) = input.move_until_out_of_bounds_or_loop(Some((y, x)));
                contains_loop
            })
            .count())
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
            41
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
