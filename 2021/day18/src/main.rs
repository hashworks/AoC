use std::fmt::Debug;
use std::fs::File;
use std::io::{self, BufRead};
use std::time::Instant;
use thiserror::Error;

use crate::snailfish::SFR;

pub mod snailfish;

#[derive(Error, Debug)]
enum AoCError {
    #[error("io error")]
    IoErr(#[from] io::Error),
    #[error("failed to parse int")]
    ParseIntErr(#[from] std::num::ParseIntError),
}

fn parse(file: File) -> Result<Vec<SFR>, AoCError> {
    io::BufReader::new(file)
        .lines()
        .map(|s| s?.parse())
        .collect()
}

fn part1(sfrs: Vec<SFR>) -> usize {
    let first = sfrs[0].clone();
    sfrs.iter()
        .skip(1)
        .fold(first, |a, sfr| a + sfr.clone())
        .magnitude()
}

fn part2(sfrs: Vec<SFR>) -> usize {
    (0..sfrs.len())
        .map(|i| {
            (0..sfrs.len())
                .filter(|&j| i != j)
                .map(|j| (sfrs[i].clone() + sfrs[j].clone()).magnitude())
                .max()
                .unwrap_or(0)
        })
        .max()
        .unwrap_or(0)
}

fn main() -> Result<(), AoCError> {
    let start = Instant::now();

    let sfrs = parse(File::open("input")?)?;

    println!("parsing: {}µs", start.elapsed().as_micros());

    let s_part1 = Instant::now();
    println!(
        "part1: {} ({}µs)",
        part1(sfrs.clone()),
        s_part1.elapsed().as_micros()
    );

    let s_part2 = Instant::now();
    println!(
        "part2: {} ({}µs)",
        part2(sfrs),
        s_part2.elapsed().as_micros()
    );

    println!("Time: {}µs", start.elapsed().as_micros());

    Ok(())
}

#[test]
fn test_part1() {
    assert_eq!(
        part1(vec![
            "[1,1]".parse().unwrap(),
            "[2,2]".parse().unwrap(),
            "[3,3]".parse().unwrap(),
            "[4,4]".parse().unwrap(),
            "[5,5]".parse().unwrap(),
            "[6,6]".parse().unwrap(),
        ]),
        1137
    );
}

#[test]
fn test_part2() {
    assert_eq!(
        part2(vec![
            "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]"
                .parse()
                .unwrap(),
            "[[[5,[2,8]],4],[5,[[9,9],0]]]".parse().unwrap(),
            "[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]".parse().unwrap(),
            "[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]".parse().unwrap(),
            "[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]".parse().unwrap(),
            "[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]".parse().unwrap(),
            "[[[[5,4],[7,7]],8],[[8,3],8]]".parse().unwrap(),
            "[[9,3],[[9,9],[6,[4,9]]]]".parse().unwrap(),
            "[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]".parse().unwrap(),
            "[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]".parse().unwrap(),
        ]),
        3993
    );
}
