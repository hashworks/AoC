// Note: Since the "intcode computer" should be reused later
// it is implemented in the crate `../intcode`.
extern crate intcode;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str;
use std::time::Instant;

fn main() {
    let s1 = Instant::now();

    let m = BufReader::new(File::open("./input").unwrap())
        .split(b',')
        .map(|ops| {
            str::from_utf8(&ops.unwrap())
                .unwrap()
                .parse::<i32>()
                .unwrap_or(0)
        })
        .collect::<Vec<i32>>();

    let part1_m = &mut m.clone();
    let (_, part1_output) = intcode::compute(part1_m, &mut vec![1]);
    println!(
        "part1: {:?}, ({}µs)",
        part1_output,
        s1.elapsed().as_micros()
    );

    let s2 = Instant::now();

    let part2_m = &mut m.clone();
    let (_, part2_output) = intcode::compute(part2_m, &mut vec![5]);
    println!(
        "part2: {:?}, ({}µs)",
        part2_output,
        s2.elapsed().as_micros()
    );

    println!("time: {}µs", s1.elapsed().as_micros());
}
