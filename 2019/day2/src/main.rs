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
    part1_m[1] = 12;
    part1_m[2] = 2;
    let (part1_m0, _) = intcode::compute(part1_m, &mut vec![]);
    println!("part1: {}, ({}µs)", part1_m0, s1.elapsed().as_micros());

    let s2 = Instant::now();

    let (part2_noun, part2_verb) = bruteforce(m, 19690720).unwrap();
    println!(
        "part2: {} ({}µs)",
        100 * part2_noun + part2_verb,
        s2.elapsed().as_micros()
    );

    println!("Time: {}µs", s1.elapsed().as_micros());
}

pub fn bruteforce(m: Vec<i32>, expected_m0: i32) -> Option<(i32, i32)> {
    for noun in 0..99 {
        for verb in 0..99 {
            let m = &mut m.clone();
            m[1] = noun;
            m[2] = verb;
            let (m0, _) = intcode::compute(m, &mut vec![]);
            if m0 == expected_m0 {
                return Some((noun, verb));
            }
        }
    }
    None
}
