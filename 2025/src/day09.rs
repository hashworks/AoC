mod util;

use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
use std::{collections::HashSet, error::Error, io::BufRead};
use util::{aoc::AoCDay, input::get_reader};

const ID: &str = "day09";
type Input = Vec<(isize, isize)>;
type Output = isize;

struct Day {}

impl AoCDay<Input, Output> for Day {
    fn parse_input(&self, id: &str) -> Result<Input, Box<dyn Error>> {
        get_reader(id)?
            .lines()
            .map(|l| {
                let l = l?;
                let (y, x) = l.split_once(',').ok_or("Invalid input: failed to split")?;
                let y = y.parse()?;
                let x = x.parse()?;
                Ok((y, x))
            })
            .collect()
    }

    fn part1(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        input
            .iter()
            .enumerate()
            .map(|(i, (y1, x1))| {
                input
                    .iter()
                    .skip(i)
                    .map(|(y2, x2)| ((y2 - y1).abs() + 1) * ((x2 - x1).abs() + 1))
                    .max()
                    .unwrap_or(0)
            })
            .max()
            .ok_or("No max found".into())
    }

    fn part2(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        !unimplemented!("Not correct");

        let mut max_y = 0;
        let mut max_x = 0;

        let border_points = input
            .windows(2)
            .flat_map(|pairs| {
                let y1 = pairs[0].0;
                let x1 = pairs[0].1;
                let y2 = pairs[1].0;
                let x2 = pairs[1].1;

                max_y = max_y.max(y1);
                max_y = max_y.max(y2);
                max_x = max_x.max(x1);
                max_x = max_x.max(x2);

                if y1 == y2 {
                    (x1.min(x2)..=x2.max(x1))
                        .map(|x| (y1, x))
                        .collect::<Vec<_>>()
                } else {
                    (y1.min(y2)..=y2.max(y1)).map(|y| (y, x1)).collect()
                }
            })
            .collect::<HashSet<(isize, isize)>>();

        max_y += 1;
        max_x += 1;

        input
            .iter()
            .enumerate()
            .map(|(i, (y1, x1))| {
                //println!("Calculating input #{}", i);
                input
                    .par_iter()
                    .skip(i)
                    .filter(|(y2, x2)| {
                        // check if inside of borders by calulating other two corners of rect
                        // walk corners north, east, south, west
                        // if we hit a position in the hashset the corner is inside
                        // if we go outside of the max the corner is outside
                        // if all corners are inside, continue

                        let c3 = (y1.abs(), x2.abs());
                        let c4 = (y2.abs(), x1.abs());

                        for (y, x) in [c3, c4] {
                            // north
                            for walking_y in (0..=y).rev() {
                                if walking_y == 0 {
                                    return false;
                                }
                                if border_points.contains(&(walking_y, x)) {
                                    break;
                                }
                            }
                            // south
                            for walking_y in y..=max_y {
                                if walking_y == max_y {
                                    return false;
                                }
                                if border_points.contains(&(walking_y, x)) {
                                    break;
                                }
                            }
                            // west
                            for walking_x in (0..=x).rev() {
                                if walking_x == 0 {
                                    return false;
                                }
                                if border_points.contains(&(y, walking_x)) {
                                    break;
                                }
                            }
                            // east
                            for walking_x in x..=max_x {
                                if walking_x == max_x {
                                    return false;
                                }
                                if border_points.contains(&(y, walking_x)) {
                                    break;
                                }
                            }
                        }

                        true
                    })
                    .map(|(y2, x2)| ((y2 - y1).abs() + 1) * ((x2 - x1).abs() + 1))
                    .max()
                    .unwrap_or(0)
            })
            .max()
            .ok_or("No max found".into())
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
            50
        );
    }

    #[test]
    fn test_solve_part2() {
        let day = Day {};
        assert_eq!(
            day.parse_and_solve_part2(format!("{}_test1", ID).as_str())
                .unwrap(),
            24
        );
    }
}
