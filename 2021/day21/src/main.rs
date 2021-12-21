use std::cmp::max;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::time::Instant;
use thiserror::Error;

#[derive(Error, Debug)]
enum AoCError {
    #[error("io error")]
    IoErr(#[from] io::Error),
    #[error("bad input parse error")]
    BadInputParseErr,
}

fn parse(file: File) -> Result<(u64, u64), AoCError> {
    let mut lines = io::BufReader::new(file).lines();
    let p1 = lines
        .next()
        .ok_or(AoCError::BadInputParseErr)??
        .chars()
        .skip(28)
        .next()
        .ok_or(AoCError::BadInputParseErr)?
        .to_digit(10)
        .ok_or(AoCError::BadInputParseErr)?;
    let p2 = lines
        .next()
        .ok_or(AoCError::BadInputParseErr)??
        .chars()
        .skip(28)
        .next()
        .ok_or(AoCError::BadInputParseErr)?
        .to_digit(10)
        .ok_or(AoCError::BadInputParseErr)?;
    Ok((p1 as u64, p2 as u64))
}

fn dirac_mod(n: u64) -> u64 {
    ((n - 1) % 10) + 1
}

fn part1(mut p1: u64, mut p2: u64) -> u64 {
    let mut p1_score = 0;
    let mut p2_score = 0;
    let mut dice_state = 0;
    let mut dice_rolls = 0;

    let mut dirac_dice = move || {
        (0..3).fold(0, |r, _| {
            dice_state = if dice_state == 100 { 1 } else { dice_state + 1 };
            r + dice_state
        })
    };

    loop {
        p1 = dirac_mod(p1 + dirac_dice());
        p1_score += p1;
        dice_rolls += 3;

        if p1_score >= 1000 {
            return p2_score * dice_rolls;
        }

        p2 = dirac_mod(p2 + dirac_dice());
        p2_score += p2;
        dice_rolls += 3;

        if p2_score >= 1000 {
            return p1_score * dice_rolls;
        }
    }
}

fn sum_multiplicator(n: &u64) -> u64 {
    match n {
        3 => 1,
        4 => 3,
        5 => 6,
        6 => 7,
        7 => 6,
        8 => 3,
        9 => 1,
        _ => 0,
    }
}

fn dimensional(
    p1: u64,
    p2: u64,
    p1_score: u64,
    p2_score: u64,
    hashmap: &mut HashMap<(u64, u64, u64, u64), (u64, u64)>,
) -> (u64, u64) {
    if let Some(c) = hashmap.get(&(p1, p2, p1_score, p2_score)) {
        *c
    } else {
        let mut p1_wins = 0;
        let mut p2_wins = 0;

        for p1_dice_sum in 3..=9 {
            let p1_win_multiplicator = sum_multiplicator(&p1_dice_sum);
            let p1 = dirac_mod(p1 + p1_dice_sum);
            let p1_score = p1_score + p1;

            if p1_score >= 21 {
                p1_wins += p1_win_multiplicator;
                continue;
            }

            for p2_dice_sum in 3..=9 {
                let p2_win_multiplicator = sum_multiplicator(&p2_dice_sum);
                let p2 = dirac_mod(p2 + p2_dice_sum);
                let p2_score = p2_score + p2;

                if p2_score >= 21 {
                    p2_wins += p2_win_multiplicator * p1_win_multiplicator;
                    continue;
                }

                let (deeper_p1_wins, deeper_p2_wins) =
                    dimensional(p1, p2, p1_score, p2_score, hashmap);

                p1_wins += deeper_p1_wins * p1_win_multiplicator * p2_win_multiplicator;
                p2_wins += deeper_p2_wins * p2_win_multiplicator * p1_win_multiplicator;
            }
        }

        hashmap.insert((p1, p2, p1_score, p2_score), (p1_wins, p2_wins));

        (p1_wins, p2_wins)
    }
}

fn part2(p1: u64, p2: u64) -> u64 {
    let (p1_wins, p2_wins) = dimensional(p1, p2, 0, 0, &mut HashMap::new());
    max(p1_wins, p2_wins)
}

fn main() -> Result<(), AoCError> {
    let start = Instant::now();

    let (p1, p2) = parse(File::open("input")?)?;

    println!("parsing: {}µs", start.elapsed().as_micros());

    let s_part1 = Instant::now();
    println!(
        "part1: {} ({}ns)",
        part1(p1, p2),
        s_part1.elapsed().as_nanos()
    );

    let s_part2 = Instant::now();
    println!(
        "part2: {} ({}µs)",
        part2(p1, p2),
        s_part2.elapsed().as_micros()
    );

    println!("Time: {}µs", start.elapsed().as_micros());

    Ok(())
}

#[test]
fn test_part1() {
    assert_eq!(part1(4, 8), 739785);
}

#[test]
fn test_part2() {
    assert_eq!(part2(4, 8), 444356092776315);
}
