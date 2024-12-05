mod util;

use std::{collections::HashSet, error::Error, io::BufRead};
use util::{aoc::AoCDay, input::get_reader};

const ID: &str = "day05";
type Input = (HashSet<(usize, usize)>, Vec<Vec<usize>>);
type Output = usize;

struct Day {}

impl AoCDay<Input, Output> for Day {
    fn parse_input(&self, id: &str) -> Result<Input, Box<dyn Error>> {
        let reader = get_reader(id)?;

        let lines: Vec<_> = reader.lines().map_while(Result::ok).collect();

        let before_map = lines
            .iter()
            .take_while(|l| !l.is_empty())
            .map(|l| {
                let (a, b) = l.split_once('|').ok_or("Invalid input")?;
                Ok((a.parse::<usize>()?, b.parse::<usize>()?))
            })
            .collect::<Result<HashSet<(_, _)>, Box<dyn Error>>>()?;

        let updates = lines
            .iter()
            .skip_while(|l| !l.is_empty())
            .skip(1)
            .map(|l| {
                l.split(',')
                    .map(|s| s.parse::<usize>())
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok((before_map, updates))
    }

    fn part1(&self, (rules, updates): &Input) -> Result<Output, Box<dyn Error>> {
        Ok(updates
            .iter()
            .filter(|&pages| {
                let mut sorted_pages = pages.clone();
                sorted_pages.sort_by(|a, b| {
                    if rules.contains(&(*a, *b)) {
                        std::cmp::Ordering::Less
                    } else {
                        std::cmp::Ordering::Greater
                    }
                });
                pages == &sorted_pages
            })
            .map(|pages| pages[pages.len() / 2])
            .sum())
    }

    fn part2(&self, (rules, updates): &Input) -> Result<Output, Box<dyn Error>> {
        Ok(updates
            .iter()
            .filter_map(|pages| {
                let mut sorted_pages = pages.clone();
                sorted_pages.sort_by(|a, b| {
                    if rules.contains(&(*a, *b)) {
                        std::cmp::Ordering::Less
                    } else {
                        std::cmp::Ordering::Greater
                    }
                });
                if pages == &sorted_pages {
                    None
                } else {
                    Some(sorted_pages)
                }
            })
            .map(|pages| pages[pages.len() / 2])
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
        assert_eq!(
            day.parse_and_solve_part1(format!("{}_test1", ID).as_str())
                .unwrap(),
            143
        );
    }

    #[test]
    fn test_solve_part2() {
        let day = Day {};
        assert_eq!(
            day.parse_and_solve_part2(format!("{}_test1", ID).as_str())
                .unwrap(),
            123
        );
    }
}
