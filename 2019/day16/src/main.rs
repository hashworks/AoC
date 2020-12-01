use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn phase(input: Vec<i64>) -> Vec<i64> {
    (1..input.len() + 1)
        .map(|base_level| {
            input
                .clone()
                .into_iter()
                .zip(
                    [0, 1, 0, -1]
                        .into_iter()
                        .map(|base| vec![base; base_level])
                        .fold(vec![], |vec, bases| {
                            vec.into_iter().chain(bases.into_iter()).collect()
                        })
                        .into_iter()
                        .cycle()
                        .skip(1),
                )
                .filter(|(integer, base)| integer != &0 && base != &&0)
                .fold(0, |acc, (integer, base)| acc + integer * base)
                .abs()
                % 10
        })
        .collect()
}

fn offset(input: Vec<i64>) -> usize {
    (input[0] * 1000000
        + input[1] * 100000
        + input[2] * 10000
        + input[3] * 1000
        + input[4] * 100
        + input[5] * 10
        + input[6]) as usize
}

fn main() {
    let s1 = Instant::now();

    let input = BufReader::new(File::open("./input_part2_test1").unwrap())
        .lines()
        .map(|l| l.unwrap())
        .take(1)
        .collect::<Vec<String>>()[0]
        .chars()
        .map(|c| c as i64 - 48)
        .collect::<Vec<i64>>();

    let mut phased = input.clone();
    for _ in 0..100 {
        phased = phase(phased);
    }

    println!(
        "part1: {:?}, ({}µs)",
        phased.into_iter().take(8).collect::<Vec<i64>>(),
        s1.elapsed().as_micros()
    );

    let s2 = Instant::now();

    let offset = offset(input.clone().into_iter().take(7).collect::<Vec<i64>>());
    let initial_length = input.len() + 1;

    let mut phased = input
        .into_iter()
        .cycle()
        .take(initial_length * 10000)
        .collect::<Vec<i64>>();
    println!("Final length: {}", phased.len());
    for p in 1..101 {
        println!("Phase {}/100…", p);
        phased = phase(phased);
    }

    println!(
        "part2: {:?}, ({}µs)",
        phased
            .into_iter()
            .skip(offset)
            .take(8)
            .collect::<Vec<i64>>(),
        s2.elapsed().as_micros()
    );

    println!("time: {}µs", s1.elapsed().as_micros());
}
