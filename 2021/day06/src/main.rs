use std::fs::File;
use std::io::{self, BufRead};
use std::time::Instant;
use thiserror::Error;

type Fishes = (
    usize,
    usize,
    usize,
    usize,
    usize,
    usize,
    usize,
    usize,
    usize,
);

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

fn parse(file: File) -> Result<Fishes, AoCError> {
    let line = io::BufReader::new(file)
        .lines()
        .next()
        .ok_or(AoCError::NoNumbersErr)??;

    line.split(',').map(|s| s.parse::<usize>()).fold(
        Ok((0, 0, 0, 0, 0, 0, 0, 0, 0)),
        |fishes, fertility| {
            let f = fishes?;
            match fertility? {
                0 => Ok((f.0 + 1, f.1, f.2, f.3, f.4, f.5, f.6, f.7, f.8)),
                1 => Ok((f.0, f.1 + 1, f.2, f.3, f.4, f.5, f.6, f.7, f.8)),
                2 => Ok((f.0, f.1, f.2 + 1, f.3, f.4, f.5, f.6, f.7, f.8)),
                3 => Ok((f.0, f.1, f.2, f.3 + 1, f.4, f.5, f.6, f.7, f.8)),
                4 => Ok((f.0, f.1, f.2, f.3, f.4 + 1, f.5, f.6, f.7, f.8)),
                5 => Ok((f.0, f.1, f.2, f.3, f.4, f.5 + 1, f.6, f.7, f.8)),
                6 => Ok((f.0, f.1, f.2, f.3, f.4, f.5, f.6 + 1, f.7, f.8)),
                7 => Ok((f.0, f.1, f.2, f.3, f.4, f.5, f.6, f.7 + 1, f.8)),
                8 => Ok((f.0, f.1, f.2, f.3, f.4, f.5, f.6, f.7, f.8 + 1)),
                _ => Err(AoCError::TooLargeNumberErr),
            }
        },
    )
}

fn parts(f: Fishes, days: usize) -> usize {
    let f = (0..=days).fold(f, |f, _| {
        (f.1, f.2, f.3, f.4, f.5, f.6, f.7 + f.0, f.8, f.0)
    });

    f.0 + f.1 + f.2 + f.3 + f.4 + f.5 + f.6 + f.7
}

fn main() -> Result<(), AoCError> {
    let start = Instant::now();

    let fishes = parse(File::open("input")?)?;

    println!("parsing: {}µs", start.elapsed().as_micros());

    let s_part1 = Instant::now();
    println!(
        "part1: {} ({}ns)",
        parts(fishes, 80),
        s_part1.elapsed().as_nanos()
    );

    let s_part2 = Instant::now();
    println!(
        "part2: {} ({}ns)",
        parts(fishes, 256),
        s_part2.elapsed().as_nanos()
    );

    println!("Time: {}µs", start.elapsed().as_micros());

    Ok(())
}

const _EXAMPLE: Fishes = (0, 1, 1, 2, 1, 0, 0, 0, 0);

#[test]
fn test_parse_example() {
    let fishes = parse(File::open("example").unwrap()).unwrap();
    assert_eq!(fishes, _EXAMPLE);
}

#[test]
fn test_part1_small() {
    assert_eq!(parts(_EXAMPLE, 18), 26);
}

#[test]
fn test_part1_big() {
    assert_eq!(parts(_EXAMPLE, 80), 5934);
}
