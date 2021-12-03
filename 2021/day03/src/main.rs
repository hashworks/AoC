use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::num::TryFromIntError;
use std::time::Instant;

#[derive(Debug)]
struct AoCError {
    kind: String,
    message: String,
    source: Option<Box<dyn Error>>,
}

impl fmt::Display for AoCError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AoCError [{}]: {}", self.kind, self.message)
    }
}

impl From<io::Error> for AoCError {
    fn from(error: io::Error) -> Self {
        AoCError {
            kind: String::from("io"),
            message: error.to_string(),
            source: Some(Box::new(error)),
        }
    }
}

impl From<TryFromIntError> for AoCError {
    fn from(error: TryFromIntError) -> Self {
        AoCError {
            kind: String::from("try_from"),
            message: error.to_string(),
            source: Some(Box::new(error)),
        }
    }
}

fn aoc_error(kind: &str, message: &str) -> AoCError {
    AoCError {
        kind: String::from(kind),
        message: String::from(message),
        source: None,
    }
}

fn parse_line(l: Result<String, io::Error>) -> Result<Vec<bool>, AoCError> {
    l?.chars()
        .map(|c| match c {
            '0' => Ok(false),
            '1' => Ok(true),
            c => Err(aoc_error("parse", format!("invalid bit '{}'", c).as_str())),
        })
        .collect::<Result<Vec<bool>, AoCError>>()
}

fn calculate_common_bits(bits_vec: &Vec<Vec<bool>>) -> Vec<bool> {
    let bit_count = match bits_vec.get(0) {
        Some(bits) => bits.len(),
        None => 0,
    };
    let mut common_bits = vec![0usize; bit_count];
    for bits in bits_vec {
        for i in 0..bits.len() {
            common_bits[i] += if bits[i] { 1 } else { 0 };
        }
    }
    common_bits
        .iter()
        .map(|&count| {
            let rest = bits_vec.len() - count;
            rest == count || rest < count
        })
        .collect::<Vec<bool>>()
}

fn calculate_common_bits_of_index(bits_vec: &Vec<Vec<bool>>, index: usize) -> bool {
    let count = bits_vec.iter().fold(0, |acc, bits| match bits.get(index) {
        Some(bit) => {
            if *bit {
                acc + 1
            } else {
                acc
            }
        }
        None => acc,
    });
    let rest = bits_vec.len() - count;
    rest == count || rest < count
}

fn binary_to_decimal(bits: &Vec<bool>) -> Result<usize, AoCError> {
    bits.iter().rev().enumerate().fold(Ok(0), |dec, (i, bit)| {
        if *bit {
            Ok(dec? + 2_usize.pow(i.try_into()?))
        } else {
            dec
        }
    })
}

fn invert_bits(bits: &Vec<bool>) -> Vec<bool> {
    bits.iter().map(|bit| !*bit).collect()
}

fn part1(bits_vec: &Vec<Vec<bool>>) -> Result<usize, AoCError> {
    let common_bits = calculate_common_bits(bits_vec);
    let gamma = binary_to_decimal(&common_bits)?;
    let epsilon = binary_to_decimal(&invert_bits(&common_bits))?;
    Ok(gamma * epsilon)
}

fn part2_filter(mut bits_vec: Vec<Vec<bool>>, common: bool) -> Result<Vec<bool>, AoCError> {
    for i in 0..bits_vec
        .get(0)
        .ok_or(aoc_error("option", "empty bits_vec"))?
        .len()
    {
        let common_bit = calculate_common_bits_of_index(&bits_vec, i);
        bits_vec = bits_vec
            .iter()
            .filter(|bits| {
                let bit = bits.get(i);
                bit.is_some()
                    && if common {
                        *bit.unwrap() == common_bit
                    } else {
                        *bit.unwrap() != common_bit
                    }
            })
            .map(|bits| bits.to_vec())
            .collect::<Vec<Vec<bool>>>();

        if bits_vec.len() == 1 {
            break;
        }
    }

    Ok(bits_vec
        .get(0)
        .ok_or(aoc_error("option", "empty bits_vec"))?
        .to_vec())
}

fn part2(bits_vec: &Vec<Vec<bool>>) -> Result<usize, AoCError> {
    let oxygen_bits_vec = part2_filter(bits_vec.to_vec(), true)?;
    let co2_bits_vec = part2_filter(bits_vec.to_vec(), false)?;

    Ok(binary_to_decimal(&oxygen_bits_vec)? * binary_to_decimal(&co2_bits_vec)?)
}

fn main() -> Result<(), AoCError> {
    let start = Instant::now();

    let file = File::open("input")?;

    let bits_vec = io::BufReader::new(file)
        .lines()
        .map(parse_line)
        .collect::<Result<Vec<_>, AoCError>>()?;

    println!("parsing: {}µs", start.elapsed().as_micros());

    let s_part1 = Instant::now();
    println!(
        "part1: {} ({}µs)",
        part1(&bits_vec)?,
        s_part1.elapsed().as_micros()
    );

    let s_part2 = Instant::now();
    println!(
        "part2: {} ({}µs)",
        part2(&bits_vec)?,
        s_part2.elapsed().as_micros()
    );

    println!("Time: {}µs", start.elapsed().as_micros());

    Ok(())
}

const _EXAMPLE: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

#[test]
fn test_calculate_common_bits_example() {
    let v = &_EXAMPLE
        .to_string()
        .lines()
        .map(|l| parse_line(Ok(l.to_string())).unwrap())
        .collect::<Vec<Vec<bool>>>();
    assert_eq!(
        vec![true, false, true, true, false],
        calculate_common_bits(v)
    );
    assert_eq!(true, calculate_common_bits_of_index(v, 0));
    assert_eq!(false, calculate_common_bits_of_index(v, 1));
    assert_eq!(true, calculate_common_bits_of_index(v, 2));
    assert_eq!(true, calculate_common_bits_of_index(v, 3));
    assert_eq!(false, calculate_common_bits_of_index(v, 4));
}

#[test]
fn test_binary_to_decimal() {
    assert_eq!(0, binary_to_decimal(&vec![]).unwrap());
    assert_eq!(
        22,
        binary_to_decimal(&vec![true, false, true, true, false]).unwrap()
    );
}

#[test]
fn test_part1() {
    assert_eq!(
        198,
        part1(
            &_EXAMPLE
                .to_string()
                .lines()
                .map(|l| parse_line(Ok(l.to_string())).unwrap())
                .collect()
        )
        .unwrap()
    );
}

#[test]
fn test_part2() {
    assert_eq!(
        230,
        part2(
            &_EXAMPLE
                .to_string()
                .lines()
                .map(|l| parse_line(Ok(l.to_string())).unwrap())
                .collect()
        )
        .unwrap()
    );
}
