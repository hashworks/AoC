mod util;

use hashbrown::HashSet;
use std::{error::Error, io::BufRead};
use util::{aoc::AoCDay, input::get_reader};

const ID: &str = "day12";

/*
AAAA
BBCD
BBCC
EEEC
*/

type Input = Vec<Vec<char>>;
type Output = usize;

fn extract_regions(map: &Input) -> Vec<(char, HashSet<(isize, isize)>)> {
    let mut visited_points = HashSet::new();

    map.iter()
        .enumerate()
        .flat_map(|(y, row)| {
            let y = y as isize;
            row.iter()
                .enumerate()
                .filter_map(|(x, c)| {
                    let x = x as isize;

                    if visited_points.contains(&(y, x)) {
                        return None;
                    }

                    let mut region = HashSet::new();
                    let mut stack = vec![(y, x)];

                    while let Some((y, x)) = stack.pop() {
                        if visited_points.contains(&(y, x)) {
                            continue;
                        }

                        visited_points.insert((y, x));
                        region.insert((y, x));

                        if y > 0 && map[y as usize - 1][x as usize] == *c {
                            stack.push((y - 1, x));
                        }

                        if x < map[y as usize].len() as isize - 1
                            && map[y as usize][x as usize + 1] == *c
                        {
                            stack.push((y, x + 1));
                        }

                        if y < map.len() as isize - 1 && map[y as usize + 1][x as usize] == *c {
                            stack.push((y + 1, x));
                        }

                        if x > 0 && map[y as usize][x as usize - 1] == *c {
                            stack.push((y, x - 1));
                        }
                    }

                    Some((*c, region))
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

fn region_edge_count(region: &HashSet<(isize, isize)>) -> usize {
    region
        .iter()
        .flat_map(|(y, x)| {
            let y = *y;
            let x = *x;
            [(y - 1, x), (y, x + 1), (y + 1, x), (y, x - 1)]
                .iter()
                .filter(|(y, x)| y < &0 || x < &0 || !region.contains(&(*y, *x)))
                .cloned()
                .collect::<Vec<_>>()
        })
        .count()
}

fn region_side_count(region: &HashSet<(isize, isize)>) -> usize {
    region
        .iter()
        .map(|(y, x)| {
            let y = *y;
            let x = *x;
            [
                ((y, x - 1), (y - 1, x - 1), (y - 1, x)),
                ((y - 1, x), (y - 1, x + 1), (y, x + 1)),
                ((y, x + 1), (y + 1, x + 1), (y + 1, x)),
                ((y + 1, x), (y + 1, x - 1), (y, x - 1)),
            ]
            .iter()
            .filter(|(a, b, c)| {
                // outer, inner and diag corners
                (!region.contains(a) && !region.contains(b) && !region.contains(c))
                    || region.contains(a) && !region.contains(b) && region.contains(c)
                    || !region.contains(a) && region.contains(b) && !region.contains(c)
            })
            .count()
        })
        .sum()
}

struct Day {}

impl AoCDay<Input, Output> for Day {
    fn parse_input(&self, id: &str) -> Result<Input, Box<dyn Error>> {
        get_reader(id)?
            .lines()
            .map(|l| Ok(l?.chars().collect()))
            .collect()
    }

    fn part1(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        Ok(extract_regions(input)
            .iter()
            .map(|(_, r)| r.len() * region_edge_count(r))
            .sum())
    }

    fn part2(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        Ok(extract_regions(input)
            .iter()
            .map(|(_, r)| r.len() * region_side_count(r))
            .sum())
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
        [140, 772, 692, 1184, 1930]
            .iter()
            .enumerate()
            .for_each(|(i, expected)| {
                assert_eq!(
                    day.parse_and_solve_part1(format!("{}_test{}", ID, i + 1).as_str())
                        .expect("Test should not fail"),
                    *expected
                );
            });
    }

    #[test]
    fn test_solve_part2() {
        let day = Day {};
        [80, 436, 236, 368, 1206]
            .iter()
            .enumerate()
            .for_each(|(i, expected)| {
                assert_eq!(
                    day.parse_and_solve_part2(format!("{}_test{}", ID, i + 1).as_str())
                        .expect("Test should not fail"),
                    *expected
                );
            });
    }
}
