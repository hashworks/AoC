use std::cmp::{max, min};
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
    #[error("bad input parse error")]
    BadInputParseErr,
}

fn parse(file: File) -> Result<(i32, i32, i32, i32), AoCError> {
    let line = io::BufReader::new(file).lines().next().unwrap()?;
    let (x_part, y_part) = line.split_once(", ").ok_or(AoCError::BadInputParseErr)?;
    let x_range_str = x_part.split_once('=').ok_or(AoCError::BadInputParseErr)?.1;
    let y_range_str = y_part.split_once('=').ok_or(AoCError::BadInputParseErr)?.1;
    let (x1, x2) = x_range_str
        .split_once("..")
        .ok_or(AoCError::BadInputParseErr)?;
    let (y1, y2) = y_range_str
        .split_once("..")
        .ok_or(AoCError::BadInputParseErr)?;

    let x1 = x1.parse()?;
    let x2 = x2.parse()?;
    let y1 = y1.parse()?;
    let y2 = y2.parse()?;

    let (x1, x2) = (min(x1, x2), max(x1, x2));
    let (y1, y2) = (min(y1, y2), max(y1, y2));

    Ok((x1, x2, y1, y2))
}

fn part1(min_y: i32) -> i32 {
    min_y * (min_y + 1) / 2
}

fn hits(dv: i32, dy: i32, x1: i32, x2: i32, y1: i32, y2: i32) -> bool {
    let mut dx = dv;
    let mut dy = dy;
    let mut x = 0;
    let mut y = 0;
    loop {
        x += dx;
        y += dy;

        if x > x2 || y < y1 || (dx == 0 && x < x1) {
            return false;
        }

        if x >= x1 && x <= x2 && y >= y1 && y <= y2 {
            return true;
        }

        dx = if dx > 0 {
            dx - 1
        } else if dx < 0 {
            dx + 1
        } else {
            0
        };
        dy -= 1;
    }
}

fn part2(x1: i32, x2: i32, y1: i32, y2: i32) -> usize {
    let mut hit_count = 0;
    for dx in 1..=x2 {
        for dy in y1..=y2 + x2 {
            if hits(dx, dy, x1, x2, y1, y2) {
                hit_count += 1;
            }
        }
    }
    hit_count
}

fn main() -> Result<(), AoCError> {
    let start = Instant::now();

    let (x1, x2, y1, y2) = parse(File::open("input")?)?;

    println!("parsing: {}µs", start.elapsed().as_micros());

    let s_part1 = Instant::now();
    println!("part1: {} ({}ns)", part1(y1), s_part1.elapsed().as_nanos());

    let s_part2 = Instant::now();
    println!(
        "part2: {} ({}µs)",
        part2(x1, x2, y1, y2),
        s_part2.elapsed().as_micros()
    );

    println!("Time: {}µs", start.elapsed().as_micros());

    Ok(())
}

#[test]
fn test_part1() {
    assert_eq!(part1(-10), 45);
}

#[test]
fn test_part2() {
    assert_eq!(part2(20, 30, -10, -5), 112);
}
