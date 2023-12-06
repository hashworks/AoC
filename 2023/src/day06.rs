mod util;

use std::{error::Error, io::BufRead};
use util::{aoc::AoCDay, input::get_reader};

const ID: &str = "day06";
type Input = Vec<(usize, usize)>;
type Output = usize;

struct Day {}

impl AoCDay<Input, Output> for Day {
    fn parse_input(&self, id: &str) -> Result<Input, Box<dyn Error>> {
        let mut lines = get_reader(id)?.lines();

        let times_str = lines.next().ok_or("Parse Error")??;
        let times = &times_str[11..] // 'Time:      '
            .split_whitespace()
            .map(|t| t.parse::<usize>())
            .collect::<Result<Vec<_>, _>>()?;

        let distances_str = lines.next().ok_or("Parse Error")??;
        let distances = &distances_str[11..] // 'Distance:      '
            .split_whitespace()
            .map(|t| t.parse::<usize>())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(times
            .iter()
            .zip(distances.iter())
            .map(|(d, t)| (*d, *t))
            .collect::<Vec<_>>())
    }

    fn part1(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        Ok(input
            .iter()
            .map(|(time, distance_record)| {
                (1..*time)
                    .zip((1..*time).rev())
                    .filter(|(hold, travel)| hold * travel > *distance_record)
                    .count()
            })
            .product())
    }

    fn part2(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        let (one_time, one_distance_record) = input.iter().fold(
            ("".to_owned(), "".to_owned()),
            |(one_time, one_distance_record), (time, distance_record)| {
                (
                    (one_time.to_owned() + &time.to_string()),
                    (one_distance_record.to_owned() + &distance_record.to_string()),
                )
            },
        );
        let one_time = one_time.parse::<usize>()?;
        let one_distance_record = one_distance_record.parse::<usize>()?;

        Ok((1..one_time)
            .zip((1..one_time).rev())
            .filter(|(hold, travel)| hold * travel > one_distance_record)
            .count())
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
            288
        );
    }

    #[test]
    fn test_solve_part2() {
        let day = Day {};
        assert_eq!(
            day.parse_and_solve_part2(format!("{}_test1", ID).as_str())
                .unwrap(),
            71503
        );
    }
}
