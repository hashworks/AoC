use std::collections::{HashMap, VecDeque};
use std::fs::read_to_string;
use std::time::Instant;

fn part1(numbers: &Vec<&str>, preamble: usize) -> Option<u64> {
    let mut preamble_vec: VecDeque<Vec<u64>> = vec![Vec::with_capacity(preamble - 1); preamble]
        .into_iter()
        .collect();
    let mut number_vec: Vec<u64> = Vec::with_capacity(numbers.len());

    for (index, number) in numbers
        .iter()
        .map(|l| l.parse::<u64>().unwrap())
        .enumerate()
    {
        if index >= preamble {
            if !preamble_vec.iter().any(|s| s.contains(&number)) {
                return Some(number);
            }
            preamble_vec.rotate_left(1);
            preamble_vec[preamble - 1].clear();
        }
        if let Some(previous_numbers) = number_vec.rchunks(preamble - 1).next() {
            for (p_index, previous_number) in previous_numbers.iter().enumerate() {
                preamble_vec[p_index].push(previous_number + number);
            }
        }
        number_vec.push(number);
    }

    None
}

fn part2(numbers: &Vec<&str>, invalid_number: u64) -> Option<u64> {
    let mut sums: HashMap<(usize, usize), (u64, u64, u64)> = HashMap::new();
    for (index, number) in numbers
        .iter()
        .rev()
        .map(|l| l.parse::<u64>().unwrap())
        .enumerate()
        .filter(|(_, n)| n < &invalid_number)
    {
        sums.insert((index, index), (number, number, number));
        if index == 0 {
            continue;
        }
        for previous_index in 0..index {
            if let Some((p_sum, p_min, p_max)) = sums.get(&(previous_index, index - 1)) {
                let sum = p_sum + number;
                if sum > invalid_number {
                    continue;
                }
                let min = *p_min.min(&number);
                let max = *p_max.max(&number);
                if sum == invalid_number {
                    return Some(min + max);
                }
                sums.insert((previous_index, index), (sum, min, max));
            }
        }
    }
    None
}

fn main() {
    let s1 = Instant::now();

    let str = read_to_string("input").unwrap();
    let number_lines = str.lines().collect();

    let invalid_number = part1(&number_lines, 25).unwrap();

    println!("part1: {} ({}µs)", invalid_number, s1.elapsed().as_micros());

    let s2 = Instant::now();

    println!(
        "part2: {} ({}µs)",
        part2(&number_lines, invalid_number).unwrap(),
        s2.elapsed().as_micros()
    );

    println!("Time: {}µs", s1.elapsed().as_micros());
}

#[test]
fn test() {
    let input = &"35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576"
    .lines()
    .collect();
    let invalid_number = part1(&input, 5).unwrap();
    assert_eq!(127, invalid_number);
    assert_eq!(62, part2(&input, invalid_number).unwrap());
}
