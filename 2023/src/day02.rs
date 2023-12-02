mod util;

use std::{error::Error, io::BufRead};
use util::{aoc::AoCDay, input::get_reader};

const ID: &str = "day02";
type Input = Vec<Game>;
type Output = usize;

#[derive(Debug)]
struct Game {
    id: usize,
    rounds: Vec<Sampling>,
}

#[derive(Debug)]
struct Sampling {
    red: usize,
    green: usize,
    blue: usize,
}

struct Day {}

impl AoCDay<Input, Output> for Day {
    fn parse_input(&self, id: &str) -> Result<Input, Box<dyn Error>> {
        let reader = get_reader(id)?;
        let mut games = Vec::new();

        for gameline in reader.lines() {
            let gameline = gameline?;
            let colon_split = gameline.split(": ").collect::<Vec<&str>>();
            let game_id = &colon_split[0][5..].parse::<usize>()?;

            let rounds = colon_split[1].split("; ").collect::<Vec<&str>>();

            let mut game = Game {
                id: *game_id,
                rounds: Vec::new(),
            };

            for sampling in rounds {
                let sampling_split = sampling.split(", ").collect::<Vec<&str>>();
                let mut sampling = Sampling {
                    red: 0,
                    green: 0,
                    blue: 0,
                };
                for color in sampling_split {
                    let color_split = color.split(' ').collect::<Vec<&str>>();
                    match color_split[1] {
                        "red" => sampling.red = color_split[0].parse::<usize>()?,
                        "green" => sampling.green = color_split[0].parse::<usize>()?,
                        "blue" => sampling.blue = color_split[0].parse::<usize>()?,
                        _ => Err("Invalid color")?,
                    }
                }
                game.rounds.push(sampling);
            }

            games.push(game);
        }

        Ok(games)
    }

    fn part1(&self, games: &Input) -> Result<Output, Box<dyn Error>> {
        Ok(games
            .iter()
            .filter(|game| {
                game.rounds.iter().all(|sampling| {
                    sampling.red <= 12 && sampling.green <= 13 && sampling.blue <= 14
                })
            })
            .map(|game| game.id)
            .sum())
    }

    fn part2(&self, games: &Input) -> Result<Output, Box<dyn Error>> {
        Ok(games
            .iter()
            .map(|game| {
                // Note: It isn't specified what the power of a set is if a color isn't present in a sampling. I'm assuming it's 0.
                let mut max_red = 0;
                let mut max_green = 0;
                let mut max_blue = 0;

                game.rounds.iter().for_each(|sampling| {
                    max_red = sampling.red.max(max_red);
                    max_blue = sampling.blue.max(max_blue);
                    max_green = sampling.green.max(max_green);
                });

                max_red * max_green * max_blue
            })
            .sum())
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
            8
        );
    }

    #[test]
    fn test_solve_part2() {
        let day = Day {};
        assert_eq!(
            day.parse_and_solve_part2(format!("{}_test1", ID).as_str())
                .unwrap(),
            2286
        );
    }
}
