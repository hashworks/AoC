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

type DigitOutput = [(usize, u8); 4];

struct SignalPattern {
    one: u8,
    seven: u8,
    four: u8,
    two_three_five: Vec<u8>,
    zero_six_nine: Vec<u8>,
    eight: u8,
}

impl SignalPattern {
    fn new() -> Self {
        Self {
            one: 0,
            four: 0,
            seven: 0,
            eight: 0,
            two_three_five: vec![],
            zero_six_nine: vec![],
        }
    }
}

fn parse_str_to_u8(s: &str) -> (usize, u8) {
    (
        s.len(),
        s.chars()
            .into_iter()
            .map(|c| match c {
                'a' => 0b00000001,
                'b' => 0b00000010,
                'c' => 0b00000100,
                'd' => 0b00001000,
                'e' => 0b00010000,
                'f' => 0b00100000,
                'g' => 0b01000000,
                _ => 0,
            })
            .sum(),
    )
}

fn parse(file: File) -> Result<Vec<(SignalPattern, DigitOutput)>, AoCError> {
    io::BufReader::new(file)
        .lines()
        .map(|l| {
            let line = l?;
            let patterns = line.split_ascii_whitespace().collect::<Vec<&str>>();
            if patterns.len() != 15 {
                return Err(AoCError::ParseBadInputErr);
            }

            let mut signal_pattern = SignalPattern::new();

            for i in 0..10 {
                let pattern = patterns[i];
                let (length, bits) = parse_str_to_u8(pattern);
                match length {
                    2 => signal_pattern.one = bits,
                    4 => signal_pattern.four = bits,
                    3 => signal_pattern.seven = bits,
                    7 => signal_pattern.eight = bits,
                    5 => signal_pattern.two_three_five.push(bits),
                    6 => signal_pattern.zero_six_nine.push(bits),
                    _ => return Err(AoCError::ParseBadInputErr),
                }
            }

            if signal_pattern.two_three_five.len() != 3 || signal_pattern.zero_six_nine.len() != 3 {
                return Err(AoCError::ParseBadInputErr);
            }

            Ok((
                signal_pattern,
                [
                    parse_str_to_u8(patterns[11]),
                    parse_str_to_u8(patterns[12]),
                    parse_str_to_u8(patterns[13]),
                    parse_str_to_u8(patterns[14]),
                ],
            ))
        })
        .collect()
}

fn part1(entries: &Vec<(SignalPattern, DigitOutput)>) -> usize {
    entries
        .iter()
        .map(|(_, d_o_v)| {
            d_o_v
                .iter()
                .filter(|bits| bits.0 == 2 || bits.0 == 4 || bits.0 == 3 || bits.0 == 7)
                .count()
        })
        .sum()
}

fn part2(entries: &Vec<(SignalPattern, DigitOutput)>) -> usize {
    entries
        .iter()
        .map(|(s_p, d_o_v)| {
            // four without one will map to [b,d]
            let b_d = s_p.four - s_p.one;

            // two_three_five with b_d is five
            let five = if s_p.two_three_five[0] & b_d == b_d {
                s_p.two_three_five[0]
            } else if s_p.two_three_five[1] & b_d == b_d {
                s_p.two_three_five[1]
            } else {
                s_p.two_three_five[2]
            };

            // two_three_five with one is three
            let three = if s_p.two_three_five[0] & s_p.one == s_p.one {
                s_p.two_three_five[0]
            } else if s_p.two_three_five[1] & s_p.one == s_p.one {
                s_p.two_three_five[1]
            } else {
                s_p.two_three_five[2]
            };

            // remaining is two
            let two = if s_p.two_three_five[0] != three && s_p.two_three_five[0] != five {
                s_p.two_three_five[0]
            } else if s_p.two_three_five[1] != three && s_p.two_three_five[1] != five {
                s_p.two_three_five[1]
            } else {
                s_p.two_three_five[2]
            };

            // zero_six_nine & one is not one for six
            let six = if s_p.zero_six_nine[0] & s_p.one != s_p.one {
                s_p.zero_six_nine[0]
            } else if s_p.zero_six_nine[1] & s_p.one != s_p.one {
                s_p.zero_six_nine[1]
            } else {
                s_p.zero_six_nine[2]
            };

            // 0b1111111 - six is c
            let c = 0b1111111 - six;

            // nine - c equals five
            let nine = if s_p.zero_six_nine[0] != six && s_p.zero_six_nine[0] - c == five {
                s_p.zero_six_nine[0]
            } else if s_p.zero_six_nine[1] != six && s_p.zero_six_nine[1] - c == five {
                s_p.zero_six_nine[1]
            } else {
                s_p.zero_six_nine[2]
            };

            // remaining must be zero

            d_o_v
                .iter()
                .rev()
                .enumerate()
                .map(|(i, (_, bits))| {
                    let bits = *bits;

                    let r = if bits == s_p.one {
                        1
                    } else if bits == two {
                        2
                    } else if bits == three {
                        3
                    } else if bits == s_p.four {
                        4
                    } else if bits == five {
                        5
                    } else if bits == six {
                        6
                    } else if bits == s_p.seven {
                        7
                    } else if bits == s_p.eight {
                        8
                    } else if bits == nine {
                        9
                    } else {
                        0
                    };
                    r * 10usize.pow(i as u32)
                })
                .sum::<usize>()
        })
        .sum()
}

fn main() -> Result<(), AoCError> {
    let start = Instant::now();

    let entries = parse(File::open("input")?)?;

    println!("parsing: {}µs", start.elapsed().as_micros());

    let s_part1 = Instant::now();
    println!(
        "part1: {} ({}ns)",
        part1(&entries),
        s_part1.elapsed().as_nanos()
    );

    let s_part2 = Instant::now();
    println!(
        "part2: {} ({}µs)",
        part2(&entries),
        s_part2.elapsed().as_micros()
    );

    println!("Time: {}µs", start.elapsed().as_micros());

    Ok(())
}

#[test]
fn test_part1_example() {
    let entries = parse(File::open("example").unwrap()).unwrap();
    assert_eq!(part1(&entries), 26);
}

#[test]
fn test_part2_example() {
    let entries = parse(File::open("example").unwrap()).unwrap();
    assert_eq!(part2(&entries), 61229);
}
