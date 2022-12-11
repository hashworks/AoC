mod util;

use std::{error::Error, io::BufRead};
use util::{aoc::AoCDay, input::get_reader};

const ID: &str = "day11";
type Input = Vec<Monkey>;
type Output = usize;

struct Monkey {
    initial_items: Vec<usize>,
    div: usize,
    op: Box<dyn Fn(usize) -> usize>,
    test: Box<dyn Fn(usize) -> usize>,
}

struct Day {}

impl AoCDay<Input, Output> for Day {
    fn parse_input(&self, id: &str) -> Result<Input, Box<dyn Error>> {
        get_reader(id)?
            .lines()
            .collect::<Result<Vec<_>, _>>()?
            .chunks(7)
            .map(|monkey_lines| {
                let initial_items = monkey_lines[1][18..]
                    .split(", ")
                    .map(|x| x.parse())
                    .collect::<Result<_, _>>()?;

                let op_line = &monkey_lines[2];
                let op: Result<Box<dyn Fn(_) -> _>, Box<dyn Error>> =
                    match (&op_line[23..24], &op_line[25..]) {
                        ("*", "old") => Ok(Box::new(|x| x * x)),
                        ("*", int) => {
                            let int = int.parse::<usize>()?;
                            Ok(Box::new(move |x| x * int))
                        }
                        ("+", "old") => Ok(Box::new(|x| x + x)),
                        ("+", int) => {
                            let int = int.parse::<usize>()?;
                            Ok(Box::new(move |x| x + int))
                        }
                        _ => Err("Invalid op".into()),
                    };

                let test_divisible_by = monkey_lines[3][21..].parse()?;
                let test_target_true = monkey_lines[4][29..].parse()?;
                let test_target_false = monkey_lines[5][30..].parse()?;
                let test = Box::new(move |x| {
                    if x % test_divisible_by == 0 {
                        test_target_true
                    } else {
                        test_target_false
                    }
                });

                Ok(Monkey {
                    initial_items,
                    div: test_divisible_by,
                    op: op?,
                    test,
                })
            })
            .collect()
    }

    fn part1(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        let mut monkeys_items = input
            .iter()
            .map(|m| m.initial_items.clone())
            .collect::<Vec<_>>();
        let mut monkey_inspections = vec![0; input.len()];
        for _ in 0..20 {
            for (monkey_index, monkey) in input.iter().enumerate() {
                while let Some(item_worry_level) = monkeys_items[monkey_index].pop() {
                    let item_worry_level = (monkey.op)(item_worry_level) / 3;
                    let target_monkey_index = (monkey.test)(item_worry_level);
                    monkeys_items[target_monkey_index].push(item_worry_level);
                    monkey_inspections[monkey_index] += 1;
                }
            }
        }

        monkey_inspections.sort();

        Ok(monkey_inspections.iter().rev().take(2).product())
    }

    fn part2(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        let mut monkeys_items = input
            .iter()
            .map(|m| m.initial_items.clone())
            .collect::<Vec<_>>();

        // Basically: to keep it in a ring where it is still divisible
        // by all monkey-dividers, we need modulo it using their lcm
        let ring = input.iter().map(|m| m.div).product::<usize>();

        let mut monkey_inspections = vec![0; input.len()];
        for _ in 0..10000 {
            for (monkey_index, monkey) in input.iter().enumerate() {
                while let Some(item_worry_level) = monkeys_items[monkey_index].pop() {
                    let item_worry_level = (monkey.op)(item_worry_level) % ring;
                    let target_monkey_index = (monkey.test)(item_worry_level);

                    monkeys_items[target_monkey_index].push(item_worry_level);
                    monkey_inspections[monkey_index] += 1;
                }
            }
        }

        monkey_inspections.sort();

        Ok(monkey_inspections.iter().rev().take(2).product())
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
            10605
        );
    }

    #[test]
    fn test_solve_part2() {
        let day = Day {};
        assert_eq!(
            day.parse_and_solve_part2(format!("{}_test1", ID).as_str())
                .unwrap(),
            2713310158
        );
    }
}

fn main() {
    Day {}.run(ID);
}
