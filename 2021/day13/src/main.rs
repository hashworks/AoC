use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::time::Instant;
use thiserror::Error;

use crate::Fold::*;

#[derive(Error, Debug)]
enum AoCError {
    #[error("io error")]
    IoErr(#[from] io::Error),
    #[error("parse int error")]
    ParseIntErr(#[from] std::num::ParseIntError),
    #[error("failed to parse bad input")]
    ParseBadInputErr,
    #[error("failed to solve bad input")]
    SolveBadInputErr,
}

enum Fold {
    X(usize),
    Y(usize),
}

fn parse(file: File) -> Result<(HashSet<(usize, usize)>, Vec<Fold>), AoCError> {
    let mut lines = io::BufReader::new(file).lines();

    let mut sheet = HashSet::new();

    while let Some(Ok(line)) = lines.next() {
        if line.is_empty() {
            break;
        }
        let (x, y) = line.split_once(',').ok_or(AoCError::ParseBadInputErr)?;
        let x = x.parse::<usize>()?;
        let y = y.parse::<usize>()?;
        sheet.insert((x, y));
    }

    let mut folds = Vec::new();

    while let Some(Ok(line)) = lines.next() {
        let (f, i) = line.split_once('=').ok_or(AoCError::ParseBadInputErr)?;
        let f = f
            .chars()
            .skip(11) // fold along[ ]
            .take(1)
            .next()
            .ok_or(AoCError::ParseBadInputErr)?;
        let i = i.parse::<usize>()?;
        match f {
            'x' => folds.push(X(i)),
            'y' => folds.push(Y(i)),
            _ => return Err(AoCError::ParseBadInputErr),
        }
    }

    Ok((sheet, folds))
}

fn part1(sheet: HashSet<(usize, usize)>, fold: &Fold) -> HashSet<(usize, usize)> {
    sheet
        .iter()
        .map(|(p_x, p_y)| match fold {
            X(x) if p_x > &x => (2 * x - p_x, *p_y),
            Y(y) if p_y > &y => (*p_x, 2 * y - p_y),
            _ => (*p_x, *p_y),
        })
        .collect()
}

fn print_sheet(sheet: HashSet<(usize, usize)>) {
    let (max_x, max_y) = sheet.iter().fold((0, 0), |(max_x, max_y), (x, y)| {
        (max_x.max(*x), max_y.max(*y))
    });

    let mut vec = vec![vec![' '; max_x + 1]; max_y + 1];

    for (x, y) in sheet {
        vec[y][x] = '█';
    }

    for row in vec {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}

fn main() -> Result<(), AoCError> {
    let start = Instant::now();

    let (mut sheet, folds) = parse(File::open("input")?)?;

    println!("parsing: {}µs", start.elapsed().as_micros());

    let s_part1 = Instant::now();
    println!(
        "part1: {} ({}µs)",
        part1(
            sheet.clone(),
            folds.first().ok_or(AoCError::SolveBadInputErr)?
        )
        .len(),
        s_part1.elapsed().as_micros()
    );

    let s_part2 = Instant::now();
    for fold in folds {
        sheet = part1(sheet, &fold);
    }
    print_sheet(sheet);
    println!("part2: {}µs", s_part2.elapsed().as_micros());

    println!("Time: {}µs", start.elapsed().as_micros());

    Ok(())
}

const _EXAMPLE: [(usize, usize); 18] = [
    (6, 10),
    (0, 14),
    (9, 10),
    (0, 3),
    (10, 4),
    (4, 11),
    (6, 0),
    (6, 12),
    (4, 1),
    (0, 13),
    (10, 12),
    (3, 4),
    (3, 0),
    (8, 4),
    (1, 10),
    (2, 14),
    (8, 10),
    (9, 0),
];

#[test]
fn test_part1_example() {
    let first_fold = part1(HashSet::from(_EXAMPLE), &Fold::Y(7));
    assert_eq!(first_fold.len(), 17);
    let second_fold = part1(first_fold, &Fold::X(5));
    assert_eq!(second_fold.len(), 16);
}
