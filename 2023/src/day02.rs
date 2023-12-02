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
                        _ => panic!("Unknown color {}, aborting", color_split[1]),
                    }
                }

                game.rounds.push(sampling);
            }

            games.push(game);
        }

        Ok(games)
    }

    fn part1(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        let mut possible_games_sum = 0;

        'game_loop: for game in input {
            for sampling in &game.rounds {
                if sampling.red > 12 || sampling.green > 13 || sampling.blue > 14 {
                    continue 'game_loop;
                }
            }
            possible_games_sum += game.id;
        }

        Ok(possible_games_sum)
    }

    fn part2(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        let mut power_sum = 0;

        for game in input {
            let mut min_red = 0;
            let mut min_green = 0;
            let mut min_blue = 0;

            for sampling in &game.rounds {
                if sampling.red > min_red {
                    min_red = sampling.red;
                }
                if sampling.blue > min_blue {
                    min_blue = sampling.blue;
                }
                if sampling.green > min_green {
                    min_green = sampling.green;
                }
            }

            power_sum += min_red * min_green * min_blue;
        }

        Ok(power_sum)
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
