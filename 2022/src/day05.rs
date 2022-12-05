mod util;

use std::{error::Error, io::BufRead};
use util::{aoc::AoCDay, input::get_reader};

const ID: &str = "day05";

type Stacks = [Vec<u8>; 9];
type Moves = Vec<(usize, usize, usize)>;

type Input = (Stacks, Moves);
type Output = String;

fn top_to_string(stacks: &Stacks) -> String {
    let mut result = String::new();
    for stack in stacks.iter() {
        if let Some(stack_element) = &stack.last() {
            result.push(**stack_element as char);
        }
    }
    result
}

struct Day {}

impl AoCDay<Input, Output> for Day {
    fn parse_input(&self, id: &str) -> Result<Input, Box<dyn Error>> {
        let reader = get_reader(id)?;

        let mut moves = Moves::new();
        let mut stacks: Stacks = Default::default();
        let mut stacks_done = false;

        for line in reader.lines() {
            let line = line?;
            if line.is_empty() {
                stacks_done = true;
                continue;
            }
            if !stacks_done {
                let bytes = line.bytes().collect::<Vec<_>>();
                for (i, stack) in stacks.iter_mut().enumerate() {
                    let index = i * 4 + 1;
                    if let Some(stack_element) = bytes.get(index) {
                        if stack_element.is_ascii_alphabetic() {
                            stack.push(*stack_element);
                        } else {
                            continue;
                        }
                    } else {
                        break;
                    }
                }
            } else {
                let elements = line.split_whitespace().collect::<Vec<_>>();
                moves.push((
                    // move 27 from 2 to 1
                    elements
                        .get(1)
                        .ok_or("invalid move: no count found")?
                        .parse()?,
                    (elements
                        .get(3)
                        .ok_or("invalid move: no from found")?
                        .as_bytes()
                        .first()
                        .ok_or("invalid move: empty from")?
                        - b'1') as usize,
                    (elements
                        .get(5)
                        .ok_or("invalid move: no to found")?
                        .as_bytes()
                        .first()
                        .ok_or("invalid move: empty to")?
                        - b'1') as usize,
                ));
            }
        }

        for stack in stacks.iter_mut() {
            stack.reverse();
        }

        Ok((stacks, moves))
    }

    fn part1(&self, (stacks, moves): &Input) -> Output {
        let mut stacks = stacks.clone();
        for (count, from, to) in moves {
            for _ in 0..*count {
                let element = stacks[*from]
                    .pop()
                    .expect("Bad input! TODO better error handling");
                stacks[*to].push(element);
            }
        }

        top_to_string(&stacks)
    }

    fn part2(&self, (stacks, moves): &Input) -> Output {
        let mut stacks = stacks.clone();
        for (count, from, to) in moves {
            let count = *count;
            let mut picked_crates = Vec::with_capacity(count);
            for _ in 0..count {
                let element = stacks[*from]
                    .pop()
                    .expect("Bad input! TODO better error handling");
                picked_crates.push(element);
            }
            for picked_crate in picked_crates.iter().rev() {
                stacks[*to].push(*picked_crate);
            }
        }

        top_to_string(&stacks)
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
            "CMZ"
        );
    }

    #[test]
    fn test_solve_part2() {
        let day = Day {};
        assert_eq!(
            day.parse_and_solve_part2(format!("{}_test1", ID).as_str())
                .unwrap(),
            "MCD"
        );
    }
}

fn main() {
    Day {}.run(ID);
}
