mod util;

use std::{error::Error, io::BufRead};
use util::{aoc::AoCDay, input::get_reader};

const ID: &str = "day02";
type Input = Vec<Round>;
type Output = usize;

struct Round {
    opponent: Item,
    outcome: Outcome,
}

impl TryInto<Round> for String {
    type Error = Box<dyn Error>;

    fn try_into(self) -> Result<Round, Self::Error> {
        let mut chars = self.chars();
        let opponent = chars.next();
        chars.next();
        let outcome = chars.next();
        if opponent.is_none() || outcome.is_none() {
            return Err("Invalid input".into());
        }
        Ok(Round {
            opponent: opponent.unwrap().try_into()?,
            outcome: outcome.unwrap().try_into()?,
        })
    }
}

enum Item {
    Rock,
    Paper,
    Scissors,
}

impl TryInto<Item> for char {
    type Error = String;

    fn try_into(self) -> Result<Item, Self::Error> {
        match self {
            'A' => Ok(Item::Rock),
            'B' => Ok(Item::Paper),
            'C' => Ok(Item::Scissors),
            _ => Err(format!("Invalid item: {}", self).into()),
        }
    }
}

enum Outcome {
    Win,
    Loose,
    Draw,
}

impl TryInto<Outcome> for char {
    type Error = String;

    fn try_into(self) -> Result<Outcome, Self::Error> {
        match self {
            'X' => Ok(Outcome::Loose),
            'Y' => Ok(Outcome::Draw),
            'Z' => Ok(Outcome::Win),
            _ => Err(format!("Invalid outcome: {}", self).into()),
        }
    }
}

struct Day {}

impl AoCDay<Input, Output> for Day {
    fn parse_input(&self, id: &str) -> Result<Input, Box<dyn Error>> {
        get_reader(id)?
            .lines()
            .map(|l| l?.try_into())
            .into_iter()
            .collect::<Result<_, _>>()
    }

    fn part1(&self, input: &Input) -> Output {
        input
            .iter()
            .map(|r| {
                let player = match r.outcome {
                    Outcome::Loose => Item::Rock,
                    Outcome::Draw => Item::Paper,
                    Outcome::Win => Item::Scissors,
                };
                match (player, &r.opponent) {
                    (Item::Rock, Item::Scissors) => 1 + 6,
                    (Item::Paper, Item::Rock) => 2 + 6,
                    (Item::Scissors, Item::Paper) => 3 + 6,
                    (Item::Rock, Item::Rock) => 1 + 3,
                    (Item::Paper, Item::Paper) => 2 + 3,
                    (Item::Scissors, Item::Scissors) => 3 + 3,
                    (Item::Rock, Item::Paper) => 1 + 0,
                    (Item::Paper, Item::Scissors) => 2 + 0,
                    (Item::Scissors, Item::Rock) => 3 + 0,
                }
            })
            .sum()
    }

    fn part2(&self, input: &Input) -> Output {
        input
            .iter()
            .map(|r| match (&r.outcome, &r.opponent) {
                (Outcome::Win, Item::Scissors) => 1 + 6,
                (Outcome::Win, Item::Rock) => 2 + 6,
                (Outcome::Win, Item::Paper) => 3 + 6,
                (Outcome::Draw, Item::Rock) => 1 + 3,
                (Outcome::Draw, Item::Paper) => 2 + 3,
                (Outcome::Draw, Item::Scissors) => 3 + 3,
                (Outcome::Loose, Item::Paper) => 1 + 0,
                (Outcome::Loose, Item::Scissors) => 2 + 0,
                (Outcome::Loose, Item::Rock) => 3 + 0,
            })
            .sum()
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
            15
        );
    }

    #[test]
    fn test_solve_part2() {
        let day = Day {};
        assert_eq!(
            day.parse_and_solve_part2(format!("{}_test1", ID).as_str())
                .unwrap(),
            12
        );
    }
}

fn main() {
    Day {}.run(ID);
}
