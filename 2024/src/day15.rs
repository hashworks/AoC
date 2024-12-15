mod util;

use std::{error::Error, io::BufRead};
use util::{aoc::AoCDay, input::get_reader};

const ID: &str = "day15";
type Input = ((usize, usize), Vec<Vec<Cell>>, Vec<Direction>);
type Output = usize;

#[derive(Clone, Debug)]
enum Cell {
    Wall,
    Box,
    LeftBox,
    RightBox,
    Empty,
}

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn new_position(&self, pos: &(usize, usize)) -> (usize, usize) {
        match self {
            Direction::North => (pos.0 - 1, pos.1),
            Direction::East => (pos.0, pos.1 + 1),
            Direction::South => (pos.0 + 1, pos.1),
            Direction::West => (pos.0, pos.1 - 1),
        }
    }
}

struct Day {}

// Note: This asserts two things:
// * The robot is in the map
// * The map is surrounded by walls
impl AoCDay<Input, Output> for Day {
    fn parse_input(&self, id: &str) -> Result<Input, Box<dyn Error>> {
        let mut robot = (0, 0);

        let lines = get_reader(id)?.lines().collect::<Result<Vec<_>, _>>()?;

        let map = lines
            .iter()
            .enumerate()
            .take_while(|(_, line)| !line.is_empty())
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .inspect(|(x, c)| {
                        if *c == '@' {
                            robot = (y, *x);
                        }
                    })
                    .map(|(_, c)| match c {
                        '#' => Ok(Cell::Wall),
                        '.' => Ok(Cell::Empty),
                        'O' => Ok(Cell::Box),
                        '@' => Ok(Cell::Empty),
                        _ => Err::<_, Box<dyn Error>>("Invalid input".into()),
                    })
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?;

        let directions = lines
            .iter()
            .skip_while(|line| !line.is_empty())
            .flat_map(|line| {
                line.chars().map(|c| match c {
                    '>' => Ok(Direction::East),
                    '<' => Ok(Direction::West),
                    'v' => Ok(Direction::South),
                    '^' => Ok(Direction::North),
                    _ => Err::<_, Box<dyn Error>>("Invalid input".into()),
                })
            })
            .collect::<Result<_, _>>()?;

        Ok((robot, map, directions))
    }

    fn part1(&self, (robot, map, directions): &Input) -> Result<Output, Box<dyn Error>> {
        let mut robot = *robot;
        let mut map = map.clone();

        for direction in directions {
            let new_pos = direction.new_position(&robot);

            match map[new_pos.0][new_pos.1] {
                Cell::Empty => robot = new_pos,
                Cell::Box => {
                    if let Some(new_box_pos) = match direction {
                        Direction::North => (0..new_pos.0)
                            .rev()
                            .take_while(|y| !matches!(map[*y][new_pos.1], Cell::Wall))
                            .find(|y| matches!(map[*y][new_pos.1], Cell::Empty))
                            .map(|y| (y, new_pos.1)),
                        Direction::East => (new_pos.1 + 1..)
                            .take_while(|x| !matches!(map[new_pos.0][*x], Cell::Wall))
                            .find(|x| matches!(map[new_pos.0][*x], Cell::Empty))
                            .map(|x| (new_pos.0, x)),
                        Direction::South => (new_pos.0 + 1..)
                            .take_while(|y| !matches!(map[*y][new_pos.1], Cell::Wall))
                            .find(|y| matches!(map[*y][new_pos.1], Cell::Empty))
                            .map(|y| (y, new_pos.1)),
                        Direction::West => (0..new_pos.1)
                            .rev()
                            .take_while(|x| !matches!(map[new_pos.0][*x], Cell::Wall))
                            .find(|x| matches!(map[new_pos.0][*x], Cell::Empty))
                            .map(|x| (new_pos.0, x)),
                    } {
                        map[new_pos.0][new_pos.1] = Cell::Empty;
                        map[new_box_pos.0][new_box_pos.1] = Cell::Box;
                        robot = new_pos;
                    }
                }
                _ => {}
            }
        }

        Ok(get_gps_coordinates_sum(&map))
    }

    fn part2(&self, (robot, map, directions): &Input) -> Result<Output, Box<dyn Error>> {
        let mut robot = (robot.0, robot.1 * 2);
        let mut map = double_map(map);

        for direction in directions {
            let new_pos = direction.new_position(&robot);

            match map[new_pos.0][new_pos.1] {
                Cell::Empty => robot = new_pos,
                Cell::LeftBox | Cell::RightBox => {
                    if let Some(mut boxes_to_move) = double_box_dfs(&map, &new_pos, direction) {
                        boxes_to_move.sort_by(|a, b| match direction {
                            Direction::North => a.0.cmp(&b.0).then(a.1.cmp(&b.1)),
                            Direction::West => a.1.cmp(&b.1).then(a.0.cmp(&b.0)),
                            Direction::South => b.0.cmp(&a.0).then(b.1.cmp(&a.1)),
                            Direction::East => b.1.cmp(&a.1).then(b.0.cmp(&a.0)),
                        });

                        boxes_to_move
                            .iter()
                            .enumerate()
                            .filter(|(i, old_box_pos)| {
                                i == &0 || boxes_to_move.get(i - 1) != Some(old_box_pos)
                            })
                            .for_each(|(_, old_box_pos)| {
                                let cell = map[old_box_pos.0][old_box_pos.1].clone();
                                let new_box_pos = direction.new_position(old_box_pos);
                                map[old_box_pos.0][old_box_pos.1] = Cell::Empty;
                                map[new_box_pos.0][new_box_pos.1] = cell;
                            });
                        robot = new_pos;
                    }
                }
                _ => {}
            }
        }

        Ok(get_gps_coordinates_sum(&map))
    }
}

fn get_gps_coordinates_sum(map: &[Vec<Cell>]) -> usize {
    map.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, cell)| matches!(cell, Cell::Box | Cell::LeftBox))
                .map(|(x, _)| y * 100 + x)
                .sum::<usize>()
        })
        .sum()
}

fn double_map(map: &[Vec<Cell>]) -> Vec<Vec<Cell>> {
    map.iter()
        .map(|row| {
            row.iter()
                .flat_map(|cell| match cell {
                    Cell::Wall => vec![Cell::Wall, Cell::Wall],
                    Cell::Box => vec![Cell::LeftBox, Cell::RightBox],
                    Cell::Empty => vec![Cell::Empty, Cell::Empty],
                    _ => vec![],
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

// This is not optimal, should be a bfs
fn double_box_dfs(
    map: &[Vec<Cell>],
    pos: &(usize, usize),
    direction: &Direction,
) -> Option<Vec<(usize, usize)>> {
    let pos_cell = &map[pos.0][pos.1];

    let (left_pos, right_pos) = match pos_cell {
        Cell::Empty => return Some(vec![]),
        Cell::LeftBox => (*pos, (pos.0, pos.1 + 1)),
        Cell::RightBox => ((pos.0, pos.1 - 1), *pos),
        _ => return None,
    };

    match direction {
        &Direction::East => (pos.1 + 1..)
            .take_while(|x| !matches!(map[pos.0][*x], Cell::Wall))
            .find(|x| matches!(map[pos.0][*x], Cell::Empty))
            .map(|x| {
                (pos.1 + 1..x)
                    .map(|x| (pos.0, x))
                    .chain([*pos])
                    .collect::<Vec<_>>()
            }),
        &Direction::West => (0..pos.1)
            .rev()
            .take_while(|x| !matches!(map[pos.0][*x], Cell::Wall))
            .find(|x| matches!(map[pos.0][*x], Cell::Empty))
            .map(|x| {
                (x + 1..pos.1)
                    .rev()
                    .map(|x| (pos.0, x))
                    .chain([*pos])
                    .collect::<Vec<_>>()
            }),
        direction => double_box_dfs(map, &direction.new_position(&left_pos), direction).map(
            |left_results| {
                double_box_dfs(map, &direction.new_position(&right_pos), direction).map(
                    |right_results| {
                        [left_pos, right_pos]
                            .iter()
                            .cloned()
                            .chain(left_results)
                            .chain(right_results)
                            .collect()
                    },
                )
            },
        )?,
    }
}

#[allow(dead_code)]
fn print_map(map: &[Vec<Cell>], robot: (usize, usize)) {
    map.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, cell)| match cell {
            Cell::Wall => print!("#"),
            Cell::Box => print!("O"),
            Cell::LeftBox => print!("["),
            Cell::RightBox => print!("]"),
            Cell::Empty => {
                if (y, x) == robot {
                    print!("@");
                } else {
                    print!(".");
                }
            }
        });
        println!();
    });
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
            2028
        );
        assert_eq!(
            day.parse_and_solve_part1(format!("{}_test2", ID).as_str())
                .unwrap(),
            10092
        );
    }

    #[test]
    fn test_solve_part2() {
        let day = Day {};
        assert_eq!(
            day.parse_and_solve_part2(format!("{}_test2", ID).as_str())
                .unwrap(),
            9021
        );
    }
}
