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
                .trim()
                .parse::<i64>()
                .unwrap()
        })
        .collect::<Vec<i64>>();

    let (_, part1_out) = intcode::compute(m.clone(), vec![1]);

    println!("part1: {:?}, ({}µs)", part1_out, s1.elapsed().as_micros());

    let s2 = Instant::now();

    let (_, part2_out) = intcode::compute(m, vec![2]);

    println!("part2: {:?}, ({}µs)", part2_out, s2.elapsed().as_micros());

    println!("time: {}µs", s1.elapsed().as_micros());
}
