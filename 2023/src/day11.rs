mod util;

use std::{error::Error, io::BufRead};
use util::{aoc::AoCDay, input::get_reader};

const ID: &str = "day11";
type Input = Vec<(usize, usize)>;
type Output = usize;

struct Day {}

impl AoCDay<Input, Output> for Day {
    fn parse_input(&self, id: &str) -> Result<Input, Box<dyn Error>> {
        Ok(get_reader(id)?
            .lines()
            .flatten()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(|(x, c)| if c == '#' { Some((y, x)) } else { None })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>())
    }

    fn part1(&self, galaxies: &Input) -> Result<Output, Box<dyn Error>> {
        calulate_distances(galaxies, 2)
    }

    fn part2(&self, galaxies: &Input) -> Result<Output, Box<dyn Error>> {
        calulate_distances(galaxies, 1000000)
    }
}

fn calulate_distances(galaxies: &Input, time_warp: usize) -> Result<Output, Box<dyn Error>> {
    let time_warp = time_warp - 1;

    let (max_y, max_x) = galaxies
        .iter()
        .fold((0, 0), |acc, (y, x)| (acc.0.max(*y), acc.1.max(*x)));

    let mut empty_columns = (0..max_y).collect::<Vec<_>>();
    let mut empty_rows = (0..max_x).collect::<Vec<_>>();

    galaxies.iter().for_each(|(y, x)| {
        if let Ok(i) = empty_columns.binary_search(y) {
            empty_columns.remove(i);
        }
        if let Ok(i) = empty_rows.binary_search(x) {
            empty_rows.remove(i);
        }
    });

    let mut moved_galaxies = galaxies.clone();

    for (i, empty_column) in empty_columns.iter().enumerate() {
        moved_galaxies.iter_mut().for_each(|(y, _)| {
            if *y > empty_column + (i * time_warp) {
                *y += time_warp;
            }
        });
    }

    for (i, empty_row) in empty_rows.iter().enumerate() {
        moved_galaxies.iter_mut().for_each(|(_, x)| {
            if *x > empty_row + (i * time_warp) {
                *x += time_warp;
            }
        });
    }

    Ok(moved_galaxies
        .iter()
        .enumerate()
        .map(|(i, galaxy_a)| {
            moved_galaxies
                .iter()
                .skip(i + 1)
                .map(|galaxy_b| manhatten_distance(*galaxy_a, *galaxy_b))
                .sum::<usize>()
        })
        .sum())
}

fn manhatten_distance((a_y, a_x): (usize, usize), (b_y, b_x): (usize, usize)) -> usize {
    (a_y as isize - b_y as isize).unsigned_abs() + (a_x as isize - b_x as isize).unsigned_abs()
}

fn main() {
    Day {}.run(ID);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manhatten_distance() {
        assert_eq!(manhatten_distance((6, 1), (11, 5)), 9);
        assert_eq!(manhatten_distance((0, 4), (10, 9)), 15);
        assert_eq!(manhatten_distance((0, 2), (7, 12)), 17);
        assert_eq!(manhatten_distance((11, 0), (11, 5)), 5);
    }

    #[test]
    fn test_solve_part1() {
        let day = Day {};
        assert_eq!(
            day.parse_and_solve_part1(format!("{}_test1", ID).as_str())
                .unwrap(),
            374
        );
    }

    #[test]
    fn test_solve_part2() {
        let day = Day {};
        let input = day.parse_input(format!("{}_test1", ID).as_str()).unwrap();
        assert_eq!(calulate_distances(&input, 10).unwrap(), 1030);
        assert_eq!(calulate_distances(&input, 100).unwrap(), 8410);
    }
}
