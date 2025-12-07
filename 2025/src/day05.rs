mod util;

use std::{error::Error, io::BufRead, ops::RangeInclusive};
use util::{aoc::AoCDay, input::get_reader};

const ID: &str = "day05";
type Input = (Vec<RangeInclusive<usize>>, Vec<usize>);
type Output = usize;

struct Day {}

impl AoCDay<Input, Output> for Day {
    fn parse_input(&self, id: &str) -> Result<Input, Box<dyn Error>> {
        let lines = get_reader(id)?.lines().collect::<Result<Vec<_>, _>>()?;

        let ranges = lines
            .iter()
            .take_while(|l| !l.is_empty())
            .map(|l| {
                let (l, r) = l.split_once('-').ok_or("Invalid input: Expected range")?;
                let l = l.parse()?;
                let r = r.parse()?;
                Ok::<_, Box<dyn Error>>(l..=r)
            })
            .collect::<Result<Vec<_>, _>>()?;

        let incredients = lines
            .iter()
            .skip_while(|l| !l.is_empty())
            .skip(1)
            .map(|l| l.parse())
            .collect::<Result<Vec<_>, _>>()?;

        Ok((ranges, incredients))
    }

    fn part1(&self, (ranges, ingredients): &Input) -> Result<Output, Box<dyn Error>> {
        Ok(ingredients
            .iter()
            .filter(|i| ranges.iter().any(|r| r.contains(i)))
            .count())
    }

    fn part2(&self, (ranges, _): &Input) -> Result<Output, Box<dyn Error>> {
        let merged_ranges = merge_ranges(&mut ranges.clone());

        Ok(merged_ranges.iter().map(|r| r.end() + 1 - r.start()).sum())
    }
}

fn merge_ranges<T: Ord + Clone + Copy>(
    ranges: &mut Vec<RangeInclusive<T>>,
) -> Vec<RangeInclusive<T>> {
    ranges.sort_by(|a, b| a.start().cmp(b.start()));
    let mut merged_ranges: Vec<RangeInclusive<T>> = vec![];
    'rangeLoop: for range in ranges {
        let mut i = 0;
        while i < merged_ranges.len() {
            let (ex_start, ex_end) = (*merged_ranges[i].start(), *merged_ranges[i].start());
            let (new_start, new_end) = (*range.start(), *range.end());
            if merged_ranges[i].contains(&new_start) && merged_ranges[i].contains(&new_end) {
                // Existing range already contains new range
                continue 'rangeLoop;
            }
            if range.contains(&ex_start) && range.contains(&ex_end) {
                // New range contains existing range
                merged_ranges[i] = new_start..=new_end;
                continue 'rangeLoop;
            }
            if merged_ranges[i].contains(&new_start) && !merged_ranges[i].contains(&new_end) {
                // Existing range can be extended at the end
                merged_ranges[i] = ex_start..=new_end;
                continue 'rangeLoop;
            }
            if !merged_ranges[i].contains(&new_start) && merged_ranges[i].contains(&new_end) {
                // Existing range can be extended at the start
                merged_ranges[i] = new_start..=ex_end;
                continue 'rangeLoop;
            }
            i += 1;
        }
        // Completely new range, add it as is
        merged_ranges.push(range.clone());
    }
    merged_ranges
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
            3
        );
    }

    #[test]
    fn test_solve_part2() {
        let day = Day {};
        assert_eq!(
            day.parse_and_solve_part2(format!("{}_test1", ID).as_str())
                .unwrap(),
            14
        );
    }
}
