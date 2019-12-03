// Note: Since the "intcode computer" should be reused later
// it is implemented in the crate `../intcode`.
extern crate intcode;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str;

fn main() {
    let m = BufReader::new(File::open("./input").unwrap())
        .split(b',')
        .map(|ops| {
            str::from_utf8(&ops.unwrap())
                .unwrap()
                .parse::<usize>()
                .unwrap_or(99)
        })
        .collect::<Vec<usize>>();

    println!("part1: {}", intcode::c(&mut m.clone(), 12, 2));
    let (part2_noun, part2_verb) = intcode::bruteforce(m, 19690720).unwrap();
    println!("part2: {}", 100 * part2_noun + part2_verb);
}
