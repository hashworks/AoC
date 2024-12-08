mod util;

use std::{
    collections::{HashMap, HashSet},
    error::Error,
    io::BufRead,
};
use util::{aoc::AoCDay, input::get_reader};

const ID: &str = "day08";
type Input = (isize, isize, HashMap<char, Vec<(isize, isize)>>);
type Output = usize;

struct Day {}

impl AoCDay<Input, Output> for Day {
    fn parse_input(&self, id: &str) -> Result<Input, Box<dyn Error>> {
        let mut max_y = 0isize;
        let mut max_x = 0isize;
        let mut antennas = HashMap::new();

        for (y, line) in get_reader(id)?.lines().enumerate() {
            let line = line?;

            let y = y as isize;
            if max_y < y {
                max_y = y;
            }

            for (x, char) in line.chars().enumerate() {
                let x = x as isize;
                if max_x < x {
                    max_x = x;
                }
                if char != '.' {
                    let ant = antennas.entry(char).or_insert(Vec::new());
                    ant.push((y, x));
                }
            }
        }

        Ok((max_x, max_y, antennas))
    }

    fn part1(&self, (max_y, max_x, antennas): &Input) -> Result<Output, Box<dyn Error>> {
        Ok(antennas
            .iter()
            .flat_map(|(_, positions)| {
                positions
                    .iter()
                    .flat_map(|pos| {
                        positions
                            .iter()
                            .filter(|&other_pos| other_pos != pos)
                            .filter_map(|other_pos| {
                                let y_distance = pos.0 - other_pos.0;
                                let x_distance = pos.1 - other_pos.1;

                                let y = pos.0 + y_distance;
                                let x = pos.1 + x_distance;

                                if y > *max_y || x > *max_x || y < 0 || x < 0 {
                                    return None;
                                }
                                Some((y, x))
                            })
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<HashSet<_>>()
            .len())
    }

    fn part2(&self, (max_y, max_x, antennas): &Input) -> Result<Output, Box<dyn Error>> {
        Ok(antennas
            .iter()
            .flat_map(|(_, positions)| {
                positions
                    .iter()
                    .flat_map(|pos| {
                        positions
                            .iter()
                            .filter(|&other_pos| other_pos != pos)
                            .flat_map(|other_pos| {
                                let y_distance = pos.0 - other_pos.0;
                                let x_distance = pos.1 - other_pos.1;

                                (0..)
                                    .map(|i| (pos.0 + y_distance * i, pos.1 + x_distance * i))
                                    .take_while(|(y, x)| {
                                        y <= max_y && x <= max_x && *y >= 0 && *x >= 0
                                    })
                                    .collect::<Vec<_>>()
                            })
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<HashSet<_>>()
            .len())
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
            14
        );
    }

    #[test]
    fn test_solve_part2() {
        let day = Day {};
        assert_eq!(
            day.parse_and_solve_part2(format!("{}_test1", ID).as_str())
                .unwrap(),
            34
        );
    }
}
