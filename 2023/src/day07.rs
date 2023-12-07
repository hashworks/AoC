mod util;

use std::{cmp::Ordering, error::Error, io::BufRead};
use util::{aoc::AoCDay, input::get_reader};

const ID: &str = "day07";
type Input = Vec<Hand>;
type Output = usize;

#[derive(Debug, Clone, Copy)]
struct Hand {
    cards: [usize; 5],
    counts: [usize; 14],
    level: usize,
    bit: usize,
    joker_mode: bool,
}

impl Hand {
    fn new(chars: Vec<char>) -> Result<Self, Box<dyn Error>> {
        if chars.len() < 7 {
            return Err("Parse Error".into());
        }
        let bit = chars[6..].iter().collect::<String>().parse::<usize>()?;

        let mut hand = Hand {
            cards: [0; 5],
            counts: [0; 14],
            level: 0,
            bit,
            joker_mode: false,
        };

        hand.parse_cards(chars)?;

        Ok(hand)
    }

    fn prepare_mode(&mut self) {
        if self.joker_mode {
            self.cards.iter_mut().for_each(|card| {
                if *card == 10 {
                    // Joker, lowest individual card
                    *card = 0;
                }
                self.counts[0] += 1;
            });
            self.counts[10] = 0;
        }

        self.set_counts();
        self.set_level();
    }

    fn parse_cards(&mut self, chars: Vec<char>) -> Result<(), Box<dyn Error>> {
        let match_card = |card: char| -> Option<usize> {
            match card {
                '2' => Some(1),
                '3' => Some(2),
                '4' => Some(3),
                '5' => Some(4),
                '6' => Some(5),
                '7' => Some(6),
                '8' => Some(7),
                '9' => Some(8),
                'T' => Some(9),
                'J' => Some(10),
                'Q' => Some(11),
                'K' => Some(12),
                'A' => Some(13),
                _ => None,
            }
        };

        self.cards = [
            match_card(chars[0]).ok_or("Parse Error")?,
            match_card(chars[1]).ok_or("Parse Error")?,
            match_card(chars[2]).ok_or("Parse Error")?,
            match_card(chars[3]).ok_or("Parse Error")?,
            match_card(chars[4]).ok_or("Parse Error")?,
        ];

        Ok(())
    }

    fn set_counts(&mut self) {
        self.counts = [0; 14];
        for card in self.cards.iter() {
            self.counts[*card] += 1;
        }
    }

    fn has_n_of_a_kind(&self, n: usize, filter: Option<usize>, use_joker: bool) -> Option<usize> {
        let joker_count = if use_joker { self.counts[0] } else { 0 };
        self.counts
            .iter()
            .enumerate()
            .skip(1)
            .filter(|(card, _)| filter.is_none() || *card != filter.unwrap())
            .find_map(|(card, count)| {
                if count + joker_count == n {
                    Some(card)
                } else {
                    None
                }
            })
    }

    fn has_full_house(&self) -> bool {
        let three_of_a_kind_joker = self.has_n_of_a_kind(3, None, true);
        let three_of_a_kind_no_joker = self.has_n_of_a_kind(3, None, false);
        three_of_a_kind_joker.is_some()
            && self
                .has_n_of_a_kind(2, three_of_a_kind_joker, false)
                .is_some()
            || three_of_a_kind_no_joker.is_some()
                && self
                    .has_n_of_a_kind(2, three_of_a_kind_no_joker, true)
                    .is_some()
    }

    fn has_n_pair(&self, n: usize, use_joker: bool) -> bool {
        let mut joker_count = if use_joker { self.counts[0] } else { 0 };

        self.counts
            .iter()
            .skip(1)
            .map(|count| {
                if joker_count > 0 && *count == 1 {
                    joker_count -= 1;
                    count + 1
                } else {
                    *count
                }
            })
            .filter(|count| *count == 2)
            .count()
            == n
    }

    fn set_level(&mut self) {
        self.level = if self.has_n_of_a_kind(5, None, true).is_some() {
            6
        } else if self.has_n_of_a_kind(4, None, true).is_some() {
            5
        } else if self.has_full_house() {
            4
        } else if self.has_n_of_a_kind(3, None, true).is_some() {
            3
        } else if self.has_n_pair(2, true) {
            2
        } else if self.has_n_pair(1, true) {
            1
        } else {
            0
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.level == other.level {
            self.cards.cmp(&other.cards)
        } else {
            self.level.cmp(&other.level)
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Eq for Hand {}

struct Day {}

impl AoCDay<Input, Output> for Day {
    fn parse_input(&self, id: &str) -> Result<Input, Box<dyn Error>> {
        let lines = get_reader(id)?.lines();

        let mut hands = vec![];

        for line in lines {
            let chars = line?.chars().collect::<Vec<char>>();
            hands.push(Hand::new(chars)?);
        }

        Ok(hands)
    }

    fn part1(&self, hands: &Input) -> Result<Output, Box<dyn Error>> {
        let mut hands = hands.clone();

        hands.iter_mut().for_each(Hand::prepare_mode);

        hands.sort();

        Ok(hands
            .iter()
            .enumerate()
            .map(|(i, hand)| (i + 1) * hand.bit)
            .sum::<usize>())
    }

    fn part2(&self, hands: &Input) -> Result<Output, Box<dyn Error>> {
        let mut hands = hands.clone();

        hands.iter_mut().for_each(|hand| {
            hand.joker_mode = true;
            hand.prepare_mode()
        });

        hands.sort();

        Ok(hands
            .iter()
            .enumerate()
            .map(|(i, hand)| (i + 1) * hand.bit)
            .sum::<usize>())
    }
}

fn main() {
    Day {}.run(ID);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        let day = Day {};
        assert_eq!(
            day.parse_and_solve_part1(format!("{}_test1", ID).as_str())
                .unwrap(),
            6440
        );
    }

    #[test]
    fn test_solve_part2() {
        let day = Day {};
        assert_eq!(
            day.parse_and_solve_part2(format!("{}_test1", ID).as_str())
                .unwrap(),
            5905
        );
    }

    #[test]
    fn has_five_of_a_kind() {
        let mut hand = Hand::new("AAAAA 0".chars().collect()).unwrap();
        hand.prepare_mode();
        assert!(hand.level == 6);
    }

    #[test]
    fn has_five_of_a_kind_one_joker() {
        let mut hand = Hand::new("AAAAJ 0".chars().collect()).unwrap();
        hand.joker_mode = true;
        hand.prepare_mode();
        assert!(hand.level == 6);
    }

    #[test]
    fn has_five_of_a_kind_two_joker() {
        let mut hand = Hand::new("AAAJJ 0".chars().collect()).unwrap();
        hand.joker_mode = true;
        hand.prepare_mode();
        assert!(hand.level == 6);
    }

    #[test]
    fn has_five_of_a_kind_all_joker() {
        let mut hand = Hand::new("JJJJJ 0".chars().collect()).unwrap();
        hand.joker_mode = true;
        hand.prepare_mode();
        assert!(hand.level == 6);
    }

    #[test]
    fn has_four_of_a_kind() {
        let mut hand = Hand::new("AAAAK 0".chars().collect()).unwrap();
        hand.prepare_mode();
        assert!(hand.level == 5);
    }

    #[test]
    fn has_four_of_a_kind_one_joker() {
        let mut hand = Hand::new("AAAJK 0".chars().collect()).unwrap();
        hand.joker_mode = true;
        hand.prepare_mode();
        assert!(hand.level == 5);
    }

    #[test]
    fn has_four_of_a_kind_two_joker() {
        let mut hand = Hand::new("AAJJK 0".chars().collect()).unwrap();
        hand.joker_mode = true;
        hand.prepare_mode();
        assert!(hand.level == 5);
    }

    #[test]
    fn has_full_house() {
        let mut hand = Hand::new("AAAKK 0".chars().collect()).unwrap();
        hand.prepare_mode();
        assert!(hand.level == 4);
    }

    #[test]
    fn has_three_of_a_kind() {
        let mut hand = Hand::new("AAAKQ 0".chars().collect()).unwrap();
        hand.prepare_mode();
        assert!(hand.level == 3);
    }

    #[test]
    fn has_three_of_a_kind_one_joker() {
        let mut hand = Hand::new("AAJKQ 0".chars().collect()).unwrap();
        hand.joker_mode = true;
        hand.prepare_mode();
        assert!(hand.level == 3);
    }

    #[test]
    fn has_two_pair() {
        let mut hand = Hand::new("AAQKK 0".chars().collect()).unwrap();
        hand.prepare_mode();
        assert!(hand.level == 2);
    }

    #[test]
    fn has_one_pair() {
        let mut hand = Hand::new("AAQK2 0".chars().collect()).unwrap();
        hand.prepare_mode();
        assert!(hand.level == 1);
    }

    #[test]
    fn has_one_pair_one_joker() {
        let mut hand = Hand::new("AJQK2 0".chars().collect()).unwrap();
        hand.joker_mode = true;
        hand.prepare_mode();
        assert!(hand.level == 1);
    }

    #[test]
    fn has_high_card() {
        let mut hand = Hand::new("23456 0".chars().collect()).unwrap();
        hand.prepare_mode();
        assert!(hand.level == 0);
    }
}
