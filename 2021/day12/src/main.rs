use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::time::Instant;
use thiserror::Error;

#[derive(Error, Debug)]
enum AoCError {
    #[error("io error")]
    IoErr(#[from] io::Error),
    #[error("failed to parse bad input")]
    ParseBadInputErr,
    #[error("failed to solve bad input")]
    SolveBadInputErr,
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
enum Cave {
    Start,
    End,
    Big(String),
    Small(String),
}

impl Cave {
    fn new(s: &str) -> Result<Self, AoCError> {
        if s == "start" {
            Ok(Cave::Start)
        } else if s == "end" {
            Ok(Cave::End)
        } else {
            let first_char = s.chars().next().ok_or(AoCError::ParseBadInputErr)?;
            if 'a' <= first_char && first_char <= 'z' {
                Ok(Cave::Small(s.to_string()))
            } else {
                Ok(Cave::Big(s.to_string()))
            }
        }
    }
}

type CaveSystem = HashMap<Cave, HashSet<Cave>>;

fn insert_with(cave_system: &mut CaveSystem, a: Cave, b: Cave) {
    if let Some(neighbors) = cave_system.get_mut(&b) {
        neighbors.insert(a);
    } else {
        cave_system.insert(b, HashSet::from([a]));
    }
}

fn parse(file: File) -> Result<CaveSystem, AoCError> {
    let mut cave_system: CaveSystem = HashMap::new();

    for l in io::BufReader::new(file).lines() {
        let l = l?;
        let (a, b) = l.split_once('-').ok_or(AoCError::ParseBadInputErr)?;
        let (a, b) = (Cave::new(&a)?, Cave::new(&b)?);

        insert_with(&mut cave_system, a.clone(), b.clone());
        insert_with(&mut cave_system, b, a);
    }

    Ok(cave_system)
}

fn parts(cave_system: &CaveSystem, allow_joker: bool) -> Result<usize, AoCError> {
    let mut count = 0;
    let mut paths = vec![(&false, vec![&Cave::Start])];

    while let Some((joker, path)) = paths.pop() {
        let last_cave = path.last().ok_or(AoCError::SolveBadInputErr)?;
        let neighbors = cave_system
            .get(last_cave)
            .ok_or(AoCError::SolveBadInputErr)?;

        for neighbor in neighbors {
            match neighbor {
                Cave::Start => {
                    continue;
                }
                Cave::End => {
                    count += 1;
                    continue;
                }
                _ => {
                    let joker = match neighbor {
                        Cave::Small(_) => {
                            if path.contains(&neighbor) {
                                if allow_joker && !joker {
                                    &true
                                } else {
                                    continue;
                                }
                            } else {
                                &joker
                            }
                        }
                        _ => joker,
                    };
                    let mut new_path = path.clone();
                    new_path.push(neighbor);
                    paths.push((&joker, new_path));
                }
            }
        }
    }

    Ok(count)
}

fn main() -> Result<(), AoCError> {
    let start = Instant::now();

    let cave_system = parse(File::open("input")?)?;

    println!("parsing: {}µs", start.elapsed().as_micros());

    let s_part1 = Instant::now();
    println!(
        "part1: {} ({}µs)",
        parts(&cave_system, false)?,
        s_part1.elapsed().as_micros()
    );

    let s_part2 = Instant::now();
    println!(
        "part2: {} ({}µs)",
        parts(&cave_system, true)?,
        s_part2.elapsed().as_micros()
    );

    println!("Time: {}µs", start.elapsed().as_micros());

    Ok(())
}

#[test]
fn test_part1_example_small() {
    let cave_system = parse(File::open("example_small").unwrap()).unwrap();
    assert_eq!(parts(&cave_system, false).unwrap(), 10);
}

#[test]
fn test_part1_example_medium() {
    let cave_system = parse(File::open("example_medium").unwrap()).unwrap();
    assert_eq!(parts(&cave_system, false).unwrap(), 19);
}

#[test]
fn test_part1_example_big() {
    let cave_system = parse(File::open("example_big").unwrap()).unwrap();
    assert_eq!(parts(&cave_system, false).unwrap(), 226);
}

#[test]
fn test_part2_example_small() {
    let cave_system = parse(File::open("example_small").unwrap()).unwrap();
    assert_eq!(parts(&cave_system, true).unwrap(), 36);
}

#[test]
fn test_part2_example_medium() {
    let cave_system = parse(File::open("example_medium").unwrap()).unwrap();
    assert_eq!(parts(&cave_system, true).unwrap(), 103);
}

#[test]
fn test_part2_example_big() {
    let cave_system = parse(File::open("example_big").unwrap()).unwrap();
    assert_eq!(parts(&cave_system, true).unwrap(), 3509);
}
