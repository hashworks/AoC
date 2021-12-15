use std::fmt::Debug;
use std::fs::File;
use std::io::{self, BufRead};
use std::time::Instant;
use thiserror::Error;
extern crate pathfinding;

use pathfinding::prelude::dijkstra;

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

fn parse(file: File) -> Result<Vec<Vec<u32>>, AoCError> {
    io::BufReader::new(file)
        .lines()
        .map(|l| {
            let l = l?;
            l.chars()
                .map(|c| c.to_digit(10).ok_or(AoCError::ParseBadInputErr))
                .collect()
        })
        .collect()
}

fn inc_risk_wrapped(risk: &u32) -> u32 {
    match risk {
        9 => 1,
        _ => risk + 1,
    }
}

fn quintuple_map(map: &mut Vec<Vec<u32>>) {
    let x_len = map.len();
    for x in 0..x_len * 5 {
        if x < x_len {
            let y_len = map[x].len();
            for y in y_len..y_len * 5 {
                let risk = map[x][y - y_len];
                map[x].push(inc_risk_wrapped(&risk));
            }
        } else {
            map.push(map[x - x_len].iter().map(inc_risk_wrapped).collect());
        }
    }
}

fn part1(map: &Vec<Vec<u32>>) -> Result<u32, AoCError> {
    let max_x = map.len() - 1;
    let max_y = map.get(0).ok_or(AoCError::ParseBadInputErr)?.len() - 1;

    let (_, risk) = dijkstra(
        &(0, 0),
        |&(x, y)| {
            let mut successors = Vec::with_capacity(4);
            if x > 0 {
                successors.push(((x - 1, y), map[x - 1][y]));
            }
            if x < max_x {
                successors.push(((x + 1, y), map[x + 1][y]));
            }
            if y > 0 {
                successors.push(((x, y - 1), map[x][y - 1]));
            }
            if y < max_y {
                successors.push(((x, y + 1), map[x][y + 1]));
            }
            successors
        },
        |p| *p == (max_x, max_y),
    )
    .ok_or(AoCError::SolveBadInputErr)?;

    Ok(risk)
}

fn main() -> Result<(), AoCError> {
    let start = Instant::now();

    let mut map = parse(File::open("input")?)?;

    println!("parsing: {}µs", start.elapsed().as_micros());

    let s_part1 = Instant::now();
    println!(
        "part1: {} ({}µs)",
        part1(&map)?,
        s_part1.elapsed().as_micros()
    );

    let s_part2 = Instant::now();
    quintuple_map(&mut map);
    println!(
        "part2: {} ({}µs)",
        part1(&map)?,
        s_part2.elapsed().as_micros()
    );

    println!("Time: {}µs", start.elapsed().as_micros());

    Ok(())
}

#[test]
fn test_part1_example() {
    let map = parse(File::open("example").unwrap()).unwrap();
    assert_eq!(part1(&map).unwrap(), 40);
}

#[test]
fn test_part2_example() {
    let mut map = parse(File::open("example").unwrap()).unwrap();
    let map_quintuple = parse(File::open("example_quintuple").unwrap()).unwrap();
    quintuple_map(&mut map);
    assert_eq!(map, map_quintuple);
    assert_eq!(part1(&map).unwrap(), 315);
}
