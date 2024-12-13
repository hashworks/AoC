mod util;

use std::{error::Error, io::BufRead};
use util::{aoc::AoCDay, input::get_reader};

const ID: &str = "day13";
type Input = Vec<(isize, isize, isize, isize, isize, isize)>;
type Output = isize;

struct Day {}

impl AoCDay<Input, Output> for Day {
    fn parse_input(&self, id: &str) -> Result<Input, Box<dyn Error>> {
        Ok(get_reader(id)?
            .lines()
            .filter(|line| line.as_ref().is_ok_and(|line| !line.is_empty()))
            .map(|line| {
                let line = line.unwrap();
                let (l, r) = line.split_once(',').ok_or("Invalid input")?;

                let ln = if let Some((_, l)) = l.split_once('+') {
                    l.parse::<isize>().map_err(|e| e.to_string())
                } else if let Some((_, l)) = l.split_once('=') {
                    l.parse::<isize>().map_err(|e| e.to_string())
                } else {
                    Err("Invalid input".to_string())
                }?;
                let rn = if let Some((_, r)) = r.split_once('+') {
                    r.parse::<isize>().map_err(|e| e.to_string())
                } else if let Some((_, r)) = r.split_once('=') {
                    r.parse::<isize>().map_err(|e| e.to_string())
                } else {
                    Err("Invalid input".to_string())
                }?;

                Ok(vec![ln, rn])
            })
            .flat_map(|result: Result<_, Box<dyn Error>>| match result {
                Ok(vec) => vec.into_iter().map(Ok).collect(),
                Err(er) => vec![Err(er)],
            })
            .collect::<Result<Vec<_>, _>>()?
            .chunks(6)
            .map(|ns| (ns[0], ns[1], ns[2], ns[3], ns[4], ns[5]))
            .collect())
    }

    fn part1(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        Ok(input
            .iter()
            .filter_map(solve_linear_equation)
            .filter(|(a, b)| a <= &100 && b <= &100 && a >= &0 && b >= &0)
            .map(|(a, b)| a * 3 + b)
            .sum())
    }

    fn part2(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        Ok(input
            .iter()
            .map(|(x1, y1, x2, y2, x, y)| {
                (*x1, *y1, *x2, *y2, 10000000000000 + x, 10000000000000 + y)
            })
            .filter_map(|c| solve_linear_equation(&c))
            .filter(|(a, b)| a >= &0 && b >= &0)
            .map(|(a, b)| a * 3 + b)
            .sum())
    }
}

/*
  a * x1 + b * x2 = x
  a * y1 + b * y2 = y

  https://en.wikipedia.org/wiki/Cramer%27s_rule
*/
fn solve_linear_equation(
    (x1, y1, x2, y2, x, y): &(isize, isize, isize, isize, isize, isize),
) -> Option<(isize, isize)> {
    let a = (y2 * x - x2 * y) / (x1 * y2 - y1 * x2);
    let b = (-y1 * x + x1 * y) / (x1 * y2 - y1 * x2);

    // Since we work with integers, we need to check if the solution is correct
    if a * x1 + b * x2 == *x {
        Some((a, b))
    } else {
        None
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
            480
        );
    }

    #[test]
    fn test_solve_part2() {
        let day = Day {};
        assert_eq!(
            day.parse_and_solve_part2(format!("{}_test1", ID).as_str())
                .unwrap(),
            42
        );
    }
}
