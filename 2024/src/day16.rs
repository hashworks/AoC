mod util;

use hashbrown::HashSet;
use pathfinding::prelude::{astar_bag, dijkstra};

use std::{error::Error, io::BufRead};
use util::{aoc::AoCDay, input::get_reader};

const ID: &str = "day16";
type Input = ((usize, usize), (usize, usize), Vec<Vec<bool>>);
type Output = usize;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Direction {
    N,
    E,
    S,
    W,
}

impl Direction {
    fn turn_left(&self) -> Self {
        match self {
            Direction::N => Direction::W,
            Direction::E => Direction::N,
            Direction::S => Direction::E,
            Direction::W => Direction::S,
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Direction::N => Direction::E,
            Direction::E => Direction::S,
            Direction::S => Direction::W,
            Direction::W => Direction::N,
        }
    }

    fn move_forward(&self, position: (usize, usize)) -> (usize, usize) {
        match self {
            Direction::N => (position.0 - 1, position.1),
            Direction::E => (position.0, position.1 + 1),
            Direction::S => (position.0 + 1, position.1),
            Direction::W => (position.0, position.1 - 1),
        }
    }
}

struct Day {}

impl AoCDay<Input, Output> for Day {
    fn parse_input(&self, id: &str) -> Result<Input, Box<dyn Error>> {
        let mut start = (0, 0);
        let mut end = (0, 0);
        let map = get_reader(id)?
            .lines()
            .enumerate()
            .map(|(y, line)| {
                let line = line?;
                line.chars()
                    .enumerate()
                    .inspect(|(x, c)| match c {
                        'S' => start = (y, *x),
                        'E' => end = (y, *x),
                        _ => {}
                    })
                    .map(|(_, c)| match c {
                        '#' => Ok(false),
                        '.' | 'S' | 'E' => Ok(true),
                        _ => Err::<_, Box<dyn Error>>("Invalid input".into()),
                    })
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok((start, end, map))
    }

    fn part1(&self, (start, end, map): &Input) -> Result<Output, Box<dyn Error>> {
        dijkstra(
            &(start.0, start.1, Direction::E),
            |state| sucessors(state, map),
            |&(y, x, _)| (y, x) == *end,
        )
        .map(|(_, cost)| cost)
        .ok_or("No path found".into())
    }

    fn part2(&self, (start, end, map): &Input) -> Result<Output, Box<dyn Error>> {
        let best_paths = astar_bag(
            &(start.0, start.1, Direction::E),
            |state| sucessors(state, map),
            |_| 0,
            |&(y, x, _)| (y, x) == *end,
        )
        .ok_or("No path found")?;

        Ok(best_paths
            .0
            .flat_map(|path| path.iter().map(|(y, x, _)| (*y, *x)).collect::<Vec<_>>())
            .collect::<HashSet<_>>()
            .len())
    }
}

fn sucessors(
    (y, x, direction): &(usize, usize, Direction),
    map: &[Vec<bool>],
) -> Vec<((usize, usize, Direction), usize)> {
    let turned_left = ((*y, *x, direction.turn_left()), 1000);
    let turned_right = ((*y, *x, direction.turn_right()), 1000);
    let next_position: (usize, usize) = direction.move_forward((*y, *x));
    if let Some(Some(Some(moved_to_empty_space))) = map.get(next_position.0).map(|row| {
        row.get(next_position.1).map(|cell| {
            if *cell {
                Some(((next_position.0, next_position.1, *direction), 1))
            } else {
                None
            }
        })
    }) {
        vec![moved_to_empty_space, turned_left, turned_right]
    } else {
        vec![turned_left, turned_right]
    }
}

#[allow(dead_code)]
fn print_map(map: &[Vec<bool>]) {
    for row in map {
        for cell in row {
            print!("{}", if *cell { '.' } else { '#' });
        }
        println!();
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
            7036
        );
        assert_eq!(
            day.parse_and_solve_part1(format!("{}_test2", ID).as_str())
                .unwrap(),
            11048
        );
    }

    #[test]
    fn test_solve_part2() {
        let day = Day {};
        assert_eq!(
            day.parse_and_solve_part2(format!("{}_test1", ID).as_str())
                .unwrap(),
            45
        );
        assert_eq!(
            day.parse_and_solve_part2(format!("{}_test2", ID).as_str())
                .unwrap(),
            64
        );
    }
}
