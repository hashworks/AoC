use std::fs::File;
use std::io::{self, BufRead};
use std::time::Instant;
use thiserror::Error;

#[derive(Error, Debug)]
enum AoCError {
    #[error("io error")]
    IoErr(#[from] io::Error),
    #[error("failed to parse Int")]
    ParseIntErr(#[from] std::num::ParseIntError),
    #[error("bad input")]
    ParseBadInputErr,
}

#[derive(Debug, PartialEq)]
enum Bracket {
    RoundOpen,
    RoundClose,
    SquareOpen,
    SquareClose,
    CurlyOpen,
    CurlyClose,
    AngleOpen,
    AngleClose,
}

fn parse(file: File) -> Result<Vec<Vec<Bracket>>, AoCError> {
    io::BufReader::new(file)
        .lines()
        .map(|l| {
            Ok(l?
                .chars()
                .map(|c| match c {
                    '(' => Ok(Bracket::RoundOpen),
                    ')' => Ok(Bracket::RoundClose),
                    '[' => Ok(Bracket::SquareOpen),
                    ']' => Ok(Bracket::SquareClose),
                    '{' => Ok(Bracket::CurlyOpen),
                    '}' => Ok(Bracket::CurlyClose),
                    '<' => Ok(Bracket::AngleOpen),
                    '>' => Ok(Bracket::AngleClose),
                    _ => return Err(AoCError::ParseBadInputErr),
                })
                .collect::<Result<Vec<Bracket>, AoCError>>()?)
        })
        .collect()
}

fn part1(input: &Vec<Vec<Bracket>>) -> usize {
    let mut score = 0;

    for line in input {
        let mut stack = Vec::with_capacity(line.len());
        for bracket in line {
            match bracket {
                Bracket::RoundOpen => {
                    stack.push(Bracket::RoundOpen);
                }
                Bracket::SquareOpen => {
                    stack.push(Bracket::SquareOpen);
                }
                Bracket::CurlyOpen => {
                    stack.push(Bracket::CurlyOpen);
                }
                Bracket::AngleOpen => {
                    stack.push(Bracket::AngleOpen);
                }
                Bracket::RoundClose => {
                    let last = stack.pop();
                    if last != Some(Bracket::RoundOpen) {
                        score += 3;
                        break;
                    }
                }
                Bracket::SquareClose => {
                    let last = stack.pop();
                    if last != Some(Bracket::SquareOpen) {
                        score += 57;
                        break;
                    }
                }
                Bracket::CurlyClose => {
                    let last = stack.pop();
                    if last != Some(Bracket::CurlyOpen) {
                        score += 1197;
                        break;
                    }
                }
                Bracket::AngleClose => {
                    let last = stack.pop();
                    if last != Some(Bracket::AngleOpen) {
                        score += 25137;
                        break;
                    }
                }
            }
        }
    }

    score
}

fn part2(input: &Vec<Vec<Bracket>>) -> usize {
    let mut scores = vec![];

    'lines: for line in input {
        let mut stack = Vec::with_capacity(line.len());
        for bracket in line {
            match bracket {
                Bracket::RoundOpen => {
                    stack.push(Bracket::RoundOpen);
                }
                Bracket::SquareOpen => {
                    stack.push(Bracket::SquareOpen);
                }
                Bracket::CurlyOpen => {
                    stack.push(Bracket::CurlyOpen);
                }
                Bracket::AngleOpen => {
                    stack.push(Bracket::AngleOpen);
                }
                Bracket::RoundClose => {
                    let last = stack.pop();
                    if last != Some(Bracket::RoundOpen) {
                        continue 'lines;
                    }
                }
                Bracket::SquareClose => {
                    let last = stack.pop();
                    if last != Some(Bracket::SquareOpen) {
                        continue 'lines;
                    }
                }
                Bracket::CurlyClose => {
                    let last = stack.pop();
                    if last != Some(Bracket::CurlyOpen) {
                        continue 'lines;
                    }
                }
                Bracket::AngleClose => {
                    let last = stack.pop();
                    if last != Some(Bracket::AngleOpen) {
                        continue 'lines;
                    }
                }
            }
        }

        scores.push(
            stack
                .iter()
                .rev()
                .map(|b| match b {
                    Bracket::RoundOpen => 1,
                    Bracket::SquareOpen => 2,
                    Bracket::CurlyOpen => 3,
                    Bracket::AngleOpen => 4,
                    _ => 0,
                })
                .fold(0, |acc, x| acc * 5 + x),
        );
    }

    scores.sort();

    if scores.len() == 0 {
        0
    } else {
        scores[(scores.len() - 1) / 2]
    }
}

fn main() -> Result<(), AoCError> {
    let start = Instant::now();

    let input = parse(File::open("input")?)?;

    println!("parsing: {}µs", start.elapsed().as_micros());

    let s_part1 = Instant::now();
    println!(
        "part1: {} ({}µs)",
        part1(&input),
        s_part1.elapsed().as_micros()
    );

    let s_part2 = Instant::now();
    println!(
        "part2: {} ({}µs)",
        part2(&input),
        s_part2.elapsed().as_micros()
    );

    println!("Time: {}µs", start.elapsed().as_micros());

    Ok(())
}

#[test]
fn test_part1_example() {
    let basin_map = parse(File::open("example").unwrap()).unwrap();
    assert_eq!(part1(&basin_map), 26397);
}

#[test]
fn test_part2_example() {
    let mut basin_map = parse(File::open("example").unwrap()).unwrap();
    assert_eq!(part2(&mut basin_map), 288957);
}
