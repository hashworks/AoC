mod util;

use std::{collections::HashSet, error::Error, io::BufRead};
use util::{aoc::AoCDay, input::get_reader};

const ID: &str = "day10";
type Input = (usize, usize, Vec<Vec<char>>);
type Output = usize;

const VERTICAL_PIPE: char = '|';
const HORIZONTAL_PIPE: char = '-';
const NORTH_EAST_PIPE: char = 'L';
const NORTH_WEST_PIPE: char = 'J';
const SOUTH_WEST_PIPE: char = '7';
const SOUTH_EAST_PIPE: char = 'F';
const START: char = 'S';
const GROUND: char = '.';
const WALL: char = '#';

struct Day {}

impl AoCDay<Input, Output> for Day {
    fn parse_input(&self, id: &str) -> Result<Input, Box<dyn Error>> {
        let (start_y, start_x, map) = get_reader(id)?.lines().enumerate().try_fold(
            (0, 0, Vec::new()),
            |(mut y, mut x, mut map), (y_index, line)| {
                let line = line?;
                map.push(
                    line.chars()
                        .enumerate()
                        .inspect(|(x_index, c)| {
                            if *c == START {
                                y = y_index;
                                x = *x_index;
                            }
                        })
                        .map(|(_, c)| c)
                        .collect(),
                );
                Ok::<_, Box<dyn Error>>((y, x, map))
            },
        )?;
        let mut map = map;

        let start_pipe = get_start_pipe(&start_y, &start_x, &map).ok_or("no valid start pipe")?;
        map[start_y][start_x] = start_pipe;

        Ok((start_y, start_x, map))
    }

    fn part1(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        get_loop_pipe_positions(input).map(|positions| positions.len() / 2)
    }

    fn part2(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        let loop_pipe_positions = get_loop_pipe_positions(input)?;
        let map = &input.2;

        let mut exploded_map = explode_map(map, &loop_pipe_positions).ok_or("no valid map")?;

        let mut queue = vec![(0, 0)];
        while let Some((y, x)) = queue.pop() {
            for (y, x) in [(-1, 0), (0, 1), (1, 0), (0, -1)]
                .iter()
                .map(|(dy, dx)| (y as isize + dy, x as isize + dx))
            {
                let y = y as usize;
                let x = x as usize;
                if exploded_map
                    .get(y)
                    .and_then(|column| column.get(x))
                    .filter(|c| **c == GROUND)
                    .is_some()
                {
                    exploded_map[y][x] = WALL;
                    queue.push((y, x));
                }
            }
        }

        let result = exploded_map
            .iter()
            .enumerate()
            .flat_map(|(y, column)| {
                column
                    .iter()
                    .enumerate()
                    .filter(|(_, c)| **c == GROUND)
                    .map(move |(x, _)| (y, x))
            })
            .map(|(y, x)| (y / 3, x / 3))
            .filter(|(y, x)| !loop_pipe_positions.contains(&(*y as isize, *x as isize)))
            .collect::<HashSet<_>>();

        Ok(result.len())
    }
}

fn explode_map(
    map: &[Vec<char>],
    loop_pipe_positions: &HashSet<(isize, isize)>,
) -> Option<Vec<Vec<char>>> {
    let mut exploded_map = vec![vec![GROUND; map[0].len() * 3]; map.len() * 3];
    for (y, column) in map.iter().enumerate() {
        for (x, c) in column
            .iter()
            .enumerate()
            .filter(|(_, c)| **c != GROUND)
            .filter(|(x, _)| loop_pipe_positions.contains(&(y as isize, *x as isize)))
        {
            let exploded_char = explode_char(*c)?;
            exploded_map[y * 3..y * 3 + 3]
                .iter_mut()
                .flat_map(|column| column.iter_mut().skip(x * 3).take(3))
                .zip(exploded_char.iter().flat_map(|column| column.iter()))
                .for_each(|(c1, c2)| *c1 = *c2);
        }
    }
    Some(exploded_map)
}

fn explode_char(c: char) -> Option<[[char; 3]; 3]> {
    match c {
        VERTICAL_PIPE => Some([
            [GROUND, WALL, GROUND],
            [GROUND, WALL, GROUND],
            [GROUND, WALL, GROUND],
        ]),
        HORIZONTAL_PIPE => Some([
            [GROUND, GROUND, GROUND],
            [WALL, WALL, WALL],
            [GROUND, GROUND, GROUND],
        ]),
        NORTH_EAST_PIPE => Some([
            [GROUND, WALL, GROUND],
            [GROUND, WALL, WALL],
            [GROUND, GROUND, GROUND],
        ]),
        NORTH_WEST_PIPE => Some([
            [GROUND, WALL, GROUND],
            [WALL, WALL, GROUND],
            [GROUND, GROUND, GROUND],
        ]),
        SOUTH_EAST_PIPE => Some([
            [GROUND, GROUND, GROUND],
            [GROUND, WALL, WALL],
            [GROUND, WALL, GROUND],
        ]),
        SOUTH_WEST_PIPE => Some([
            [GROUND, GROUND, GROUND],
            [WALL, WALL, GROUND],
            [GROUND, WALL, GROUND],
        ]),
        _ => None,
    }
}

// Unsafe: This is under the assumption that the start is actually part of a loop
fn get_loop_pipe_positions(
    (start_y, start_x, map): &Input,
) -> Result<HashSet<(isize, isize)>, Box<dyn Error>> {
    let start_pipe = map
        .get(*start_y)
        .and_then(|column| column.get(*start_x))
        .ok_or("no valid start pipe")?;

    let initial_directions = get_next_directions(*start_pipe).ok_or("no directions for pipe")?;

    let mut previous_position = (*start_y as isize, *start_x as isize);
    let mut current_position = (
        *start_y as isize + initial_directions.0 .0,
        *start_x as isize + initial_directions.0 .1,
    );

    let mut positions = HashSet::new();
    positions.insert(previous_position);
    positions.insert(current_position);

    while current_position != (*start_y as isize, *start_x as isize) {
        let pipe = map
            .get(current_position.0 as usize)
            .and_then(|column| column.get(current_position.1 as usize))
            .ok_or("out of bounds")?;

        let directions = get_next_directions(*pipe).ok_or("no directions for pipe")?;

        let new_position = (
            current_position.0 + directions.0 .0,
            current_position.1 + directions.0 .1,
        );

        let new_position = if new_position != previous_position {
            new_position
        } else {
            (
                current_position.0 + directions.1 .0,
                current_position.1 + directions.1 .1,
            )
        };

        previous_position = current_position;
        current_position = new_position;

        positions.insert(current_position);
    }

    Ok(positions)
}

fn get_next_directions(pipe: char) -> Option<((isize, isize), (isize, isize))> {
    match pipe {
        VERTICAL_PIPE => Some(((-1, 0), (1, 0))),
        HORIZONTAL_PIPE => Some(((0, -1), (0, 1))),
        NORTH_EAST_PIPE => Some(((-1, 0), (0, 1))),
        NORTH_WEST_PIPE => Some(((-1, 0), (0, -1))),
        SOUTH_EAST_PIPE => Some(((1, 0), (0, 1))),
        SOUTH_WEST_PIPE => Some(((1, 0), (0, -1))),
        _ => None,
    }
}

// Unsafe: This is under the assumption that the start pipe is biunique
fn get_start_pipe(start_y: &usize, start_x: &usize, map: &[Vec<char>]) -> Option<char> {
    let start_pipe_north = if start_y > &0 {
        Some(&map[*start_y - 1][*start_x])
    } else {
        None
    };
    let start_pipe_east = map[*start_y].get(*start_x + 1);
    let start_pipe_south = map.get(*start_y + 1).map(|column| &column[*start_x]);
    let start_pipe_west = if start_x > &0 {
        Some(&map[*start_y][*start_x - 1])
    } else {
        None
    };

    match (
        start_pipe_north
            .map(|pipe| has_south_connection(*pipe))
            .unwrap_or(false),
        start_pipe_east
            .map(|pipe| has_west_connection(*pipe))
            .unwrap_or(false),
        start_pipe_south
            .map(|pipe| has_north_connection(*pipe))
            .unwrap_or(false),
        start_pipe_west
            .map(|pipe| has_east_connection(*pipe))
            .unwrap_or(false),
    ) {
        (true, _, true, _) => Some(VERTICAL_PIPE),
        (_, true, _, true) => Some(HORIZONTAL_PIPE),
        (true, true, _, _) => Some(NORTH_EAST_PIPE),
        (true, _, _, true) => Some(NORTH_WEST_PIPE),
        (_, true, true, _) => Some(SOUTH_EAST_PIPE),
        (_, _, true, true) => Some(SOUTH_WEST_PIPE),

        _ => None,
    }
}

fn has_north_connection(pipe: char) -> bool {
    pipe == VERTICAL_PIPE || pipe == NORTH_EAST_PIPE || pipe == NORTH_WEST_PIPE
}

fn has_east_connection(pipe: char) -> bool {
    pipe == HORIZONTAL_PIPE || pipe == NORTH_EAST_PIPE || pipe == SOUTH_EAST_PIPE
}

fn has_south_connection(pipe: char) -> bool {
    pipe == VERTICAL_PIPE || pipe == SOUTH_EAST_PIPE || pipe == SOUTH_WEST_PIPE
}

fn has_west_connection(pipe: char) -> bool {
    pipe == HORIZONTAL_PIPE || pipe == NORTH_WEST_PIPE || pipe == SOUTH_WEST_PIPE
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
            8
        );
    }

    #[test]
    fn test_solve_part2_2() {
        let day = Day {};
        assert_eq!(
            day.parse_and_solve_part2(format!("{}_test2", ID).as_str())
                .unwrap(),
            4
        );
    }

    #[test]
    fn test_solve_part2_3() {
        let day = Day {};
        assert_eq!(
            day.parse_and_solve_part2(format!("{}_test3", ID).as_str())
                .unwrap(),
            4
        );
    }

    #[test]
    fn test_solve_part2_4() {
        let day = Day {};
        assert_eq!(
            day.parse_and_solve_part2(format!("{}_test4", ID).as_str())
                .unwrap(),
            8
        );
    }

    #[test]
    fn test_solve_part2_5() {
        let day = Day {};
        assert_eq!(
            day.parse_and_solve_part2(format!("{}_test5", ID).as_str())
                .unwrap(),
            10
        );
    }
}
