use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::time::Instant;
use thiserror::Error;

#[derive(Error, Debug)]
enum AoCError {
    #[error("io error")]
    IoErr(#[from] io::Error),
    #[error("failed to parse int")]
    ParseIntErr(#[from] std::num::ParseIntError),
    #[error("no numbers found")]
    NoNumbersErr,
    #[error("number too large")]
    TooLargeNumberErr,
}

fn parse(file: File) -> Result<Vec<(usize, usize, usize, usize)>, AoCError> {
    io::BufReader::new(file)
        .lines()
        .map(|l| {
            Ok({
                let line = l?;
                let mut split = line.split_ascii_whitespace();
                let first_pair = split
                    .next()
                    .ok_or(AoCError::NoNumbersErr)?
                    .split_once(',')
                    .ok_or(AoCError::NoNumbersErr)?;
                split.next();
                let second_pair = split
                    .next()
                    .ok_or(AoCError::NoNumbersErr)?
                    .split_once(',')
                    .ok_or(AoCError::NoNumbersErr)?;
                (
                    first_pair.0.parse::<usize>()?,
                    first_pair.1.parse::<usize>()?,
                    second_pair.0.parse::<usize>()?,
                    second_pair.1.parse::<usize>()?,
                )
            })
        })
        .collect()
}

const MAX_XY: usize = 1000;

fn parts(lines: &Vec<(usize, usize, usize, usize)>, diagonal: bool) -> Result<usize, AoCError> {
    let mut overlaps = HashSet::<(usize, usize)>::new();
    let mut diagram = [[false; MAX_XY]; MAX_XY];

    for (x1, y1, x2, y2) in lines
        .iter()
        .filter(|(x1, y1, x2, y2)| diagonal || x1 == x2 || y1 == y2)
    {
        if x1 >= &MAX_XY || y1 >= &MAX_XY || x2 >= &MAX_XY || y2 >= &MAX_XY {
            return Err(AoCError::TooLargeNumberErr);
        }

        let (x1, y1, x2, y2) = (*x1 as i64, *y1 as i64, *x2 as i64, *y2 as i64);

        let x_step = (x2 - x1).signum();
        let y_step = (y2 - y1).signum();
        let (mut x, mut y) = (x1, y1);

        while (x, y) != (x2 + x_step, y2 + y_step) {
            let x_u = x as usize;
            let y_u = y as usize;

            if diagram[x_u][y_u] {
                overlaps.insert((x_u, y_u));
            } else {
                diagram[x_u][y_u] = true;
            }

            x += x_step;
            y += y_step;
        }
    }

    Ok(overlaps.len())
}

fn main() -> Result<(), AoCError> {
    let start = Instant::now();

    let lines = parse(File::open("input")?)?;

    println!("parsing: {}µs", start.elapsed().as_micros());

    let s_part1 = Instant::now();
    println!(
        "part1: {} ({}µs)",
        parts(&lines, false)?,
        s_part1.elapsed().as_micros()
    );

    let s_part2 = Instant::now();
    println!(
        "part2: {} ({}µs)",
        parts(&lines, true)?,
        s_part2.elapsed().as_micros()
    );

    println!("Time: {}µs", start.elapsed().as_micros());

    Ok(())
}

const _EXAMPLE: [(usize, usize, usize, usize); 10] = [
    (0, 9, 5, 9),
    (8, 0, 0, 8),
    (9, 4, 3, 4),
    (2, 2, 2, 1),
    (7, 0, 7, 4),
    (6, 4, 2, 0),
    (0, 9, 2, 9),
    (3, 4, 1, 4),
    (0, 0, 8, 8),
    (5, 5, 8, 2),
];

#[test]
fn test_parse_example() {
    let lines = parse(File::open("example").unwrap()).unwrap();
    assert_eq!(_EXAMPLE.len(), lines.len());
    assert_eq!(_EXAMPLE[0], lines[0]);
    assert_eq!(_EXAMPLE[9], lines[9]);
}

#[test]
fn test_part1() {
    assert_eq!(5, parts(&_EXAMPLE.to_vec(), false).unwrap());
}

#[test]
fn test_part2() {
    assert_eq!(12, parts(&_EXAMPLE.to_vec(), true).unwrap());
}
