use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::num::ParseIntError;
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

impl From<ParseIntError> for AoCError {
    fn from(error: ParseIntError) -> Self {
        AoCError {
            kind: String::from("parse"),
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

#[derive(Clone, Debug)]
enum Instruction {
    Forward,
    Up,
    Down,
}

fn part1(instructions: &Vec<(Instruction, i64)>) -> i64 {
    let (h, d) = instructions.iter().fold((0, 0), |(h, d), (c, i)| match c {
        Instruction::Forward => (h + i, d),
        Instruction::Up => (h, d - i),
        Instruction::Down => (h, d + i),
    });
    h * d
}

fn part2(instructions: &Vec<(Instruction, i64)>) -> i64 {
    let (h, d, _) = instructions
        .iter()
        .fold((0, 0, 0), |(h, d, a), (c, i)| match c {
            Instruction::Forward => (h + i, d + i * a, a),
            Instruction::Up => (h, d, a - i),
            Instruction::Down => (h, d, a + i),
        });
    h * d
}

fn parse_line(l: Result<String, io::Error>) -> Result<(Instruction, i64), AoCError> {
    let line = l?;
    let (instr, vstr) = line
        .split_once(' ')
        .ok_or(aoc_error("option", "split_once failed"))?;
    let instruction_char = instr
        .chars()
        .next()
        .ok_or(aoc_error("option", "failed to read the first char"))?;
    let instruction = match instruction_char {
        'u' => Instruction::Up,
        'd' => Instruction::Down,
        'f' => Instruction::Forward,
        c => {
            return Err(aoc_error(
                "parse",
                format!("invalid instruction '{}'", c).as_str(),
            ))
        }
    };
    Ok((instruction, vstr.parse()?))
}

fn main() -> Result<(), AoCError> {
    let start = Instant::now();

    let file = File::open("input")?;

    let s_parse = Instant::now();
    let instructions = io::BufReader::new(file)
        .lines()
        .map(parse_line)
        .collect::<Result<Vec<(Instruction, i64)>, AoCError>>()?;
    println!("parsing: {}µs", s_parse.elapsed().as_micros());

    let s_part1 = Instant::now();
    println!(
        "part1: {} ({}ns)",
        part1(&instructions),
        s_part1.elapsed().as_nanos()
    );

    let s_part2 = Instant::now();
    println!(
        "part2: {} ({}ns)",
        part2(&instructions),
        s_part2.elapsed().as_nanos()
    );

    println!("Time: {}µs", start.elapsed().as_micros());

    Ok(())
}

const _EXAMPLE: [(Instruction, i64); 6] = [
    (Instruction::Forward, 5),
    (Instruction::Down, 5),
    (Instruction::Forward, 8),
    (Instruction::Up, 3),
    (Instruction::Down, 8),
    (Instruction::Forward, 2),
];

#[test]
fn test_part1_example() {
    assert_eq!(150, part1(&_EXAMPLE.to_vec()));
}

#[test]
fn test_part2_example() {
    assert_eq!(900, part2(&_EXAMPLE.to_vec()));
}
