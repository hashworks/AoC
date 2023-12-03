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
                    incomplete_number.push(*c as usize - 48);

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
                    let number = digit_slice_to_number(&incomplete_number);

                    // Check left and right of number
                    if (x - incomplete_number.len() > 0
                        && input[y][x - incomplete_number.len() - 1] != '.')
                        || input[y].get(x).unwrap_or(&'.') != &'.'
                    {
                        part_numbers.push(number);
                    } else {
                        let left = x - incomplete_number.len();
                        let left = if left > 0 { left - 1 } else { left };

                        let right = if x == line.len() { x - 1 } else { x };

                        // Check over and under number
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
                    incomplete_number.push(*c as usize - 48);

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
                    let number = digit_slice_to_number(&incomplete_number);

                    // Check left of number
                    if x - incomplete_number.len() > 0
                        && input[y][x - incomplete_number.len() - 1] == '*'
                    {
                        gear_sum += number
                            * gear_map
                                .insert((x - incomplete_number.len() - 1, y), number)
                                .unwrap_or(0);
                    // Check right of number
                    } else if input[y].get(x).unwrap_or(&'.') == &'*' {
                        gear_sum += number * gear_map.insert((x, y), number).unwrap_or(0);
                    } else {
                        let left = x - incomplete_number.len();
                        let left = if left > 0 { left - 1 } else { left };

                        let right = if x == line.len() { x - 1 } else { x };

                        // Check over number
                        if y > 0 && input[y - 1][left..=right].iter().any(|c| c == &'*') {
                            let star_x =
                                left + get_star_position(&input[y - 1][left..=right]).unwrap();
                            gear_sum +=
                                number * gear_map.insert((star_x, y - 1), number).unwrap_or(0);

                        // Check below number
                        } else if y + 1 < input.len()
                            && input[y + 1][left..=right].iter().any(|c| c == &'*')
                        {
                            let star_x =
                                left + get_star_position(&input[y + 1][left..=right]).unwrap();
                            gear_sum +=
                                number * gear_map.insert((star_x, y + 1), number).unwrap_or(0);
                        }
                    }

                    incomplete_number.clear();
                }
            }
        }

        Ok(gear_sum)
    }
}

fn get_star_position(slice: &[char]) -> Option<usize> {
    slice
        .iter()
        .enumerate()
        .filter(|(_, c)| c == &&'*')
        .map(|(i, _)| i)
        .next()
}

fn digit_slice_to_number(slice: &[usize]) -> usize {
    slice
        .iter()
        .rev()
        .enumerate()
        .map(|(i, d)| d * 10usize.pow(i as u32))
        .sum()
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

    #[test]
    fn test_digit_list_to_number() {
        assert_eq!(digit_slice_to_number(&[1]), 1);
        assert_eq!(digit_slice_to_number(&[1, 2]), 12);
        assert_eq!(digit_slice_to_number(&[1, 2, 3]), 123);
    }

    #[test]
    fn test_get_star_position() {
        assert_eq!(get_star_position(&[]), None);
        assert_eq!(get_star_position(&['a']), None);
        assert_eq!(get_star_position(&['*']), Some(0));
        assert_eq!(get_star_position(&['a', '*']), Some(1));
        assert_eq!(get_star_position(&['a', '*', 'b']), Some(1));
    }
}
