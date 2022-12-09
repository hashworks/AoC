mod util;

use std::{collections::HashSet, error::Error, io::BufRead};
use util::{aoc::AoCDay, input::get_reader};

const ID: &str = "day09";
type Input = Vec<(u8, usize)>;
type Output = usize;

struct Day {}

impl AoCDay<Input, Output> for Day {
    fn parse_input(&self, id: &str) -> Result<Input, Box<dyn Error>> {
        get_reader(id)?
            .lines()
            .map(|line| {
                let line = line?;
                Ok((
                    *line.as_bytes().first().ok_or("bad input: empty line")?,
                    line[2..].parse::<usize>()?,
                ))
            })
            .collect()
    }

    fn part1(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        let mut h = (0i16, 0i16);
        let mut t = (0i16, 0i16);
        let mut visited_points = HashSet::<(i16, i16)>::new();

        visited_points.insert(h);

        for (dir, dist) in input {
            let (hdx, hdy) = match_direction(dir)?;

            for _ in 0..*dist {
                h.0 += hdx;
                h.1 += hdy;

                let th_dx = h.0 - t.0;
                let th_dy = h.1 - t.1;

                if th_dx.abs() > 1 || th_dy.abs() > 1 {
                    t.0 += th_dx.clamp(-1, 1);
                    t.1 += th_dy.clamp(-1, 1);
                    visited_points.insert(t);
                }
            }
        }

        Ok(visited_points.len())
    }

    fn part2(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        let mut h = (0i16, 0i16);
        let mut ts = vec![(0i16, 0i16); 9];
        let mut visited_points = HashSet::<(i16, i16)>::new();

        visited_points.insert(h);

        for (dir, dist) in input {
            let (hdx, hdy) = match_direction(dir)?;

            for _ in 0..*dist {
                h.0 += hdx;
                h.1 += hdy;

                let mut last_element = h;
                for (i, t) in ts.iter_mut().enumerate() {
                    let th_dx = last_element.0 - t.0;
                    let th_dy = last_element.1 - t.1;

                    if th_dx.abs() > 1 || th_dy.abs() > 1 {
                        t.0 += th_dx.clamp(-1, 1);
                        t.1 += th_dy.clamp(-1, 1);
                        if i == 8 {
                            visited_points.insert(*t);
                        }
                    }

                    last_element = *t;
                }
            }
        }

        Ok(visited_points.len())
    }
}

fn match_direction(dir: &u8) -> Result<(i16, i16), Box<dyn Error>> {
    match dir {
        b'U' => Ok((0, 1)),
        b'D' => Ok((0, -1)),
        b'R' => Ok((1, 0)),
        b'L' => Ok((-1, 0)),
        _ => Err("bad input: invalid direction".into()),
    }
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
            13
        );
    }

    #[test]
    fn test_solve_part2() {
        let day = Day {};
        assert_eq!(
            day.parse_and_solve_part2(format!("{}_test2", ID).as_str())
                .unwrap(),
            36
        );
    }
}

fn main() {
    Day {}.run(ID);
}
