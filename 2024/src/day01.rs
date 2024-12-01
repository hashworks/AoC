mod util;

use std::{collections::HashMap, error::Error, io::BufRead};
use util::{aoc::AoCDay, input::get_reader};

const ID: &str = "day01";
type Input = (Vec<isize>, Vec<isize>);
type Output = isize;

struct Day {}

impl AoCDay<Input, Output> for Day {
    fn parse_input(&self, id: &str) -> Result<Input, Box<dyn Error>> {
        let reader = get_reader(id)?;

        let mut list1 = Vec::new();
        let mut list2 = Vec::new();

        for line in reader.lines() {
            let line = line?;
            let whitespace_split = line.split_whitespace().collect::<Vec<&str>>();

            let num1 = whitespace_split[0].parse::<isize>()?;
            let num2 = whitespace_split[1].parse::<isize>()?;

            list1.push(num1);
            list2.push(num2);
        }

        Ok((list1, list2))
    }

    fn part1(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        let mut list1 = input.0.clone();
        let mut list2 = input.1.clone();

        list1.sort();
        list2.sort();

        Ok(list1
            .iter()
            .zip(list2.iter())
            .fold(0, |acc, (num1, num2)| acc + (num1 - num2).abs()))
    }

    fn part2(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        let mut dict = HashMap::new();

        input.1.iter().for_each(|num| {
            *(dict.entry(*num).or_insert(0)) += 1;
        });

        Ok(input
            .0
            .iter()
            .fold(0, |acc, num1| acc + num1 * dict.get(num1).unwrap_or(&0)))
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
            11
        );
    }

    #[test]
    fn test_solve_part2() {
        let day = Day {};
        assert_eq!(
            day.parse_and_solve_part2(format!("{}_test1", ID).as_str())
                .unwrap(),
            31
        );
    }
}
