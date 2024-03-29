mod util;

use nom::{branch::alt, character::complete, multi::separated_list0, sequence::tuple, IResult};
use std::{cmp::Ordering, error::Error, io::BufRead};
use util::{aoc::AoCDay, input::get_reader};

const ID: &str = "day13";
type Input = Vec<(ListOrInt, ListOrInt)>;
type Output = usize;

#[derive(Debug, Clone, PartialEq, Eq)]
enum ListOrInt {
    List(Vec<ListOrInt>),
    Int(u8),
}

impl PartialOrd for ListOrInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (ListOrInt::Int(a), ListOrInt::Int(b)) => a.partial_cmp(b),
            (ListOrInt::List(a), ListOrInt::List(b)) => a.partial_cmp(b),
            (ListOrInt::Int(_), ListOrInt::List(_)) => {
                ListOrInt::List(vec![self.clone()]).partial_cmp(other)
            }
            (ListOrInt::List(_), ListOrInt::Int(_)) => {
                self.partial_cmp(&ListOrInt::List(vec![other.clone()]))
            }
        }
    }
}

impl Ord for ListOrInt {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn parse_int(i: &str) -> IResult<&str, ListOrInt> {
    complete::u8(i).map(|(i, o)| (i, ListOrInt::Int(o)))
}

fn parse_list(i: &str) -> IResult<&str, ListOrInt> {
    tuple((
        complete::char('['),
        separated_list0(complete::char(','), parse_list_or_int),
        complete::char(']'),
    ))(i)
    .map(|(i, (_, o, _))| (i, ListOrInt::List(o)))
}

// This can be parsed using serde_json, but I wanted to try nom
fn parse_list_or_int(i: &str) -> IResult<&str, ListOrInt> {
    alt((parse_list, parse_int))(i)
}

struct Day {}

impl AoCDay<Input, Output> for Day {
    fn parse_input(&self, id: &str) -> Result<Input, Box<dyn Error>> {
        get_reader(id)?
            .lines()
            .filter_map(|x| x.ok())
            .collect::<Vec<_>>()
            .chunks(3)
            .map(|pairs| {
                let left = match parse_list_or_int(&pairs[0]) {
                    Ok((_, o)) => o,
                    Err(e) => Err(format!("Parse error: {:?}", e))?,
                };
                let right = match parse_list_or_int(&pairs[1]) {
                    Ok((_, o)) => o,
                    Err(e) => Err(format!("Parse error: {:?}", e))?,
                };
                Ok((left, right))
            })
            .collect()
    }

    fn part1(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        Ok(input
            .iter()
            .enumerate()
            .filter(|(_, (left, right))| left < right)
            .map(|(i, _)| i + 1)
            .sum())
    }

    fn part2(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        let dividers = [
            ListOrInt::List(vec![ListOrInt::Int(2)]),
            ListOrInt::List(vec![ListOrInt::Int(6)]),
        ];

        let mut all = input
            .iter()
            .flat_map(|(left, right)| [left, right])
            .chain(dividers.iter())
            .collect::<Vec<_>>();

        all.sort();

        Ok(dividers
            .iter()
            .map(|x| all.binary_search(&x).expect("assertion: contains x") + 1)
            .product())
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
            13
        );
    }

    #[test]
    fn test_solve_part2() {
        let day = Day {};
        assert_eq!(
            day.parse_and_solve_part2(format!("{}_test1", ID).as_str())
                .unwrap(),
            140
        );
    }
}

fn main() {
    Day {}.run(ID);
}
