mod trench;

use std::fs::File;
use std::io::{self, BufRead};
use std::time::Instant;
use thiserror::Error;
use trench::GameOfTrench;

#[derive(Error, Debug)]
enum AoCError {
    #[error("io error")]
    IoErr(#[from] io::Error),
    #[error("failed to parse int")]
    ParseIntErr(#[from] std::num::ParseIntError),
}

fn parse(file: File) -> Result<GameOfTrench, AoCError> {
    Ok(io::BufReader::new(file)
        .lines()
        .map(|l| Ok(l?))
        .collect::<Result<Vec<String>, AoCError>>()?
        .into())
}

fn main() -> Result<(), AoCError> {
    let start = Instant::now();

    let mut game_of_trench = parse(File::open("input")?)?;

    println!("parsing: {}µs", start.elapsed().as_micros());

    let s_part1 = Instant::now();
    game_of_trench.play(2);
    println!(
        "part1: {} ({}µs)",
        game_of_trench.count_pixels(),
        s_part1.elapsed().as_micros()
    );

    let s_part2 = Instant::now();
    game_of_trench.play(48);
    println!(
        "part2: {} ({}µs)",
        game_of_trench.count_pixels(),
        s_part2.elapsed().as_micros()
    );

    println!("Time: {}µs", start.elapsed().as_micros());

    Ok(())
}

#[test]
fn test_example() {
    let mut game_of_trench = parse(File::open("example").unwrap()).unwrap();
    game_of_trench.play(2);
    assert_eq!(game_of_trench.count_pixels(), 35);
    game_of_trench.play(48);
    assert_eq!(game_of_trench.count_pixels(), 3351);
}
