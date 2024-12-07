mod util;

use std::{collections::HashSet, error::Error, io::BufRead, sync::PoisonError};
use util::{aoc::AoCDay, input::get_reader};

const ID: &str = "day06";

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
    direction: Direction,
    position: (isize, isize),
    obstacles: Vec<Vec<bool>>,
    visited_positions: Vec<(isize, isize, Direction)>,
}

impl Clone for Map {
    fn clone(&self) -> Self {
        Map {
            direction: self.direction.clone(),
            position: self.position,
            obstacles: self.obstacles.clone(),
            visited_positions: self.visited_positions.clone(),
        }
    }
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

    fn move_until_out_of_bounds(&mut self) {
        loop {
            let mut next_position;
            loop {
                next_position = self.direction.move_forward(self.position);
                if self.is_out_of_bounds(next_position) {
                    return;
                }
                if self.is_obstacle(next_position) {
                    self.direction = self.direction.turn_right();
                } else {
                    break;
                }
            }

            self.position = next_position;

            self.visited_positions
                .push((self.position.0, self.position.1, self.direction.clone()));
        }
    }
}

type Input = Map;
type Output = usize;

struct Day {}

impl AoCDay<Input, Output> for Day {
    fn parse_input(&self, id: &str) -> Result<Input, Box<dyn Error>> {
        let mut visited_positions = vec![];

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
                            visited_positions.push((y as isize, x as isize, Direction::N));
                            Ok(false)
                        }
                        _ => Err("Invalid input"),
                    })
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<std::result::Result<_, _>>()?;

        let position = visited_positions.first().ok_or("No start position found")?;

        Ok(Map {
            direction: Direction::N,
            position: (position.0, position.1),
            obstacles,
            visited_positions,
        })
    }

    fn part1(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        let mut input = input.clone();
        input.move_until_out_of_bounds();
        Ok(input
            .visited_positions
            .iter()
            .map(|(y, x, _)| (y, x))
            .collect::<HashSet<_>>()
            .len())
    }

    fn part2(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        let mut input = input.clone();
        input.move_until_out_of_bounds();

        Ok(input
            .visited_positions
            .iter()
            .enumerate()
            .filter(|(i, (y, x, d))| {
                let possible_loop_target = match d {
                    Direction::N => (y + 1, x + 1, Direction::E),
                    Direction::E => (y + 1, x - 1, Direction::S),
                    
                }
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
