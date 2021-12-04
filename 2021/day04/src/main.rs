use std::fs::File;
use std::io::{self, BufRead};
use std::time::Instant;
use thiserror::Error;

#[derive(Error, Debug)]
enum AoCError {
    #[error("io error")]
    IoErr(#[from] io::Error),
    #[error("failed to parse int")]
    ParseIntErr(#[from] std::num::ParseIntError),
    #[error("no numbers found")]
    NoNumbersErr,
    #[error("no winner found")]
    NoWinnerErr,
}

#[derive(Debug, Clone, Copy)]
struct Card {
    winning_number: Option<usize>,
    rows: [[(usize, bool); 5]; 5],
}

impl Card {
    fn new() -> Self {
        Card {
            winning_number: None,
            rows: [[(0, false); 5]; 5],
        }
    }
    fn mark_and_check(&mut self, number: usize) -> bool {
        if self.winning_number.is_some() {
            return false;
        }
        let marked_cell = self.rows.iter().enumerate().find_map(|(r_nr, row)| {
            row.iter().enumerate().find_map(|(c_nr, &(n, _))| {
                if n == number {
                    Some((r_nr, c_nr))
                } else {
                    None
                }
            })
        });
        if let Some((r_nr, c_nr)) = marked_cell {
            self.rows[r_nr][c_nr].1 = true;
            if self.rows[r_nr].iter().all(|(_, m)| *m) || self.rows.iter().all(|row| row[c_nr].1) {
                self.winning_number = Some(number);
                return true;
            }
        }
        false
    }
    fn score(&self) -> usize {
        self.rows
            .iter()
            .map(|row| {
                row.iter()
                    .filter(|(_, m)| !m)
                    .map(|&(n, _)| n)
                    .sum::<usize>()
            })
            .sum::<usize>()
            * self.winning_number.unwrap_or(0)
    }
}

fn parse(file: File) -> Result<(Vec<usize>, Vec<Card>), AoCError> {
    let mut lines = io::BufReader::new(file).lines();

    let numbers = lines
        .next()
        .ok_or(AoCError::NoNumbersErr)??
        .split(',')
        .map(|s| Ok(s.parse::<usize>()?))
        .collect::<Result<Vec<usize>, AoCError>>()?;

    let mut cards = Vec::new();
    let mut card = Card::new();
    let mut r_nr = 0;
    while let Some(line) = lines.next() {
        let line = line?;
        if line.is_empty() {
            continue;
        }
        let mut n_split = line.split_whitespace();
        for c_nr in 0..5 {
            card.rows[r_nr][c_nr].0 = n_split
                .next()
                .ok_or(AoCError::NoNumbersErr)?
                .parse::<usize>()?
        }
        if r_nr == 4 {
            cards.push(card);
            card = Card::new();
            r_nr = 0;
        } else {
            r_nr += 1;
        }
    }

    Ok((numbers, cards))
}

fn part1(numbers: &Vec<usize>, mut cards: Vec<Card>) -> Result<usize, AoCError> {
    for number in numbers {
        for i in 0..cards.len() {
            if cards[i].mark_and_check(*number) {
                return Ok(cards[i].score());
            }
        }
    }
    Err(AoCError::NoWinnerErr)
}

fn part2(numbers: &Vec<usize>, mut cards: Vec<Card>) -> Result<usize, AoCError> {
    let mut last_winner = 0;
    for number in numbers {
        for i in 0..cards.len() {
            if cards[i].mark_and_check(*number) {
                last_winner = i;
            }
        }
    }
    Ok(cards[last_winner].score())
}

fn main() -> Result<(), AoCError> {
    let start = Instant::now();

    let (numbers, cards) = parse(File::open("input")?)?;

    println!("parsing: {}µs", start.elapsed().as_micros());

    let s_part1 = Instant::now();
    println!(
        "part1: {} ({}µs)",
        part1(&numbers, cards.to_vec())?,
        s_part1.elapsed().as_micros()
    );

    let s_part2 = Instant::now();
    println!(
        "part2: {} ({}µs)",
        part2(&numbers, cards.to_vec())?,
        s_part2.elapsed().as_micros()
    );

    println!("Time: {}µs", start.elapsed().as_micros());

    Ok(())
}

#[test]
fn test_parse_example() {
    let (numbers, cards) = parse(File::open("example").unwrap()).unwrap();
    assert_eq!(27, numbers.len());
    assert_eq!(3, cards.len());
    assert_eq!(22, cards[0].rows[0][0].0);
    assert_eq!(19, cards[0].rows[4][4].0);
    assert_eq!(3, cards[1].rows[0][0].0);
    assert_eq!(6, cards[1].rows[4][4].0);
    assert_eq!(14, cards[2].rows[0][0].0);
    assert_eq!(7, cards[2].rows[4][4].0);
    assert_eq!(4512, part1(&numbers, cards.to_vec()).unwrap());
    assert_eq!(1924, part2(&numbers, cards).unwrap());
}
