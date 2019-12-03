use std::cmp;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn part1(f: File, fuel: &dyn Fn(i32, i32) -> i32) -> i32 {
    BufReader::new(f)
        .lines()
        .map(|ms| ms.unwrap().parse().unwrap_or(0))
        .fold(0, fuel)
}

fn part2_recursive_fuel(acc: i32, m: i32) -> i32 {
    let m = cmp::max(m / 3 - 2, 0);
    acc + if m > 0 { part2_recursive_fuel(m, m) } else { 0 }
}

fn main() {
    let s1 = Instant::now();

    println!(
        "part1: {} ({}µs)",
        part1(File::open("./input").unwrap(), &|acc, m| acc + (m / 3 - 2)),
        s1.elapsed().as_micros()
    );

    let s2 = Instant::now();

    println!(
        "part2: {} ({}µs)",
        part1(File::open("./input").unwrap(), &part2_recursive_fuel),
        s2.elapsed().as_micros()
    );

    println!("Time: {}µs", s1.elapsed().as_micros());
}
