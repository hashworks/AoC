mod util;

use std::collections::HashMap;
use std::{error::Error, io::BufRead};
use util::{aoc::AoCDay, input::get_reader};

const ID: &str = "day03";
type Input = Vec<Vec<char>>;
type Output = usize;

struct Day {}

impl AoCDay<Input, Output> for Day {
    fn parse_input(&self, id: &str) -> Result<Input, Box<dyn Error>> {
        let reader = get_reader(id);

        let mut input = Vec::new();
        for line in reader?.lines() {
            let line = line?;
            input.push(line.chars().collect());
        }

        Ok(input)
    }

    fn part1(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        let mut part_numbers = Vec::new();

        for (y, line) in input.iter().enumerate() {
            let mut incomplete_number = Vec::new();
            for (x, c) in line.iter().enumerate() {
                let (x, end_of_line) = if c.is_ascii_digit() {
                    incomplete_number.push(*c);

                    // Edge case: We are at the end of the line, so we asume we are in the next column
                    if x + 1 == line.len() {
                        (x + 1, true)
                    } else {
                        (x, false)
                    }
                } else {
                    (x, false)
                };

                if !incomplete_number.is_empty() && (!c.is_ascii_digit() || end_of_line) {
                    // We could calculate this, but I'm lazy
                    let number = incomplete_number
                        .iter()
                        .collect::<String>()
                        .parse::<usize>()
                        .unwrap();

                    // Check left and right of number
                    if (x - incomplete_number.len() > 0
                        && input[y]
                            .get(x - incomplete_number.len() - 1)
                            .unwrap_or(&'.')
                            != &'.')
                        || input[y].get(x).unwrap_or(&'.') != &'.'
                    {
                        part_numbers.push(number);
                    } else {
                        let left = if x - incomplete_number.len() > 0 {
                            x - incomplete_number.len() - 1
                        } else {
                            x - incomplete_number.len()
                        };

                        let right = if x == line.len() { x - 1 } else { x };

                        // Check over number
                        if (y > 0
                            && input[y - 1][left..=right]
                                .iter()
                                .any(|c| c != &'.' && !c.is_ascii_digit()))
                            || (y + 1 < input.len()
                                && input[y + 1][left..=right]
                                    .iter()
                                    .any(|c| c != &'.' && !c.is_ascii_digit()))
                        {
                            part_numbers.push(number);
                        }
                    }

                    incomplete_number.clear();
                }
            }
        }

        Ok(part_numbers.iter().sum())
    }

    fn part2(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        let mut gear_map = HashMap::new();
        let mut gear_sum = 0;

        for (y, line) in input.iter().enumerate() {
            let mut incomplete_number = Vec::new();
            for (x, c) in line.iter().enumerate() {
                let (x, end_of_line) = if c.is_ascii_digit() {
                    incomplete_number.push(*c);

                    // Edge case: We are at the end of the line, so we asume we are in the next column
                    if x + 1 == line.len() {
                        (x + 1, true)
                    } else {
                        (x, false)
                    }
                } else {
                    (x, false)
                };

                if !incomplete_number.is_empty() && (!c.is_ascii_digit() || end_of_line) {
                    // We could calculate this, but I'm lazy
                    let number = incomplete_number
                        .iter()
                        .collect::<String>()
                        .parse::<usize>()
                        .unwrap();

                    // Check left of number
                    if x - incomplete_number.len() > 0
                        && input[y]
                            .get(x - incomplete_number.len() - 1)
                            .unwrap_or(&'.')
                            == &'*'
                    {
                        if let Some(other) =
                            gear_map.insert((x - incomplete_number.len() - 1, y), number)
                        {
                            gear_sum += number * other;
                        }
                    // Check right of number
                    } else if input[y].get(x).unwrap_or(&'.') == &'*' {
                        if let Some(other) = gear_map.insert((x, y), number) {
                            gear_sum += number * other;
                        }
                    } else {
                        let left = if x - incomplete_number.len() > 0 {
                            x - incomplete_number.len() - 1
                        } else {
                            x - incomplete_number.len()
                        };

                        let right = if x == line.len() { x - 1 } else { x };

                        // Check over number
                        if y > 0 && input[y - 1][left..=right].iter().any(|c| c == &'*') {
                            let star_x = left
                                + input[y - 1][left..=right]
                                    .iter()
                                    .enumerate()
                                    .filter(|(_, c)| c == &&'*')
                                    .map(|(i, _)| i)
                                    .next()
                                    .unwrap();
                            if let Some(other) = gear_map.insert((star_x, y - 1), number) {
                                gear_sum += number * other;
                            }

                        // Check below number
                        } else if y + 1 < input.len()
                            && input[y + 1][left..=right].iter().any(|c| c == &'*')
                        {
                            let star_x = left
                                + input[y + 1][left..=right]
                                    .iter()
                                    .enumerate()
                                    .filter(|(_, c)| c == &&'*')
                                    .map(|(i, _)| i)
                                    .next()
                                    .unwrap();
                            if let Some(other) = gear_map.insert((star_x, y + 1), number) {
                                gear_sum += number * other;
                            }
                        }
                    }

                    incomplete_number.clear();
                }
            }
        }

        Ok(gear_sum)
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
            4361
        );
    }

    #[test]
    fn test_solve_part2() {
        let day = Day {};
        assert_eq!(
            day.parse_and_solve_part2(format!("{}_test1", ID).as_str())
                .unwrap(),
            467835
        );
    }
}
