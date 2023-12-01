mod util;

use std::{error::Error, io::BufRead};
use util::{aoc::AoCDay, input::get_reader};

const ID: &str = "day01";
type Input = Vec<Vec<char>>;
type Output = u64;

struct Day {}

impl AoCDay<Input, Output> for Day {
    fn parse_input(&self, _id: &str) -> Result<Input, Box<dyn Error>> {
        let reader = get_reader(_id)?;

        let mut output = Vec::new();

        for line in reader.lines() {
            output.push(line?.chars().collect());
        }

        Ok(output)
    }

    fn part1(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        let mut digits = Vec::new();
        for line in input {
            let mut chars = Vec::new();
            for c in line {
                if c.is_ascii_digit() {
                    chars.push(c);
                    break;
                }
            }
            for c in line.iter().rev() {
                if c.is_ascii_digit() {
                    chars.push(c);
                    break;
                }
            }
            digits.push(chars.iter().cloned().collect::<String>().parse()?);
        }

        Ok(digits.iter().sum())
    }

    fn part2(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        let mut digits = Vec::new();
        for line in input {
            let mut parsed_line = Vec::new();

            let mut i = 0;
            while i < line.len() {
                if line[i].is_ascii_digit() {
                    parsed_line.push(line[i]);
                } else {
                    if i >= 2 {
                        if line[i - 2] == 'o' && line[i - 1] == 'n' && line[i] == 'e' {
                            parsed_line.push('1');
                        } else if line[i - 2] == 't' && line[i - 1] == 'w' && line[i] == 'o' {
                            parsed_line.push('2');
                        } else if line[i - 2] == 's' && line[i - 1] == 'i' && line[i] == 'x' {
                            parsed_line.push('6');
                        }
                    }
                    if i >= 3 {
                        if line[i - 3] == 'f'
                            && line[i - 2] == 'o'
                            && line[i - 1] == 'u'
                            && line[i] == 'r'
                        {
                            parsed_line.push('4');
                        }
                        if line[i - 3] == 'f'
                            && line[i - 2] == 'i'
                            && line[i - 1] == 'v'
                            && line[i] == 'e'
                        {
                            parsed_line.push('5');
                        }
                        if line[i - 3] == 'n'
                            && line[i - 2] == 'i'
                            && line[i - 1] == 'n'
                            && line[i] == 'e'
                        {
                            parsed_line.push('9');
                        }
                    }
                    if i >= 4 {
                        if line[i - 4] == 't'
                            && line[i - 3] == 'h'
                            && line[i - 2] == 'r'
                            && line[i - 1] == 'e'
                            && line[i] == 'e'
                        {
                            parsed_line.push('3');
                        } else if line[i - 4] == 's'
                            && line[i - 3] == 'e'
                            && line[i - 2] == 'v'
                            && line[i - 1] == 'e'
                            && line[i] == 'n'
                        {
                            parsed_line.push('7');
                        } else if line[i - 4] == 'e'
                            && line[i - 3] == 'i'
                            && line[i - 2] == 'g'
                            && line[i - 1] == 'h'
                            && line[i] == 't'
                        {
                            parsed_line.push('8');
                        }
                    }
                }

                i += 1;
            }

            let mut chars = Vec::new();
            for c in &parsed_line {
                if c.is_ascii_digit() {
                    chars.push(*c);
                    break;
                }
            }
            for c in parsed_line.iter().rev() {
                if c.is_ascii_digit() {
                    chars.push(*c);
                    break;
                }
            }

            digits.push(chars.iter().cloned().collect::<String>().parse()?);
        }

        Ok(digits.iter().sum())
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
