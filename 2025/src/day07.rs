mod util;

use std::{
    collections::{BTreeMap, BTreeSet, HashSet},
    error::Error,
    io::BufRead,
};
use util::{aoc::AoCDay, input::get_reader};

const ID: &str = "day07";
type Input = (usize, Vec<Vec<usize>>);
type Output = usize;

struct Day {}

impl AoCDay<Input, Output> for Day {
    fn parse_input(&self, id: &str) -> Result<Input, Box<dyn Error>> {
        let mut lines = get_reader(id)?.lines();

        let first_line = lines.next().ok_or("Invalid input: No first line")??;

        let start_pos = first_line
            .find('S')
            .ok_or("Invalid input: No start found")?;

        let mut splitters = vec![vec![]; first_line.len()];

        for (x, l) in lines.enumerate() {
            for (y, _) in l?.chars().enumerate().filter(|(_, c)| *c == '^') {
                splitters[y].push(x + 1);
            }
        }

        Ok((start_pos, splitters))
    }

    fn part1(&self, (start_pos, splitters): &Input) -> Result<Output, Box<dyn Error>> {
        let mut hit_splitters = HashSet::new();
        let mut beams = BTreeSet::from([(0, *start_pos)]);

        while let Some((y, x)) = beams.pop_first() {
            if let Some(&splitter_y) = splitters[x].iter().find(|&&s_y| s_y > y) {
                hit_splitters.insert((splitter_y, x));
                if x < splitters.len() - 1 {
                    beams.insert((splitter_y, x + 1));
                }
                if x >= 1 {
                    beams.insert((splitter_y, x - 1));
                }
            }
        }

        Ok(hit_splitters.len())
    }

    fn part2(&self, (start_pos, splitters): &Input) -> Result<Output, Box<dyn Error>> {
        let mut paths = BTreeMap::from([((0, *start_pos), 1)]);
        let mut finished_paths = 0;

        while let Some(((y, x), n)) = paths.pop_first() {
            if let Some(&splitter_y) = splitters[x].iter().find(|&&s_y| s_y > y) {
                let mut insert_or_inc = |path| {
                    if let Some(m) = paths.get_mut(&path) {
                        *m += n;
                    } else {
                        paths.insert(path, n);
                    }
                };
                if x < splitters.len() - 1 {
                    insert_or_inc((splitter_y, x + 1));
                }
                if x >= 1 {
                    insert_or_inc((splitter_y, x - 1));
                }
            } else {
                finished_paths += n;
            }
        }

        Ok(finished_paths)
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
            21
        );
    }

    #[test]
    fn test_solve_part2() {
        let day = Day {};
        assert_eq!(
            day.parse_and_solve_part2(format!("{}_test1", ID).as_str())
                .unwrap(),
            40
        );
    }
}
