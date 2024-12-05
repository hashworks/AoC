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
            .filter(|pages| validate(pages, rules))
            .map(|pages| pages[pages.len() / 2])
            .sum())
    }

    fn part2(&self, (rules, updates): &Input) -> Result<Output, Box<dyn Error>> {
        Ok(updates
            .iter()
            .filter(|pages| !validate(pages, rules))
            .map(|pages| {
                let mut pages = pages.clone();
                pages.sort_by(|a, b| {
                    if rules.contains(&(*a, *b)) {
                        std::cmp::Ordering::Less
                    } else {
                        std::cmp::Ordering::Greater
                    }
                });
                pages
            })
            .map(|pages| pages[pages.len() / 2])
            .sum())
    }
}

fn validate(pages: &[usize], rules: &HashSet<(usize, usize)>) -> bool {
    pages.iter().enumerate().all(|(i, page)| {
        if i == 0 || i == pages.len() - 1 {
            true
        } else {
            let prev_slice = &pages[0..i];
            let after_slice = &pages[i + 1..];
            prev_slice
                .iter()
                .all(|prev_page| rules.contains(&(*prev_page, *page)))
                && after_slice
                    .iter()
                    .all(|after_page| rules.contains(&(*page, *after_page)))
        }
    })
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
