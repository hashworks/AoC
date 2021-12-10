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

fn parse(file: File) -> Result<Vec<Vec<u32>>, AoCError> {
    io::BufReader::new(file)
        .lines()
        .map(|l| {
            Ok(l?
                .chars()
                .map(|c| c.to_digit(10).ok_or(AoCError::ParseBadInputErr))
                .collect::<Result<Vec<u32>, AoCError>>()?)
        })
        .collect()
}

fn part1(basin_map: &Vec<Vec<u32>>) -> u32 {
    let mut lowest_points = 0;

    let max_x = basin_map.len() - 1;
    for x in 0..=max_x {
        let max_y = basin_map[x].len() - 1;
        for y in 0..=max_y {
            let p = basin_map[x][y];
            if (x == 0 || basin_map[x - 1][y] > p)
                && (y == 0 || basin_map[x][y - 1] > p)
                && (x == max_x || basin_map[x + 1][y] > p)
                && (y == max_y || basin_map[x][y + 1] > p)
            {
                lowest_points += p + 1;
            }
        }
    }

    lowest_points
}

fn flood_fill(basin_map: &mut Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
    let mut count = 0;
    let mut stack = vec![(x, y)];

    while !stack.is_empty() {
        let (x, y) = stack.pop().expect("we checked that the stack isn't empty");
        if basin_map[x][y] != 9 {
            basin_map[x][y] = 9;
            count += 1;
            if x != basin_map.len() - 1 {
                stack.push((x + 1, y));
            }
            if y != basin_map[x].len() - 1 {
                stack.push((x, y + 1));
            }
            if x != 0 {
                stack.push((x - 1, y));
            }
            if y != 0 {
                stack.push((x, y - 1));
            }
        }
    }

    count
}

fn part2(basin_map: &mut Vec<Vec<u32>>) -> u32 {
    let mut lowest_points = vec![];

    let max_x = basin_map.len() - 1;
    for x in 0..=max_x {
        let max_y = basin_map[x].len() - 1;
        for y in 0..=max_y {
            let p = basin_map[x][y];
            if (x == 0 || basin_map[x - 1][y] > p)
                && (y == 0 || basin_map[x][y - 1] > p)
                && (x == max_x || basin_map[x + 1][y] > p)
                && (y == max_y || basin_map[x][y + 1] > p)
            {
                lowest_points.push((x, y));
            }
        }
    }

    let mut basin_size = lowest_points
        .iter()
        .map(|(x, y)| flood_fill(basin_map, *x, *y))
        .collect::<Vec<_>>();

    basin_size.sort_by(|a, b| b.cmp(a));

    basin_size.iter().take(3).product()
}

fn main() -> Result<(), AoCError> {
    let start = Instant::now();

    let mut basin_map = parse(File::open("input")?)?;

    println!("parsing: {}µs", start.elapsed().as_micros());

    let s_part1 = Instant::now();
    println!(
        "part1: {} ({}µs)",
        part1(&basin_map),
        s_part1.elapsed().as_micros()
    );

    let s_part2 = Instant::now();
    println!(
        "part2: {} ({}µs)",
        part2(&mut basin_map),
        s_part2.elapsed().as_micros()
    );

    println!("Time: {}µs", start.elapsed().as_micros());

    Ok(())
}

#[test]
fn test_part1_example() {
    let basin_map = parse(File::open("example").unwrap()).unwrap();
    assert_eq!(part1(&basin_map), 15);
}

#[test]
fn test_part2_example() {
    let mut basin_map = parse(File::open("example").unwrap()).unwrap();
    assert_eq!(part2(&mut basin_map), 1134);
}
