mod util;

use std::{error::Error, io::BufRead};
use util::{aoc::AoCDay, input::get_reader};

const ID: &str = "day04";
type Input = Vec<(usize, usize, usize, usize)>;
type Output = usize;

struct Day {}

impl AoCDay<Input, Output> for Day {
    fn parse_input(&self, id: &str) -> Result<Input, Box<dyn Error>> {
        let reader = get_reader(id)?;
        reader
            .lines()
            .map(|line| {
                let line = line?;
                let (l, r) = line.split_once(',').ok_or("no ',' in line")?;
                let (a, b) = l.split_once('-').ok_or("no '-' in left part")?;
                let (c, d) = r.split_once('-').ok_or("no '-' in right part")?;
                Ok((a.parse()?, b.parse()?, c.parse()?, d.parse()?))
            })
            .collect::<Result<_, Box<dyn Error>>>()
    }

    fn part1(&self, input: &Input) -> Output {
        input
            .iter() // The derefence is important here https://github.com/rust-lang/rust/issues/105259
            .filter(|&&(a, b, c, d)| a <= c && d <= b || c <= a && b <= d)
            .count()
    }

    fn part2(&self, input: &Input) -> Output {
        input
            .iter()
            .filter(|&&(a, b, c, d)| b >= c && d >= a)
            .count()
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
            2
        );
    }

    #[test]
    fn test_solve_part2() {
        let day = Day {};
        assert_eq!(
            day.parse_and_solve_part2(format!("{}_test1", ID).as_str())
                .unwrap(),
            4
        );
    }
}

fn main() {
    Day {}.run(ID);
}
