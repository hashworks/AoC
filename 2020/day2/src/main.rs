use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
extern crate nom;

use nom::{
    bytes::complete::tag,
    character::streaming::{anychar, digit1},
    IResult,
};

#[derive(Debug, PartialEq)]
pub struct Policy {
    pub min: usize,
    pub max: usize,
    pub char: char,
}

#[derive(Debug, PartialEq)]
pub struct Password {
    pub policy: Policy,
    pub password: String,
}

impl Password {
    fn is_valid_part1(&self) -> bool {
        let count = self
            .password
            .chars()
            .filter(|c| c == &self.policy.char)
            .count();
        count < self.policy.min || count > self.policy.max
    }

    fn is_valid_part2(&self) -> bool {
        self.password
            .chars()
            .enumerate()
            .filter(|(p, c)| {
                (*p == self.policy.min - 1 || *p == self.policy.max - 1) && *c == self.policy.char
            })
            .take(2)
            .count()
            == 1
    }
}

fn password(input_string: String) -> Password {
    let (password, (min, max, char)) = password_parser(input_string.as_str()).unwrap();

    Password {
        policy: Policy {
            min: min.parse::<usize>().unwrap(),
            max: max.parse::<usize>().unwrap(),
            char,
        },
        password: password.into(),
    }
}

// 1-3 a: a
// 18-20 h: hhrmbrhhhhlhhvhmhhhb
fn password_parser(input: &str) -> IResult<&str, (&str, &str, char)> {
    let (input, min) = digit1(input)?;
    let (input, _) = tag("-")(input)?;
    let (input, max) = digit1(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, char) = anychar(input)?;
    let (input, _) = tag(": ")(input)?;

    Ok((input, (min, max, char)))
}

fn count_filter(f: File, filter: &dyn Fn(&Password) -> bool) -> usize {
    BufReader::new(f)
        .lines()
        .map(|line| password(line.unwrap()))
        .filter(filter)
        .count()
}

fn main() {
    let s1 = Instant::now();

    println!(
        "part1: {} ({}µs)",
        count_filter(File::open("./input").unwrap(), &|p| !p.is_valid_part1()),
        s1.elapsed().as_micros()
    );

    let s2 = Instant::now();

    println!(
        "part2: {} ({}µs)",
        count_filter(File::open("./input").unwrap(), &|p| p.is_valid_part2()),
        s2.elapsed().as_micros()
    );

    println!("Time: {}µs", s1.elapsed().as_micros());
}
