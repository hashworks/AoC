use std::fs::File;
use std::io::prelude::*;
use std::str::from_utf8;
use std::time::Instant;

fn part1(buffer: &Vec<u8>) -> u32 {
    let mut lines = buffer.split(|b| b == &b'\n');
    let offset = from_utf8(lines.next().unwrap())
        .unwrap()
        .parse::<u32>()
        .unwrap();
    let (min, min_id) = lines
        .next()
        .unwrap()
        .split(|b| b == &b',')
        .filter(|&b| b[0] != b'x')
        .map(|bytes| from_utf8(bytes).unwrap().parse::<u32>().unwrap())
        .fold((std::u32::MAX, 0), |(min, min_id), id| {
            let new_min = (offset / id + 1) * id;
            if new_min < min {
                (new_min, id)
            } else {
                (min, min_id)
            }
        });

    (min - offset) * min_id
}

// TODO: Use chinese remainder theorem, otherwise this takes forever
fn part2(buffer: &Vec<u8>, first_id_min: u64, first_id_max: u64) -> u64 {
    let numbers: Vec<Option<u64>> = buffer
        .split(|b| b == &b'\n')
        .skip(1)
        .next()
        .unwrap()
        .split(|b| b == &b',')
        .map(|bytes| match bytes[0] {
            b'x' => None,
            _ => Some(from_utf8(bytes).unwrap().parse::<u64>().unwrap()),
        })
        .collect();

    let initial_first_id = numbers[0].unwrap();
    let mut next_min = first_id_min;
    let mut trillions = 1;

    loop {
        let first_id = (next_min / initial_first_id + 1) * initial_first_id;
        next_min = first_id;

        let (_, _, ongoing) = numbers
            .iter()
            .enumerate()
            .skip(1)
            .filter_map(|(i, o)| if let Some(n) = o { Some((i, n)) } else { None })
            .fold(
                (first_id, 0, true),
                |(previous_id, previous_id_index, ongoing), (index, id)| {
                    let id = (first_id / id + 1) * id;
                    next_min = next_min.min(id);
                    (
                        id,
                        index,
                        ongoing && id - (index - previous_id_index) as u64 == previous_id,
                    )
                },
            );

        if first_id > first_id_max {
            panic!("First id over max");
        }
        if ongoing {
            return first_id;
        }
        let new_trillions = first_id / 1000000000000;
        if new_trillions > trillions {
            trillions = new_trillions;
            println!("first_id > {}_000_000_000_000", trillions);
        }
    }
}

fn main() {
    let s1 = Instant::now();

    let mut file = File::open("input").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    println!("part1: {} ({}µs)", part1(&buffer), s1.elapsed().as_micros());

    let s2 = Instant::now();

    println!(
        "part2: {} ({}µs)",
        part2(&buffer, 100000000000000, u64::MAX),
        s2.elapsed().as_micros()
    );

    println!("Time: {}µs", s1.elapsed().as_micros());
}

#[test]
fn test_part2_example1() {
    assert_eq!(
        1068781,
        part2(
            &"
7,13,x,x,59,x,31,19"
                .as_bytes()
                .to_vec(),
            1000000,
            1068781
        )
    );
}

#[test]
fn test_part2_example2() {
    assert_eq!(
        3417,
        part2(
            &"
17,x,13,19"
                .as_bytes()
                .to_vec(),
            3000,
            3417
        )
    );
}

#[test]
fn test_part2_example3() {
    assert_eq!(
        754018,
        part2(
            &"
67,7,59,61"
                .as_bytes()
                .to_vec(),
            700000,
            754018
        )
    );
}

#[test]
fn test_part2_example4() {
    assert_eq!(
        779210,
        part2(
            &"
67,x,7,59,61"
                .as_bytes()
                .to_vec(),
            700000,
            779210
        )
    );
}

#[test]
fn test_part2_example5() {
    assert_eq!(
        1261476,
        part2(
            &"
67,7,x,59,61"
                .as_bytes()
                .to_vec(),
            1200000,
            1261476
        )
    );
}

#[test]
fn test_part2_example6() {
    assert_eq!(
        1202161486,
        part2(
            &"
1789,37,47,1889"
                .as_bytes()
                .to_vec(),
            1200000000,
            1202161486
        )
    );
}
