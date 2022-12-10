mod util;

use std::{error::Error, io::BufRead};
use util::{aoc::AoCDay, input::get_reader};

const ID: &str = "day10";
type Input = Vec<Option<i16>>;
type Output = i16;

struct Day {}

impl AoCDay<Input, Output> for Day {
    fn parse_input(&self, id: &str) -> Result<Input, Box<dyn Error>> {
        get_reader(id)?
            .lines()
            .map(|line| {
                let line = line?;
                if &line[..4] == "addx" {
                    Ok(Some(line[5..].parse::<i16>()?))
                } else {
                    Ok(None)
                }
            })
            .collect()
    }

    fn part1(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        let mut cycle = 0;
        let mut x = 1;
        let mut signal_strength = 0;

        for op in input {
            cycle += 1;
            set_signal_strength(&mut signal_strength, cycle, x);
            if let Some(addx) = op {
                cycle += 1;
                set_signal_strength(&mut signal_strength, cycle, x);
                x += addx;
            }
        }

        Ok(signal_strength)
    }

    fn part2(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        let mut cycle = 0;
        let mut x = 1;

        println!();

        for op in input {
            cycle += 1;
            draw_pixel(cycle, x);
            if let Some(addx) = op {
                cycle += 1;
                draw_pixel(cycle, x);
                x += addx;
            }
        }

        println!();

        Ok(42)
    }
}

fn draw_pixel(cycle: i16, x: i16) {
    let horizontal_position = (cycle - 1) % 40;
    if x + 1 >= horizontal_position && x - 1 <= horizontal_position {
        print!("\u{2588}");
    } else {
        print!(" ");
    }
    if cycle % 40 == 0 {
        println!();
    }
}

fn set_signal_strength(signal_strength: &mut i16, cycle: i16, x: i16) {
    if (cycle - 20) % 40 == 0 {
        *signal_strength += cycle * x
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
            13140
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

fn main() {
    Day {}.run(ID);
}
