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
            .iter()
            .filter(|(a, b, c, d)| {
                c >= a && c <= b && d >= a && d <= b || // (c,d) is inside (a,b)
                a >= c && a <= d && b >= c && b <= d // (a,b) is inside (c,d)
            })
            .count()
    }

    fn part2(&self, input: &Input) -> Output {
        input
            .iter()
            .filter(|(a, b, c, d)| {
                c >= a && c <= b || // c is inside (a,b)
                d >= a && d <= b || // d is inside (a,b)
                a >= c && a <= d || // a is inside (c,d)
                b >= c && b <= d // b is inside (c,d)
            })
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
