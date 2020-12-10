use std::fs::read_to_string;
use std::time::Instant;

fn part1(sorted_numbers: &Vec<u32>) -> u32 {
    let (ones, threes) = sorted_numbers
        .windows(2)
        .fold((1, 1), |(ones, threes), pair| match pair[1] - pair[0] {
            1 => (ones + 1, threes),
            3 => (ones, threes + 1),
            _ => (ones, threes),
        });
    ones * threes
}

fn part2(sorted_numbers: &mut Vec<u32>) -> u64 {
    let mut sorted_numbers0 = vec![0];
    sorted_numbers0.append(sorted_numbers);
    sorted_numbers0
        .windows(2)
        .collect::<Vec<_>>()
        .split(|pair| pair[1] - pair[0] == 3)
        .map(|pair| match pair.len() {
            4 => 7,
            3 => 4,
            2 => 2,
            _ => 1,
        })
        .product::<u64>()
}

fn main() {
    let s1 = Instant::now();

    let mut numbers: Vec<u32> = read_to_string("input")
        .unwrap()
        .lines()
        .map(|l| l.parse::<u32>().unwrap())
        .collect();
    numbers.sort();

    println!("part1: {} ({}ns)", part1(&numbers), s1.elapsed().as_nanos());

    let s2 = Instant::now();

    println!(
        "part2: {} ({}ns)",
        part2(&mut numbers),
        s2.elapsed().as_nanos()
    );

    println!("Time: {}ns", s1.elapsed().as_nanos());
}

#[test]
fn test() {
    let mut input = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
    input.sort();
    assert_eq!(35, part1(&input));
    assert_eq!(8, part2(&mut input));
}

#[test]
fn test_part2_long() {
    let mut input = vec![
        28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8,
        17, 7, 9, 4, 2, 34, 10, 3,
    ];
    input.sort();
    assert_eq!(220, part1(&input));
    assert_eq!(19208, part2(&mut input));
}
