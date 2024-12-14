mod util;

use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{error::Error, io::BufRead};
use util::{aoc::AoCDay, input::get_reader};

const ID: &str = "day14";
type Input = (isize, isize, Vec<(isize, isize, isize, isize)>);
type Output = isize;

struct Day {}

impl AoCDay<Input, Output> for Day {
    fn parse_input(&self, id: &str) -> Result<Input, Box<dyn Error>> {
        let (max_x, max_y) = if id.ends_with("test1") {
            (11, 7)
        } else {
            (101, 103)
        };

        Ok((
            max_x,
            max_y,
            get_reader(id)?
                .lines()
                .map(|line| {
                    let line = line?;

                    let (ps, vs) = line
                        .strip_prefix("p=")
                        .ok_or("Invalid input")?
                        .split_once(" v=")
                        .ok_or("Invalid input")?;

                    let (p1, p2) = ps.split_once(',').ok_or("Invalid input")?;

                    let (v1, v2) = vs.split_once(',').ok_or("Invalid input")?;

                    Ok((p1.parse()?, p2.parse()?, v1.parse()?, v2.parse()?))
                })
                .collect::<Result<Vec<_>, Box<dyn Error>>>()?,
        ))
    }

    fn part1(&self, (max_x, max_y, robots): &Input) -> Result<Output, Box<dyn Error>> {
        let (max_x, max_y) = (*max_x, *max_y);

        let robots = robots
            .iter()
            .map(|(x, y, vx, vy)| {
                (
                    (x + vx * 100).rem_euclid(max_x),
                    (y + vy * 100).rem_euclid(max_y),
                )
            })
            .collect::<Vec<_>>();

        let (q1, q2, q3, q4) = robots
            .iter()
            .fold((0, 0, 0, 0), |(q1, q2, q3, q4), (x, y)| {
                let x = *x;
                let y = *y;

                if x >= 0 && y >= 0 && x < max_x / 2 && y < max_y / 2 {
                    (q1 + 1, q2, q3, q4)
                } else if x > max_x / 2 && y >= 0 && x < max_x && y < max_y / 2 {
                    (q1, q2 + 1, q3, q4)
                } else if x >= 0 && y > max_y / 2 && x < max_x / 2 && y < max_y {
                    (q1, q2, q3 + 1, q4)
                } else if x > max_x / 2 && y > max_y / 2 && x < max_x && y < max_y {
                    (q1, q2, q3, q4 + 1)
                } else {
                    (q1, q2, q3, q4)
                }
            });

        Ok(q1 * q2 * q3 * q4)
    }

    fn part2(&self, (max_x, max_y, robots): &Input) -> Result<Output, Box<dyn Error>> {
        let (max_x, max_y) = (*max_x, *max_y);

        (0..100000)
            .into_par_iter()
            .find_first(|i| {
                has_straight_line(
                    &mut robots
                        .iter()
                        .map(|(x, y, vx, vy)| {
                            (
                                (x + vx * i).rem_euclid(max_x),
                                (y + vy * i).rem_euclid(max_y),
                                *vx,
                                *vy,
                            )
                        })
                        .collect::<Vec<_>>(),
                    20,
                )
            })
            .ok_or("No solution found".into())
    }
}

fn has_straight_line(robots: &mut [(isize, isize, isize, isize)], length: usize) -> bool {
    robots.sort_by(|a, b| {
        if a.0 == b.0 {
            a.1.cmp(&b.1)
        } else {
            a.0.cmp(&b.0)
        }
    });

    let mut streak = 0;

    for i in 1..robots.len() {
        if robots[i].0 == robots[i - 1].0 && robots[i].1 == robots[i - 1].1 + 1 {
            streak += 1;
        } else {
            streak = 1;
        }

        if streak > length {
            return true;
        }
    }

    false
}

#[allow(dead_code)]
fn print_robots(max_x: isize, max_y: isize, robots: Vec<(isize, isize)>) {
    (0..max_y).for_each(|y| {
        (0..max_x).for_each(|x| {
            let robots = robots
                .iter()
                .filter(|(rx, ry)| *rx == x && *ry == y)
                .count();
            if robots > 0 {
                print!("{}", robots);
            } else {
                print!(".");
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
            12
        );
    }
}
