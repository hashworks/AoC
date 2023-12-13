mod util;

use std::{error::Error, io::BufRead};
use util::{aoc::AoCDay, input::get_reader};

const ID: &str = "day13";
type Input = Vec<Vec<Vec<bool>>>;
type Output = usize;

struct Day {}

impl AoCDay<Input, Output> for Day {
    fn parse_input(&self, id: &str) -> Result<Input, Box<dyn Error>> {
        let mut maps = vec![vec![]];

        for column in get_reader(id)?
            .lines()
            .flatten()
            .map(|l| l.chars().map(|c| c == '#').collect::<Vec<_>>())
        {
            if column.is_empty() {
                maps.push(vec![]);
            } else {
                maps.last_mut().unwrap().push(column);
            }
        }

        Ok(maps)
    }

    fn part1(&self, maps: &Input) -> Result<Output, Box<dyn Error>> {
        Ok(maps
            .iter()
            .map(|map| {
                let (h_ref, v_ref) = get_reflection_position(map, None, None);
                h_ref.map(|h| h * 100).unwrap_or(v_ref.unwrap_or(0))
            })
            .sum())
    }

    fn part2(&self, maps: &Input) -> Result<Output, Box<dyn Error>> {
        Ok(maps
            .iter()
            .map(|map| {
                let (old_h_ref, old_v_ref) = get_reflection_position(map, None, None);

                for (y, row) in map.iter().enumerate() {
                    for (x, cell) in row.iter().enumerate() {
                        let mut map = map.clone();
                        map[y][x] = !cell;
                        let (new_h_ref, new_v_ref) =
                            get_reflection_position(&map, old_h_ref, old_v_ref);
                        let value = new_h_ref.map(|h| h * 100).or(new_v_ref);
                        if let Some(value) = value {
                            return value;
                        }
                    }
                }

                0
            })
            .sum())
    }
}

fn transpose(map: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut transposed = vec![];

    for x in 0..map[0].len() {
        let mut t_column = vec![];

        for column in map {
            t_column.push(column[x]);
        }

        transposed.push(t_column);
    }

    transposed
}

fn get_horizontal_reflection(map: &Vec<Vec<bool>>, ignore_y: Option<usize>) -> Option<usize> {
    map.iter()
        .enumerate()
        .filter(|(y, _)| ignore_y != Some(*y))
        .find(|(y, row)| {
            y > &0
                && map.get(y - 1) == Some(row)
                && (0..y - 1).rev().zip(y + 1..map.len()).all(|(y1, y2)| {
                    let left = map.get(y1);
                    let right = map.get(y2);
                    left.is_none() || right.is_none() || left == right
                })
        })
        .map(|(y, _)| y)
}

fn get_reflection_position(
    map: &Vec<Vec<bool>>,
    ignore_h_reflection: Option<usize>,
    ignore_v_reflection: Option<usize>,
) -> (Option<usize>, Option<usize>) {
    let h_reflection = get_horizontal_reflection(map, ignore_h_reflection);
    if h_reflection.is_some() {
        (h_reflection, None)
    } else {
        (
            None,
            get_horizontal_reflection(&transpose(map), ignore_v_reflection),
        )
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
            405
        );
    }

    #[test]
    fn test_solve_part2() {
        let day = Day {};
        assert_eq!(
            day.parse_and_solve_part2(format!("{}_test1", ID).as_str())
                .unwrap(),
            400
        );
    }
}
