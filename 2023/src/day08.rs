mod util;

use std::{collections::HashMap, error::Error, io::BufRead};
use util::{aoc::AoCDay, input::get_reader};

const ID: &str = "day08";
type Input = (Vec<Instruction>, HashMap<[char; 3], ([char; 3], [char; 3])>);
type Output = usize;

#[derive(Debug)]
enum Instruction {
    Left,
    Right,
}

struct Day {}

impl AoCDay<Input, Output> for Day {
    fn parse_input(&self, id: &str) -> Result<Input, Box<dyn Error>> {
        let mut lines = get_reader(id)?.lines();

        let instructions = lines
            .next()
            .ok_or("No first line")??
            .chars()
            .map(|c| match c {
                'L' => Ok(Instruction::Left),
                'R' => Ok(Instruction::Right),
                _ => Err("Invalid instruction".into()),
            })
            .collect::<Result<Vec<_>, Box<dyn Error>>>()?;

        // NFK = (LMH, RSS)
        let map = lines
            .skip(1)
            .map(|line| {
                let line = line?;
                let chars = line.chars().collect::<Vec<_>>();

                #[allow(clippy::get_first)]
                let a1 = chars.get(0).ok_or("No a1 char")?;
                let a2 = chars.get(1).ok_or("No a2 char")?;
                let a3 = chars.get(2).ok_or("No a3 char")?;

                let b1 = chars.get(7).ok_or("No b1 char")?;
                let b2 = chars.get(8).ok_or("No b2 char")?;
                let b3 = chars.get(9).ok_or("No b3 char")?;

                let c1 = chars.get(12).ok_or("No b1 char")?;
                let c2 = chars.get(13).ok_or("No b2 char")?;
                let c3 = chars.get(14).ok_or("No b3 char")?;

                Ok(([*a1, *a2, *a3], ([*b1, *b2, *b3], [*c1, *c2, *c3])))
            })
            .collect::<Result<HashMap<_, _>, Box<dyn Error>>>()?;

        Ok((instructions, map))
    }

    fn part1(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        let instructions = &input.0;
        let map = &input.1;

        let mut position = ['A', 'A', 'A'];
        let mut i = 0;

        let mut steps = 0;

        loop {
            let (left, right) = map.get(&position).ok_or("No map entry")?;

            position = match instructions.get(i).ok_or("No instruction")? {
                Instruction::Left => *left,
                Instruction::Right => *right,
            };

            steps += 1;
            if position == ['Z', 'Z', 'Z'] {
                break;
            }

            i += 1;
            if i >= instructions.len() {
                i = 0;
            }
        }

        Ok(steps)
    }

    fn part2(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        let instructions = &input.0;
        let map = &input.1;

        let mut positions = map
            .keys()
            .filter(|c| c[2] == 'A')
            .cloned()
            .collect::<Vec<_>>();

        let steps = positions
            .iter_mut()
            .map(|position| {
                let mut steps = 0;
                let mut i = 0;

                loop {
                    let (left, right) = map.get(position).ok_or("No map entry")?;

                    *position = match instructions.get(i).ok_or("No instruction")? {
                        Instruction::Left => *left,
                        Instruction::Right => *right,
                    };

                    steps += 1;
                    if position[2] == 'Z' {
                        break;
                    }

                    i += 1;
                    if i >= instructions.len() {
                        i = 0;
                    }
                }

                Ok(steps)
            })
            .collect::<Result<Vec<_>, Box<dyn Error>>>()?;

        Ok(steps.iter().fold(1, |a, b| lcm(a, *b)))
    }
}

fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
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
            6
        );
    }

    #[test]
    fn test_solve_part2() {
        let day = Day {};
        assert_eq!(
            day.parse_and_solve_part2(format!("{}_test2", ID).as_str())
                .unwrap(),
            6
        );
    }
}
