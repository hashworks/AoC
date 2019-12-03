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
                .parse::<usize>()
                .unwrap_or(99)
        })
        .collect::<Vec<usize>>();

    println!(
        "part1: {}, ({}µs)",
        intcode::c(&mut m.clone(), 12, 2),
        s1.elapsed().as_micros()
    );

    let s2 = Instant::now();

    let (part2_noun, part2_verb) = intcode::bruteforce(m, 19690720).unwrap();
    println!(
        "part2: {} ({}ms)",
        100 * part2_noun + part2_verb,
        s2.elapsed().as_millis()
    );

    println!("Time: {}ms", s1.elapsed().as_millis());
}
