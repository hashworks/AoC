mod util;

use pathfinding::directed::astar::astar;
use std::{error::Error, io::BufRead};
use util::{aoc::AoCDay, input::get_reader};

const ID: &str = "day17";
type Input = Vec<Vec<usize>>;
type Output = usize;

struct Day {}

impl AoCDay<Input, Output> for Day {
    fn parse_input(&self, id: &str) -> Result<Input, Box<dyn Error>> {
        get_reader(id)?
            .lines()
            .map(|l| {
                let l = l?;
                l.chars()
                    .map(|s| {
                        s.to_digit(10)
                            .map(|i| i as usize)
                            .ok_or("Invalid input".into())
                    })
                    .collect()
            })
            .collect()
    }

    fn part1(&self, cost_map: &Input) -> Result<Output, Box<dyn Error>> {
        let target_y = cost_map.len() as isize - 1;
        let target_x = cost_map.get(0).map(|r| r.len() as isize - 1).unwrap_or(0);

        astar(
            &START,
            |state| state.get_successors(cost_map, 1, 3),
            |state| state.manhatten_distance(target_y, target_x),
            |state| state.success(target_y, target_x),
        )
        .map(|(_, total_cost)| total_cost)
        .ok_or("No path found".into())
    }

    fn part2(&self, cost_map: &Input) -> Result<Output, Box<dyn Error>> {
        let target_y = cost_map.len() as isize - 1;
        let target_x = cost_map.get(0).map(|r| r.len() as isize - 1).unwrap_or(0);

        astar(
            &START,
            |state| state.get_successors(cost_map, 4, 10),
            |state| state.manhatten_distance(target_y, target_x),
            |state| state.success(target_y, target_x),
        )
        .map(|(_, total_cost)| total_cost)
        .ok_or("No path found".into())
    }
}

const START: State = State {
    y: 0,
    x: 0,
    direction: Direction::Right,
    direction_step_counter: 1,
};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct State {
    y: isize,
    x: isize,
    direction: Direction,
    direction_step_counter: usize,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

impl State {
    fn get_successors(
        &self,
        cost_map: &Input,
        minimum_step_counter: usize,
        maximum_step_counter: usize,
    ) -> Vec<(State, usize)> {
        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
        .iter()
        .filter(|&&direction| direction != self.direction.opposite())
        .filter(|&&direction| {
            direction == self.direction || self.direction_step_counter >= minimum_step_counter
        })
        .filter_map(|&direction| {
            let direction_step_counter = if direction == self.direction {
                self.direction_step_counter + 1
            } else {
                1
            };

            let y = self.y
                + match direction {
                    Direction::Up => -1,
                    Direction::Down => 1,
                    _ => 0,
                };
            let x = self.x
                + match direction {
                    Direction::Left => -1,
                    Direction::Right => 1,
                    _ => 0,
                };

            if direction_step_counter <= maximum_step_counter
                && y >= 0
                && x >= 0
                && y < cost_map.len() as isize
                && x < cost_map.get(0).map(|r| r.len() as isize).unwrap_or(0)
            {
                if let Some(cost) = cost_map.get(y as usize).and_then(|r| r.get(x as usize)) {
                    return Some((
                        State {
                            y,
                            x,
                            direction,
                            direction_step_counter,
                        },
                        *cost,
                    ));
                }
            }

            None
        })
        .collect()
    }

    fn manhatten_distance(&self, target_y: isize, target_x: isize) -> usize {
        (self.y - target_y).unsigned_abs() + (self.x - target_x).unsigned_abs()
    }

    fn success(&self, target_y: isize, target_x: isize) -> bool {
        self.y == target_y && self.x == target_x
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
            102
        );
    }

    #[test]
    fn test_solve_part2() {
        let day = Day {};
        assert_eq!(
            day.parse_and_solve_part2(format!("{}_test1", ID).as_str())
                .unwrap(),
            94
        );
    }
}
