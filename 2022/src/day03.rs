mod util;

use std::{error::Error, io::BufRead};
use util::{aoc::AoCDay, input::get_reader};

const ID: &str = "day03";
type Input = Vec<Vec<u8>>;
type Output = usize;

struct Day {}

impl AoCDay<Input, Output> for Day {
    fn parse_input(&self, id: &str) -> Result<Input, Box<dyn Error>> {
        let reader = get_reader(id)?;
        let items: Input = reader
            .lines()
            .map(|l| Ok(l?.bytes().collect()))
            .collect::<Result<_, Box<dyn Error>>>()?;

        Ok(items)
    }

    fn part1(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        input
            .iter()
            .map(|items| {
                let (lh, rh) = items.split_at(items.len() / 2);
                lh.iter()
                    .find(|item| rh.contains(item))
                    .map(item_priority)
                    .ok_or("no item found".into())
            })
            .sum()
    }

    fn part2(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        input
            .chunks_exact(3)
            .map(|items| {
                items[0]
                    .iter()
                    .find(|item| items[1].contains(item) && items[2].contains(item))
                    .map(item_priority)
                    .ok_or("no item found".into())
            })
            .sum()
    }
}

fn item_priority(char: &u8) -> usize {
    (if char.is_ascii_lowercase() {
        char - b'a' + 1
    } else {
        char - b'A' + 1 + 26
    }) as usize
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
            157
        );
    }

    #[test]
    fn test_solve_part2() {
        let day = Day {};
        assert_eq!(
            day.parse_and_solve_part2(format!("{}_test1", ID).as_str())
                .unwrap(),
            70
        );
    }
}

fn main() {
    Day {}.run(ID);
}
