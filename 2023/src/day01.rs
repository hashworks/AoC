mod util;

use std::{error::Error, io::BufRead};
use util::{aoc::AoCDay, input::get_reader};

const ID: &str = "day01";
type Input = Vec<Vec<char>>;
type Output = usize;

struct Day {}

impl AoCDay<Input, Output> for Day {
    fn parse_input(&self, id: &str) -> Result<Input, Box<dyn Error>> {
        let reader = get_reader(id)?;

        let mut output = Vec::new();

        for line in reader.lines() {
            output.push(line?.chars().collect());
        }

        Ok(output)
    }

    fn part1(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        let mut sum = 0;
        for line in input {
            sum += 10
                * line
                    .iter()
                    .filter(|c| c.is_ascii_digit())
                    .map(|c| *c as usize - 48)
                    .next()
                    .unwrap_or(0)
                + line
                    .iter()
                    .filter(|c| c.is_ascii_digit())
                    .map(|c| *c as usize - 48)
                    .last()
                    .unwrap_or(0);
        }
        Ok(sum)
    }

    fn part2(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        let mut sum = 0;

        for line in input {
            let mut parsed_digits = Vec::new();

            let mut i = 0;
            while i < line.len() {
                if line[i].is_ascii_digit() {
                    parsed_digits.push(line[i] as usize - 48);
                } else {
                    if i >= 2 {
                        if line[i - 2..=i] == ['o', 'n', 'e'] {
                            parsed_digits.push(1);
                        } else if line[i - 2..=i] == ['t', 'w', 'o'] {
                            parsed_digits.push(2);
                        } else if line[i - 2..=i] == ['s', 'i', 'x'] {
                            parsed_digits.push(6);
                        }
                    }
                    if i >= 3 {
                        if line[i - 3..=i] == ['f', 'o', 'u', 'r'] {
                            parsed_digits.push(4);
                        } else if line[i - 3..=i] == ['f', 'i', 'v', 'e'] {
                            parsed_digits.push(5);
                        } else if line[i - 3..=i] == ['n', 'i', 'n', 'e'] {
                            parsed_digits.push(9);
                        }
                    }
                    if i >= 4 {
                        if line[i - 4..=i] == ['t', 'h', 'r', 'e', 'e'] {
                            parsed_digits.push(3);
                        } else if line[i - 4..=i] == ['s', 'e', 'v', 'e', 'n'] {
                            parsed_digits.push(7);
                        } else if line[i - 4..=i] == ['e', 'i', 'g', 'h', 't'] {
                            parsed_digits.push(8);
                        }
                    }
                }
                i += 1;
            }

            sum += 10 * parsed_digits[0] + parsed_digits.last().unwrap_or(&0);
        }

        Ok(sum)
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
            142
        );
    }

    #[test]
    fn test_solve_part2() {
        let day = Day {};
        assert_eq!(
            day.parse_and_solve_part2(format!("{}_test2", ID).as_str())
                .unwrap(),
            281
        );
    }
}
