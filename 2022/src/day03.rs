mod util;

use std::{collections::HashSet, error::Error, io::BufRead};
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

    fn part1(&self, input: &Input) -> Output {
        input
            .iter()
            .map(|items| {
                let (lh, rh) = items.split_at(items.len() / 2);
                let lh: HashSet<_> = lh.iter().copied().collect();
                let rh: HashSet<_> = rh.iter().copied().collect();
                let item = lh.intersection(&rh).next().unwrap_or(&b'a');
                item_priority(item)
            })
            .sum()
    }

    fn part2(&self, input: &Input) -> Output {
        input
            .chunks_exact(3)
            .map(|items| {
                let first: HashSet<_> = items[0].iter().copied().collect();
                let second: HashSet<_> = items[1].iter().copied().collect();
                let third: HashSet<_> = items[2].iter().copied().collect();

                let intersections = &(&first & &second) & &third;
                let badge = intersections.iter().next().unwrap_or(&b'a');
                item_priority(badge)
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
