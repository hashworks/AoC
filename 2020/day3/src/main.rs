use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn parse(f: File) -> Vec<Vec<bool>> {
    BufReader::new(f)
        .lines()
        .map(|r| r.unwrap())
        .map(|l| l.chars().map(|c| c == '#').collect())
        .collect()
}

fn calculate_slope(slope: &Vec<Vec<bool>>, right: usize, down: usize) -> u32 {
    slope.iter()
        .step_by(down)
        .fold((0, 0), |(pos, trees), l| {
            (pos+right, if *l.get(pos % l.len()).unwrap() {
                trees+1
            } else {
                trees
            })
        }).1
}

fn main() {
    let s1 = Instant::now();

    let slope = parse(File::open("./input").unwrap());

    let part1 = calculate_slope(&slope, 3, 1);

    println!(
        "part1: {} ({}µs)",
        part1,
        s1.elapsed().as_micros()
    );

    let s2 = Instant::now();

    let part2 = part1 *
        [(1usize,1usize),(5,1),(7,1),(1,2)]
            .iter()
            .map(|(r, d)| calculate_slope(&slope, *r, *d))
            .product::<u32>();

    println!(
        "part2: {} ({}µs)",
        part2,
        s2.elapsed().as_micros()
    );

    println!("Time: {}µs", s1.elapsed().as_micros());
}
