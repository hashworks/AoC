mod util;

use std::error::Error;
use std::{collections::HashMap, collections::HashSet, io::BufRead};
use util::{aoc::AoCDay, input::get_reader};

const ID: &str = "day14";
type Input = Vec<HashSet<usize>>;
type Output = usize;

struct Day {}

impl AoCDay<Input, Output> for Day {
    fn parse_input(&self, id: &str) -> Result<Input, Box<dyn Error>> {
        let mut input = HashMap::new();

        for line in get_reader(id)?.lines() {
            for pairs in line?
                .split(" -> ")
                .map(|xy| {
                    let (x, y) = xy.split_once(',').ok_or("Bad input: Can't split by ,")?;
                    Ok::<_, Box<dyn Error>>((x.parse::<usize>()?, y.parse::<usize>()?))
                })
                .collect::<Result<Vec<_>, _>>()?
                .windows(2)
            {
                let from = pairs[0];
                let to = pairs[1];

                let (from_x, to_x) = if from.0 <= to.0 {
                    (from.0, to.0)
                } else {
                    (to.0, from.0)
                };

                let (from_y, to_y) = if from.1 <= to.1 {
                    (from.1, to.1)
                } else {
                    (to.1, from.1)
                };

                for y in from_y..=to_y {
                    let y_map = input.entry(y).or_insert_with(HashSet::new);
                    for x in from_x..=to_x {
                        y_map.insert(x);
                    }
                }
            }
        }

        let max_y = *input.keys().max().ok_or("Bad input: No max y")?;

        let mut y_vec = Vec::with_capacity(max_y);
        for y in 0..=max_y {
            if let Some(y_set) = input.get(&y) {
                y_vec.push(y_set.clone());
            } else {
                y_vec.push(HashSet::new());
            }
        }

        Ok(y_vec)
    }

    fn part1(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        let mut y_vec = input.clone();
        let max_y = y_vec.len() - 1;

        let mut resting_sand = 0;

        'sandloop: loop {
            let mut x = 500;
            for y in 0.. {
                if y > max_y {
                    break 'sandloop;
                }
                let y_set = &y_vec[y];
                if y_set.contains(&x) {
                    if !y_set.contains(&(x - 1)) {
                        x -= 1;
                    } else if !y_set.contains(&(x + 1)) {
                        x += 1;
                    } else if y == 0 {
                        return Err("Bad input: Sand is stuck".into());
                    } else {
                        y_vec[y - 1].insert(x);
                        resting_sand += 1;
                        break;
                    }
                }
            }
        }

        Ok(resting_sand)
    }

    fn part2(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        let mut y_vec = input.clone();
        let max_y = y_vec.len() - 1;

        let floor = 2 + max_y;
        y_vec.resize(floor, HashSet::new());

        let mut resting_sand = 0;

        'sandloop: loop {
            let mut x = 500;
            for y in 0.. {
                if y == floor {
                    y_vec[y - 1].insert(x);
                    resting_sand += 1;
                    break;
                }
                let y_set = &y_vec[y];
                if y_set.contains(&x) {
                    if y == 0 {
                        break 'sandloop;
                    }
                    if !y_set.contains(&(x - 1)) {
                        x -= 1;
                    } else if !y_set.contains(&(x + 1)) {
                        x += 1;
                    } else {
                        y_vec[y - 1].insert(x);
                        resting_sand += 1;
                        break;
                    }
                }
            }
        }

        Ok(resting_sand)
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
            24
        );
    }

    #[test]
    fn test_solve_part2() {
        let day = Day {};
        assert_eq!(
            day.parse_and_solve_part2(format!("{}_test1", ID).as_str())
                .unwrap(),
            93
        );
    }
}

fn main() {
    Day {}.run(ID);
}
