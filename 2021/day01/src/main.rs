use std::error::Error;
use std::fs::File;
use std::io::{self, prelude::*};
use std::time::Instant;

/**
 * Nicer, but slower (3x)
 */
fn _part1_slow_zip(measurements: &Vec<usize>) -> usize {
    measurements
        .iter()
        .skip(1)
        .zip(measurements.iter())
        .filter(|(m1, m2)| m1 > m2)
        .count()
}

fn _part1_filter(measurements: &Vec<usize>) -> usize {
    let mut p = measurements.first().unwrap_or(&0);
    measurements
        .iter()
        .skip(1)
        .filter(|m| {
            let f = m > &p;
            p = m;
            f
        })
        .count()
}

fn _part2_zip(measurements: &Vec<usize>) -> usize {
    _part1_filter(
        &measurements
            .iter()
            .zip(measurements.iter().skip(1))
            .zip(measurements.iter().skip(2))
            .map(|((m1, m2), m3)| m1 + m2 + m3)
            .collect(),
    )
}

// https://doc.rust-lang.org/std/vec/struct.Vec.html#method.windows

fn part1_windows(measurements: &Vec<usize>) -> usize {
    measurements.windows(2).filter(|&w| w[0] < w[1]).count()
}

fn part2_windows(measurements: &Vec<usize>) -> usize {
    part1_windows(
        &measurements
            .windows(3)
            .map(|w| w.iter().sum())
            .collect::<Vec<usize>>(),
    )
}

fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();

    let file = File::open("input")?;

    let measurements = io::BufReader::new(file)
        .lines()
        .map(|l| Ok(l?.parse()?))
        .collect::<Result<Vec<usize>, Box<dyn Error>>>()?;

    let s_part1 = Instant::now();
    println!(
        "part1: {} ({}ns)",
        part1_windows(&measurements),
        s_part1.elapsed().as_nanos()
    );

    let s_part2 = Instant::now();
    println!(
        "part2: {} ({}µs)",
        part2_windows(&measurements),
        s_part2.elapsed().as_micros()
    );

    println!("Time: {}µs", start.elapsed().as_micros());

    Ok(())
}

const _EXAMPLE: [usize; 10] = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

#[test]
fn test_part1_example() {
    assert_eq!(0, _part1_filter(&[].to_vec()));
    assert_eq!(7, _part1_filter(&_EXAMPLE.to_vec()));
}

#[test]
fn test_part1_slow_zip_example() {
    assert_eq!(0, _part1_slow_zip(&[].to_vec()));
    assert_eq!(7, _part1_slow_zip(&_EXAMPLE.to_vec()));
}

#[test]
fn test_part2_example() {
    assert_eq!(0, _part2_zip(&[].to_vec()));
    assert_eq!(5, _part2_zip(&_EXAMPLE.to_vec()));
}

#[test]
fn test_part1_windows_example() {
    assert_eq!(0, part1_windows(&[].to_vec()));
    assert_eq!(7, part1_windows(&_EXAMPLE.to_vec()));
}

#[test]
fn test_part2_windows_example() {
    assert_eq!(0, part2_windows(&[].to_vec()));
    assert_eq!(5, part2_windows(&_EXAMPLE.to_vec()));
}
