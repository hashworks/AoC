use std::{fs::File, collections::HashMap};
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn valid_number(v: String, min: u32, max: u32) -> bool {
    if let Ok(n) = v.parse::<u32>() {
        n >= min && n <= max
    } else {
        false
    }
}

fn valid_height(v: String) -> bool {
    match v.chars().rev().take(2).collect::<String>().as_str() {
        "mc" => {
            valid_number(v.chars().take(3).collect::<String>(), 150, 193)
        },
        "ni" => {
            valid_number(v.chars().take(2).collect::<String>(), 59, 76)
        },
        _ => false
    }
}

fn valid_hex(v: String) -> bool {
    v.len() == 7 && v.chars().nth(0) == Some('#') && v.chars().skip(1).fold(true, |valid, c| valid && (c.is_numeric() || c.is_ascii_hexdigit()))
}

fn valid_ecl(v: String) -> bool {
    match v.as_str() {
        "amb" => true,
        "blu" => true,
        "brn" => true,
        "gry" => true,
        "grn" => true,
        "hzl" => true,
        "oth" => true,
        _ => false
    }
}

fn valid_pid(v: String) -> bool {
    v.len() == 9 && v.chars().fold(true, |valid, c| valid && c.is_numeric())
}

fn is_valid_property_map(property_map: HashMap<String, String>, validate_fields: bool) -> bool {
    property_map.len() == 7 && (!validate_fields || property_map.into_iter().fold(true, |valid, (p, v)| {
        valid && match p.as_str() {
            "byr" => valid_number(v, 1920, 2002),
            "iyr" => valid_number(v, 2010, 2020),
            "eyr" => valid_number(v, 2020, 2030),
            "hgt" => valid_height(v),
            "hcl" => valid_hex(v),
            "ecl" => valid_ecl(v),
            "pid" => valid_pid(v),
            _ => false
        }
    }))
}

fn batch_check(f: File, validate_fields: bool) -> i32 {
    let r = BufReader::new(f)
        .lines()
        .map(|r| r.unwrap())
        .fold((0, HashMap::<String, String>::new()), |(valid, property_map), l| {
            // Empty line, passport done. Validate and empty hashmap
            if l.is_empty() {
                if is_valid_property_map(property_map, validate_fields) {
                    (valid+1, HashMap::<String, String>::new())
                } else {
                    (valid, HashMap::<String, String>::new())
                }
            } else {
                let (_, mut property_map, property, value) = l.chars().into_iter().fold((true, property_map, String::new(), String::new()), |(is_property, mut property_map, mut property, mut value), char| {
                    match (is_property, char) {
                        // property finished, value starting
                        (true, ':') => (false, property_map, property, String::new()),
                        // value finished, new property starting
                        (false, ' ') => {
                            if property != "cid" {
                                property_map.insert(property, value);
                            }
                            (true, property_map, String::new(), String::new())
                        },
                        // build property
                        (true, char) => {
                            property.push(char);
                            (is_property, property_map, property, value)
                        },
                        // build value
                        _ => {
                            value.push(char);
                            (is_property, property_map, property, value)
                        }
                    }
                });
                // add last property
                if property != "cid" {
                    property_map.insert(property, value);
                }
                (valid, property_map)
            }
        });
    // validate last property_map
    if is_valid_property_map(r.1, validate_fields) {
        r.0+1
    } else {
        r.0
    }
}

fn main() {
    let s1 = Instant::now();

    println!("part1: {} ({}µs)", batch_check(File::open("./input").unwrap(), false), s1.elapsed().as_micros());

    let s2 = Instant::now();

    println!("part2: {} ({}µs)", batch_check(File::open("./input").unwrap(), true), s2.elapsed().as_micros());

    println!("Time: {}µs", s1.elapsed().as_micros());
}

#[test]
fn valid_number_works() {
    assert!(!valid_number("2024".to_owned(), 2025, 2026));
    assert!(valid_number("2025".to_owned(), 2025, 2026));
    assert!(valid_number("2026".to_owned(), 2025, 2026));
    assert!(!valid_number("2027".to_owned(), 2025, 2026));
}

#[test]
fn valid_height_works() {
    assert!(!valid_height("149cm".to_owned()));
    assert!(valid_height("193cm".to_owned()));
    assert!(!valid_height("58in".to_owned()));
    assert!(valid_height("76in".to_owned()));
}

#[test]
fn valid_hex_works() {
    assert!(valid_hex("#ffffff".to_owned()));
    assert!(valid_hex("#000000".to_owned()));
    assert!(valid_hex("#000fff".to_owned()));
    assert!(!valid_hex("a".to_owned()));
    assert!(!valid_hex("#".to_owned()));
    assert!(!valid_hex("#a".to_owned()));
    assert!(!valid_hex("#zzzzzz".to_owned()));
}

#[test]
fn valid_pid_works() {
    assert!(valid_pid("000000000".to_owned()));
    assert!(valid_pid("999999999".to_owned()));
    assert!(!valid_pid("aaaaaaaaa".to_owned()));
    assert!(!valid_pid("a".to_owned()));
    assert!(!valid_pid("0".to_owned()));
    assert!(!valid_pid("0000000001".to_owned()));
}