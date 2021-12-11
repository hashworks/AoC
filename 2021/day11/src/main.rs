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

fn parts(o_map: &mut Vec<Vec<u32>>, steps: Option<usize>) -> usize {
    let optional_target_flashes = o_map.len() * o_map[0].len();
    let mut flashes = 0;

    for step in 0..steps.unwrap_or(usize::max_value()) {
        let mut overloaded = vec![];
        for x in 0..o_map.len() {
            for y in 0..o_map[x].len() {
                if o_map[x][y] == 10 {
                    o_map[x][y] = 1;
                } else {
                    o_map[x][y] += 1;
                    if o_map[x][y] == 10 {
                        overloaded.push((x, y));
                    }
                }
            }
        }

        while !overloaded.is_empty() {
            let (x, y) = overloaded
                .pop()
                .expect("we checked that the stack isn't empty");
            flashes += 1;
            let x = x as i32;
            let y = y as i32;
            for (a_x, a_y) in [
                (x - 1, y),
                (x + 1, y),
                (x, y - 1),
                (x, y + 1),
                (x + 1, y + 1),
                (x - 1, y - 1),
                (x + 1, y - 1),
                (x - 1, y + 1),
            ]
            .iter()
            {
                if *a_x < 0
                    || *a_x >= o_map.len() as i32
                    || *a_y < 0
                    || *a_y >= o_map[0].len() as i32
                {
                    continue;
                }
                let (a_x, a_y) = (*a_x as usize, *a_y as usize);
                if o_map[a_x][a_y] != 10 {
                    o_map[a_x][a_y] += 1;
                    if o_map[a_x][a_y] == 10 {
                        overloaded.push((a_x, a_y));
                    }
                }
            }
        }

        if steps.is_none() {
            if flashes == optional_target_flashes {
                return step + 1;
            } else {
                flashes = 0;
            }
        }
    }

    flashes
}

fn main() -> Result<(), AoCError> {
    let start = Instant::now();

    let mut o_map = parse(File::open("input")?)?;

    println!("parsing: {}µs", start.elapsed().as_micros());

    let s_part1 = Instant::now();
    println!(
        "part1: {} ({}µs)",
        parts(&mut o_map.clone(), Some(100)),
        s_part1.elapsed().as_micros()
    );

    let s_part2 = Instant::now();
    println!(
        "part2: {} ({}µs)",
        parts(&mut o_map, None),
        s_part2.elapsed().as_micros()
    );

    println!("Time: {}µs", start.elapsed().as_micros());

    Ok(())
}

#[test]
fn test_part1_example_single() {
    let mut o_map = parse(File::open("example_small").unwrap()).unwrap();
    assert_eq!(parts(&mut o_map, Some(1)), 9);
}

#[test]
fn test_part1_example_small() {
    let mut o_map = parse(File::open("example_normal").unwrap()).unwrap();
    assert_eq!(parts(&mut o_map, Some(10)), 204);
}

#[test]
fn test_part1_example_big() {
    let mut o_map = parse(File::open("example_normal").unwrap()).unwrap();
    assert_eq!(parts(&mut o_map, Some(100)), 1656);
}

#[test]
fn test_part2_example() {
    let mut o_map = parse(File::open("example_normal").unwrap()).unwrap();
    assert_eq!(parts(&mut o_map, None), 195);
}
