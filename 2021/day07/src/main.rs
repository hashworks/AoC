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
    #[error("no numbers found")]
    NoNumbersErr,
}

fn parse(file: File) -> Result<Vec<i32>, AoCError> {
    let line = io::BufReader::new(file)
        .lines()
        .next()
        .ok_or(AoCError::NoNumbersErr)??;

    line.split(',').map(|s| Ok(s.parse::<i32>()?)).collect()
}

fn part1(numbers: &mut Vec<i32>) -> i32 {
    let mut numbers = numbers.to_vec();
    numbers.sort();

    let median = (numbers[numbers.len() / 2] + numbers[(numbers.len() - 1) / 2]) / 2;

    numbers.iter().fold(0, |acc, n| acc + (n - median).abs())
}

fn part2(numbers: &Vec<i32>) -> Option<i32> {
    let average = numbers.iter().sum::<i32>() / numbers.len() as i32;

    let window = [average - 2, average - 1, average, average + 1, average + 2];

    let fuel_options = numbers
        .iter()
        .map(|n| {
            window
                .iter()
                .map(|window_average| {
                    let x = (n - window_average).abs();
                    x * (1 + x) / 2
                })
                .collect::<Vec<i32>>()
        })
        .fold(vec![0, 0, 0, 0, 0], |mut acc, v| {
            for (i, v) in v.iter().enumerate() {
                acc[i] += v;
            }
            acc
        });

    match fuel_options.iter().min() {
        Some(min) => Some(*min),
        None => None,
    }
}

fn main() -> Result<(), AoCError> {
    let start = Instant::now();

    let mut numbers = parse(File::open("input")?)?;

    println!("parsing: {}µs", start.elapsed().as_micros());

    let s_part1 = Instant::now();
    println!(
        "part1: {} ({}µs)",
        part1(&mut numbers),
        s_part1.elapsed().as_micros()
    );

    let s_part2 = Instant::now();
    println!(
        "part2: {} ({}µs)",
        part2(&numbers).ok_or(AoCError::NoNumbersErr)?,
        s_part2.elapsed().as_micros()
    );

    println!("Time: {}µs", start.elapsed().as_micros());

    Ok(())
}

const _EXAMPLE: [i32; 10] = [16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

#[test]
fn test_parse_example() {
    let numbers = parse(File::open("example").unwrap()).unwrap();
    assert_eq!(numbers, _EXAMPLE);
}

#[test]
fn test_part1_small() {
    assert_eq!(part1(&mut _EXAMPLE.to_vec().clone()), 37);
}

#[test]
fn test_part2_small() {
    assert_eq!(part2(&mut _EXAMPLE.to_vec().clone()), Some(168));
}
