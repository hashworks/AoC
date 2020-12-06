use std::collections::HashSet;
use std::fs::read_to_string;
use std::time::Instant;

fn _part1_slow(qd: &String) -> usize {
    qd.split("\n\n")
        .map(|group| {
            group
                .chars()
                .filter(|c| c.is_alphabetic())
                .collect::<HashSet<char>>()
                .len()
        })
        .sum()
}

fn _part2_slow(qd: &String) -> usize {
    qd.split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|l| l.chars().collect::<HashSet<char>>())
                .fold(None, |maybe_set1: Option<HashSet<char>>, set2| {
                    maybe_set1
                        .map(|set1| set1.intersection(&set2).map(|c| *c).collect())
                        .or(Some(set2))
                })
                .unwrap()
                .len()
        })
        .sum()
}

fn part1_fast(qd: &String) -> u32 {
    qd.split("\n\n")
        .map(|group| {
            group
                .bytes()
                .filter(|&c| c != b'\n')
                .fold(u32::MIN, |answers, answer| answers | 1_u32 << answer)
                .count_ones()
        })
        .sum()
}

fn part2_fast(qd: &String) -> u32 {
    qd.split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|l| {
                    l.bytes()
                        .fold(u32::MIN, |answers, answer| answers | 1_u32 << answer)
                })
                .fold(u32::MAX, |answers, answer| answers & answer)
                .count_ones()
        })
        .sum()
}

fn main() {
    let s1 = Instant::now();

    let question_data = read_to_string("input").unwrap();

    println!(
        "part1: {} ({}µs)",
        part1_fast(&question_data),
        s1.elapsed().as_micros()
    );

    let s2 = Instant::now();

    println!(
        "part2: {} ({}µs)",
        part2_fast(&question_data),
        s2.elapsed().as_micros()
    );

    println!("Time: {}µs", s1.elapsed().as_micros());
}
