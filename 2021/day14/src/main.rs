use core::hash::Hash;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::File;
use std::io::{self, BufRead};
use std::time::Instant;
use thiserror::Error;

#[derive(Error, Debug)]
enum AoCError {
    #[error("io error")]
    IoErr(#[from] io::Error),
    #[error("parse int error")]
    ParseIntErr(#[from] std::num::ParseIntError),
    #[error("failed to parse bad input")]
    ParseBadInputErr,
}

type Pairs<T> = HashMap<(T, T), usize>;
type Rules<T> = HashMap<(T, T), T>;

fn parse(file: File) -> Result<(Pairs<char>, Rules<char>), AoCError> {
    let mut lines = io::BufReader::new(file).lines();
    let input_line = lines.next().ok_or(AoCError::ParseBadInputErr)??;

    let mut pairs = Pairs::new();
    input_line
        .chars()
        .zip(input_line.chars().skip(1))
        .for_each(|(a, b)| *pairs.entry((a, b)).or_insert(0) += 1);

    let rules = lines
        .skip(1)
        .map(|l| {
            let l = l?;
            let mut chars = l.chars();
            let a = chars.next().ok_or(AoCError::ParseBadInputErr)?;
            let b = chars.next().ok_or(AoCError::ParseBadInputErr)?;
            let c = chars.skip(4).next().ok_or(AoCError::ParseBadInputErr)?;
            Ok(((a, b), c))
        })
        .collect::<Result<Rules<char>, AoCError>>()?;

    Ok((pairs, rules))
}

fn polymerize<T: Eq + Hash + Copy>(
    mut pairs: Pairs<T>,
    rules: &Rules<T>,
    steps: usize,
) -> Pairs<T> {
    for _ in 0..steps {
        let mut new_pairs = HashMap::new();
        for ((a, b), count) in pairs {
            if let Some(&new_char) = rules.get(&(a, b)) {
                *new_pairs.entry((a, new_char)).or_insert(0) += count;
                *new_pairs.entry((new_char, b)).or_insert(0) += count;
            }
        }
        pairs = new_pairs;
    }
    pairs
}

fn count_polymers<T: Eq + Hash + Copy>(pairs: Pairs<T>, rules: &Rules<T>, steps: usize) -> usize {
    let polymer_pairs = polymerize(pairs, rules, steps);
    let mut polymers = HashMap::new();
    for ((a, b), count) in polymer_pairs {
        *polymers.entry(a).or_insert(0) += count;
        *polymers.entry(b).or_insert(0) += count;
    }
    let mut min = std::usize::MAX;
    let mut max = 0;
    for (_, value) in polymers {
        if value < min {
            min = value;
        }
        if value > max {
            max = value;
        }
    }
    (max + (max % 2) - min + (min % 2)) / 2
}

fn main() -> Result<(), AoCError> {
    let start = Instant::now();

    let (pairs, rules) = parse(File::open("input")?)?;

    println!("parsing: {}µs", start.elapsed().as_micros());

    let s_part1 = Instant::now();
    println!(
        "part1: {} ({}µs)",
        count_polymers(pairs.clone(), &rules, 10),
        s_part1.elapsed().as_micros()
    );

    let s_part2 = Instant::now();
    println!(
        "part2: {} ({}µs)",
        count_polymers(pairs, &rules, 40),
        s_part2.elapsed().as_micros()
    );

    println!("Time: {}µs", start.elapsed().as_micros());

    Ok(())
}

#[test]
fn test_part1_example() {
    let (pairs, rules) = parse(File::open("example").unwrap()).unwrap();
    assert_eq!(count_polymers(pairs, &rules, 10), 1588);
}
