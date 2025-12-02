mod util;

use std::{error::Error, io::BufRead, ops::RangeInclusive};
use util::{aoc::AoCDay, input::get_reader};

const ID: &str = "day02";
type Input = Vec<RangeInclusive<usize>>;
type Output = usize;

struct Day {}

impl AoCDay<Input, Output> for Day {
    fn parse_input(&self, id: &str) -> Result<Input, Box<dyn Error>> {
        get_reader(id)?
            .lines()
            .next()
            .ok_or_else(|| Box::<dyn Error>::from("No line found in input"))??
            .split(',')
            .into_iter()
            .filter_map(|s| s.split_once('-'))
            .map(|(l, r)| {
                let l = l.parse()?;
                let r = r.parse()?;
                Ok(l..=r)
            })
            .collect()
    }

    fn part1(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        Ok(input.iter().fold(0, |sum, range| {
            range.clone().fold(sum, |sum, i| {
                let str = i.to_string();
                let (l, r) = str.split_at(str.len() / 2);
                if l == r { sum + i } else { sum }
            })
        }))
    }

    fn part2(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        Ok(input.iter().fold(0, |sum, range| {
            range.clone().fold(sum, |sum, i| {
                let chars = i.to_string().chars().collect::<Vec<_>>();
                for point in 1..=chars.len() / 2 {
                    let mut chunks = chars.chunks(point);
                    if let Some(first_chunk) = chunks.next() {
                        if chunks.all(|chunk| chunk == first_chunk) {
                            return sum + i;
                        }
                    }
                }
                sum
            })
        }))
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
            1227775554
        );
    }

    #[test]
    fn test_solve_part2() {
        let day = Day {};
        assert_eq!(
            day.parse_and_solve_part2(format!("{}_test1", ID).as_str())
                .unwrap(),
            4174379265
        );
    }
}
