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
    #[error("bad input parse error")]
    BadInputSolveErr,
}

fn map_hex(c: char) -> Result<(bool, bool, bool, bool), AoCError> {
    match c {
        '0' => Ok((false, false, false, false)),
        '1' => Ok((false, false, false, true)),
        '2' => Ok((false, false, true, false)),
        '3' => Ok((false, false, true, true)),
        '4' => Ok((false, true, false, false)),
        '5' => Ok((false, true, false, true)),
        '6' => Ok((false, true, true, false)),
        '7' => Ok((false, true, true, true)),
        '8' => Ok((true, false, false, false)),
        '9' => Ok((true, false, false, true)),
        'A' => Ok((true, false, true, false)),
        'B' => Ok((true, false, true, true)),
        'C' => Ok((true, true, false, false)),
        'D' => Ok((true, true, false, true)),
        'E' => Ok((true, true, true, false)),
        'F' => Ok((true, true, true, true)),
        _ => Err(AoCError::BadInputParseErr),
    }
}

fn parse(file: File) -> Result<Vec<bool>, AoCError> {
    let mut bits = vec![];
    for char in io::BufReader::new(file)
        .lines()
        .next()
        .ok_or(AoCError::BadInputParseErr)??
        .chars()
    {
        let (a, b, c, d) = map_hex(char)?;
        bits.push(a);
        bits.push(b);
        bits.push(c);
        bits.push(d);
    }
    bits.reverse();
    Ok(bits)
}

fn bits_to_int(bits: &Vec<bool>) -> usize {
    let mut int = 0;
    for (i, &bit) in bits.iter().rev().enumerate() {
        if bit {
            int += 2_usize.pow(i as u32);
        }
    }
    int
}

fn handle_literal_value(bits: &mut Vec<bool>) -> Result<(usize, usize), AoCError> {
    let mut bits_handled = 0;
    let mut value_bits = vec![];
    loop {
        let not_last_group = bits.pop().ok_or(AoCError::BadInputSolveErr)?;

        value_bits.push(bits.pop().ok_or(AoCError::BadInputSolveErr)?);
        value_bits.push(bits.pop().ok_or(AoCError::BadInputSolveErr)?);
        value_bits.push(bits.pop().ok_or(AoCError::BadInputSolveErr)?);
        value_bits.push(bits.pop().ok_or(AoCError::BadInputSolveErr)?);

        bits_handled += 5;

        if !not_last_group {
            break;
        }
    }
    Ok((bits_handled, bits_to_int(&value_bits)))
}

fn handle_operator_length_type_0(
    bits: &mut Vec<bool>,
) -> Result<(usize, usize, Vec<usize>), AoCError> {
    let mut version_sum = 0;
    let mut bits_handled = 0;
    let mut values = vec![];

    let mut total_length_in_bits = vec![];
    for _ in 0..15 {
        total_length_in_bits.push(bits.pop().ok_or(AoCError::BadInputSolveErr)?);
    }
    let total_length_in_bits = bits_to_int(&total_length_in_bits);

    while bits_handled < total_length_in_bits {
        let (new_version_sum, new_bits_handled, value) = handle_package(bits)?;
        version_sum += new_version_sum;
        bits_handled += new_bits_handled;
        values.push(value);
    }

    Ok((version_sum, total_length_in_bits + 15, values))
}

fn handle_operator_length_type_1(
    bits: &mut Vec<bool>,
) -> Result<(usize, usize, Vec<usize>), AoCError> {
    let mut version_sum = 0;
    let mut bits_handled = 0;
    let mut packages_handled = 0;
    let mut values = vec![];

    let mut number_of_subpackages = vec![];
    for _ in 0..11 {
        number_of_subpackages.push(bits.pop().ok_or(AoCError::BadInputSolveErr)?);
    }
    let number_of_subpackages = bits_to_int(&number_of_subpackages);

    while packages_handled < number_of_subpackages {
        let (new_version_sum, new_bits_handled, value) = handle_package(bits)?;
        version_sum += new_version_sum;
        bits_handled += new_bits_handled;
        packages_handled += 1;
        values.push(value);
    }

    Ok((version_sum, bits_handled + 11, values))
}

fn handle_package(bits: &mut Vec<bool>) -> Result<(usize, usize, usize), AoCError> {
    let version = bits_to_int(&vec![
        bits.pop().ok_or(AoCError::BadInputSolveErr)?,
        bits.pop().ok_or(AoCError::BadInputSolveErr)?,
        bits.pop().ok_or(AoCError::BadInputSolveErr)?,
    ]);
    let type_id = bits_to_int(&vec![
        bits.pop().ok_or(AoCError::BadInputSolveErr)?,
        bits.pop().ok_or(AoCError::BadInputSolveErr)?,
        bits.pop().ok_or(AoCError::BadInputSolveErr)?,
    ]);

    if type_id == 4 {
        let (bits_handled, value) = handle_literal_value(bits)?;
        Ok((version, bits_handled + 6, value))
    } else {
        let length_type_id = bits.pop().ok_or(AoCError::BadInputSolveErr)?;
        let (new_version, bits_handled, values) = if length_type_id {
            handle_operator_length_type_1(bits)?
        } else {
            handle_operator_length_type_0(bits)?
        };
        Ok((
            version + new_version,
            bits_handled + 7,
            match type_id {
                0 => values.iter().sum(),
                1 => values.iter().product(),
                2 => *values.iter().min().ok_or(AoCError::BadInputSolveErr)?,
                3 => *values.iter().max().ok_or(AoCError::BadInputSolveErr)?,
                _ => {
                    if values.len() != 2 {
                        return Err(AoCError::BadInputSolveErr)?;
                    }
                    if match type_id {
                        5 => values[0] > values[1],
                        6 => values[0] < values[1],
                        7 => values[0] == values[1],
                        _ => return Err(AoCError::BadInputSolveErr)?,
                    } {
                        1
                    } else {
                        0
                    }
                }
            },
        ))
    }
}

fn main() -> Result<(), AoCError> {
    let start = Instant::now();

    let mut bits = parse(File::open("input")?)?;

    println!("parsing: {}µs", start.elapsed().as_micros());

    let s_parts = Instant::now();
    let (version_sum, _, value) = handle_package(&mut bits)?;
    println!("part1: {}", version_sum);
    println!("part2: {} ({}µs)", value, s_parts.elapsed().as_micros());

    println!("Time: {}µs", start.elapsed().as_micros());

    Ok(())
}

#[test]
fn test_example_literal_package() {
    let mut example_literal_package =
        parse(File::open("example_literal_package").unwrap()).unwrap();
    assert_eq!(
        handle_package(&mut example_literal_package).unwrap(),
        (6, 21, 2021)
    );
}

#[test]
fn test_example_1() {
    let mut example_1 = parse(File::open("example_1").unwrap()).unwrap();
    let (version_sum, _, _) = handle_package(&mut example_1).unwrap();
    assert_eq!(version_sum, 16);
}

#[test]
fn test_example_2() {
    let mut example_2 = parse(File::open("example_2").unwrap()).unwrap();
    let (version_sum, _, _) = handle_package(&mut example_2).unwrap();
    assert_eq!(version_sum, 12);
}

#[test]
fn test_example_3() {
    let mut example_3 = parse(File::open("example_3").unwrap()).unwrap();
    let (version_sum, _, _) = handle_package(&mut example_3).unwrap();
    assert_eq!(version_sum, 23);
}

#[test]
fn test_example_4() {
    let mut example_4 = parse(File::open("example_4").unwrap()).unwrap();
    let (version_sum, _, _) = handle_package(&mut example_4).unwrap();
    assert_eq!(version_sum, 31);
}
