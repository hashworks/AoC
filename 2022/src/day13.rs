mod util;

use nom::{character::complete, error::ErrorKind, multi::separated_list1, IResult};
use std::{error::Error, io::BufRead};
use util::{aoc::AoCDay, input::get_reader};

const ID: &str = "day13";
type Input = Vec<(ListOrInt, ListOrInt)>;
type Output = usize;

#[derive(Debug, Clone)]
enum ListOrInt {
    List(Vec<ListOrInt>),
    Int(u8),
}

impl PartialEq for ListOrInt {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ListOrInt::Int(a), ListOrInt::Int(b)) => a == b,
            (ListOrInt::List(a), ListOrInt::List(b)) => a == b,
            _ => false,
        }
    }
}

impl PartialOrd for ListOrInt {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
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

fn parse_list(i: &str) -> IResult<&str, ListOrInt, (&str, ErrorKind)> {
    if let Ok((i, _)) = complete::char::<_, nom::error::Error<_>>('[')(i) {
        let (i, o) = separated_list1(complete::char(','), parse_list)(i)?;
        let (i, _) = complete::char(']')(i)?;

        Ok((i, ListOrInt::List(o)))
    } else if let Ok((i, o)) = complete::u8::<_, nom::error::Error<_>>(i) {
        Ok((i, ListOrInt::Int(o)))
    } else {
        Ok((i, ListOrInt::List(vec![])))
    }
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
                let left = match parse_list(&pairs[0]) {
                    Ok((_, o)) => o,
                    Err(e) => Err(format!("Parse error: {:?}", e))?,
                };
                let right = match parse_list(&pairs[1]) {
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
        let mut all = input
            .iter()
            .flat_map(|(left, right)| vec![left, right])
            .collect::<Vec<_>>();

        let divider_packet_1 = ListOrInt::List(vec![ListOrInt::Int(2)]);
        let divider_packet_2 = ListOrInt::List(vec![ListOrInt::Int(6)]);
        all.push(&divider_packet_1);
        all.push(&divider_packet_2);

        all.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Less));

        Ok(all
            .iter()
            .enumerate()
            .filter(|&(_, &packet)| packet == &divider_packet_1 || packet == &divider_packet_2)
            .take(2)
            .map(|(i, _)| i + 1)
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