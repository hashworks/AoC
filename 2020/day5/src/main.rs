use std::io::{BufRead, BufReader};
use std::fs::File;
use std::time::Instant;

fn seat_to_id(seat: String) -> usize {
    seat.chars().fold(0, |a, c| (a << 1) + match c {
        'B' | 'R' => 1,
        _ => 0
    })
}

fn part1(f: File) -> usize {
    BufReader::new(f)
        .lines()
        .map(|r| r.unwrap())
        .map(seat_to_id).max().unwrap()
}

// We assert that max() is <= 1000, increase if necessary
fn part2(f: File) -> usize {
    let mut seats: [bool; 1000] = [false; 1000];

    BufReader::new(f)
        .lines()
        .map(|r| r.unwrap())
        .map(seat_to_id).for_each(|s| seats[s] = true);

    seats.iter()
         .enumerate()
         .filter(|(_, &s)| !s)
         .filter(|(i, _)| i != &0usize && seats[i-1] && seats[i+1])
         .take(1)
         .next()
         .unwrap().0
}

fn main() {
    let s1 = Instant::now();

    let max = part1(File::open("input").unwrap());

    println!("part1: {} ({}µs)", max, s1.elapsed().as_micros());

    let s2 = Instant::now();

    println!("part2: {} ({}µs)", part2(File::open("input").unwrap()), s2.elapsed().as_micros());

    println!("Time: {}µs", s1.elapsed().as_micros());
}

#[test]
fn seat_to_id_works() {
    assert_eq!(seat_to_id("FBFBBFFRLR".to_owned()), 357);
    assert_eq!(seat_to_id("BFFFBBFRRR".to_owned()), 567);
    assert_eq!(seat_to_id("FFFBBBFRRR".to_owned()), 119);
    assert_eq!(seat_to_id("BBFFBBFRLL".to_owned()), 820);
}