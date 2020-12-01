use itertools::Itertools;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn part1(f: File) -> i32 {
    let numbers: HashSet<i32> = BufReader::new(f)
        .lines()
        .map(|nr| nr.unwrap().parse::<i32>().unwrap())
        .collect();

    return numbers
        .iter()
        .map(|a| (a, 2020 - *a))
        .filter(|(_, b)| numbers.contains(b))
        .map(|(a, b)| a * b)
        .next()
        .unwrap();
}

fn part2(f: File) -> i32 {
    let numbers: HashSet<i32> = BufReader::new(f)
        .lines()
        .map(|nr| nr.unwrap().parse::<i32>().unwrap())
        .collect();

    return numbers
        .iter()
        .cartesian_product(numbers.iter())
        .map(|(a, b)| (a, b, 2020 - *a - *b))
        .filter(|(_, _, c)| numbers.contains(c))
        .map(|(a, b, c)| a * b * c)
        .next()
        .unwrap();
}

fn main() {
    let s1 = Instant::now();

    println!(
        "part1: {} ({}µs)",
        part1(File::open("./input").unwrap()),
        s1.elapsed().as_micros()
    );

    let s2 = Instant::now();

    println!(
        "part2: {} ({}µs)",
        part2(File::open("./input").unwrap()),
        s2.elapsed().as_micros()
    );

    println!("Time: {}µs", s1.elapsed().as_micros());
}
