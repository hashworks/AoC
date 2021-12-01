use std::error::Error;
use std::fs::File;
use std::io::{self, prelude::*};
use std::time::Instant;

/**
 * Nicer, but slower (4x)
 */
fn _part1_slow_zip(measurements: &Vec<usize>) -> usize {
    measurements
        .iter()
        .skip(1)
        .zip(measurements.iter())
        .fold(0, |acc, (m1, m2)| if m1 > m2 { acc + 1 } else { acc })
}

fn part1(measurements: &Vec<usize>) -> usize {
    let mut p = measurements.first().unwrap_or(&0);
    measurements.iter().skip(1).fold(0, |mut acc, m| {
        if m > p {
            acc += 1;
        }
        p = m;
        acc
    })
}

fn part2(measurements: &Vec<usize>) -> usize {
    part1(
        &measurements
            .iter()
            .zip(measurements.iter().skip(1))
            .zip(measurements.iter().skip(2))
            .map(|((m1, m2), m3)| m1 + m2 + m3)
            .collect(),
    )
}

fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();

    let file = File::open("input")?;

    let measurements = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap().parse::<usize>().unwrap())
        .collect();

    let s_part1 = Instant::now();
    println!(
        "part1: {} ({}ns)",
        part1(&measurements),
        s_part1.elapsed().as_nanos()
    );

    let s_part2 = Instant::now();
    println!(
        "part2: {} ({}µs)",
        part2(&measurements),
        s_part2.elapsed().as_micros()
    );

    println!("Time: {}µs", start.elapsed().as_micros());

    Ok(())
}

#[test]
fn test_part1_example1() {
    assert_eq!(0, part1(&[].to_vec()));
    assert_eq!(
        7,
        part1(&[199, 200, 208, 210, 200, 207, 240, 269, 260, 263].to_vec())
    );
}

#[test]
fn test_part1_slow_zip_example1() {
    assert_eq!(0, _part1_slow_zip(&[].to_vec()));
    assert_eq!(
        7,
        _part1_slow_zip(&[199, 200, 208, 210, 200, 207, 240, 269, 260, 263].to_vec())
    );
}

#[test]
fn test_part2_example1() {
    assert_eq!(0, part2(&[].to_vec()));
    assert_eq!(
        5,
        part2(&[199, 200, 208, 210, 200, 207, 240, 269, 260, 263].to_vec())
    );
}
