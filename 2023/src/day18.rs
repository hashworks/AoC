mod util;

use std::{error::Error, io::BufRead};
use util::{aoc::AoCDay, input::get_reader};

const ID: &str = "day18";
type Input = Vec<Movement>;
type Output = isize;

#[derive(Debug)]
struct Movement {
    direction: Direction,
    distance: usize,
    color: Vec<char>,
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Day {}

impl AoCDay<Input, Output> for Day {
    fn parse_input(&self, id: &str) -> Result<Input, Box<dyn Error>> {
        get_reader(id)?
            .lines()
            .map(|l| {
                let l = l?;

                let direction: Result<_, Box<dyn Error>> =
                    match l.chars().next().ok_or("Empty line")? {
                        'U' => Ok(Direction::Up),
                        'D' => Ok(Direction::Down),
                        'L' => Ok(Direction::Left),
                        'R' => Ok(Direction::Right),
                        _ => return Err("Invalid direction".into()),
                    };
                let direction = direction?;

                let (step_str, color_str) = l[2..].split_once(' ').ok_or("Invalid line")?;
                let distance = step_str.parse()?;
                let color = color_str[2..color_str.len() - 1].chars().collect();

                Ok(Movement {
                    direction,
                    distance,
                    color,
                })
            })
            .collect()
    }

    fn part1(&self, movements: &Input) -> Result<Output, Box<dyn Error>> {
        let mut pos = (0, 0);
        let mut edges = vec![];

        let mut trenches = 0;

        for movement in movements {
            let distance = movement.distance as isize;
            trenches += distance;
            match movement.direction {
                Direction::Up => {
                    pos.0 -= distance;
                    edges.push(pos);
                }
                Direction::Down => {
                    pos.0 += distance;
                    edges.push(pos);
                }
                Direction::Left => {
                    pos.1 -= distance;
                    edges.push(pos);
                }
                Direction::Right => {
                    pos.1 += distance;
                    edges.push(pos);
                }
            }
        }

        Ok(shoelace_theorem(&edges) + trenches / 2 + 1)
    }

    fn part2(&self, movements: &Input) -> Result<Output, Box<dyn Error>> {
        let mut pos = (0, 0);
        let mut edges = vec![];

        let mut trenches = 0;

        for movement in movements {
            let distance = hex_chars_to_int(&movement.color[0..5]);
            trenches += distance;
            match movement.color[5] {
                '3' => {
                    pos.0 -= distance;
                    edges.push(pos);
                }
                '1' => {
                    pos.0 += distance;
                    edges.push(pos);
                }
                '2' => {
                    pos.1 -= distance;
                    edges.push(pos);
                }
                '0' => {
                    pos.1 += distance;
                    edges.push(pos);
                }
                _ => return Err("Invalid direction".into()),
            }
        }

        Ok(shoelace_theorem(&edges) + trenches / 2 + 1)
    }
}

fn shoelace_theorem(points: &[(isize, isize)]) -> isize {
    points
        .windows(2)
        .map(|p| p[0].0 * p[1].1 - p[1].0 * p[0].1)
        .sum::<isize>()
        .abs()
        / 2
}

fn hex_chars_to_int(hex: &[char]) -> isize {
    hex.iter()
        .map(|c| c.to_digit(16).unwrap_or(0) as isize)
        .fold(0, |acc, d| acc * 16 + d)
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
            62
        );
    }

    #[test]
    fn test_solve_part2() {
        let day = Day {};
        assert_eq!(
            day.parse_and_solve_part2(format!("{}_test1", ID).as_str())
                .unwrap(),
            952408144115
        );
    }
}
