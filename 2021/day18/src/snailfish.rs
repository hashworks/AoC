use std::{fmt::Display, ops::Add, str::FromStr};

use crate::AoCError;

#[derive(Clone)]
pub(crate) enum SFR {
    SF(Box<(SFR, SFR)>),
    R(usize),
}

impl Display for SFR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SFR::SF(sf) => write!(f, "[{},{}]", sf.0, sf.1),
            SFR::R(r) => write!(f, "{}", r),
        }
    }
}

impl FromStr for SFR {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self, AoCError> {
        if let Some(s) = s.parse::<usize>().ok() {
            Ok(SFR::R(s))
        } else {
            let mut comma_position = 0;
            let mut level = 0;
            for (i, c) in s.chars().enumerate() {
                match c {
                    '[' => level += 1,
                    ']' => level -= 1,
                    ',' if level == 1 => comma_position = i,
                    _ => {}
                }
            }
            Ok(SFR::SF(Box::new((
                s[1..comma_position].parse()?,
                s[comma_position + 1..s.len() - 1].parse()?,
            ))))
        }
    }
}

impl Add for SFR {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let sfr = SFR::SF(Box::new((self, other)));
        sfr.reduce()
    }
}

impl SFR {
    pub fn magnitude(&self) -> usize {
        match self {
            SFR::R(r) => *r,
            SFR::SF(sf) => 3 * sf.0.magnitude() + 2 * sf.1.magnitude(),
        }
    }

    pub fn reduce(self) -> SFR {
        let mut sfr = self;
        loop {
            let (new_sfr, changed) = sfr.reduce_once(0);
            sfr = new_sfr;
            if !changed {
                break;
            }
        }
        sfr
    }

    fn reduce_once(self, level: usize) -> (SFR, bool) {
        let (sfr, _, _, changed) = self.reduce_explode(level);
        if changed {
            (sfr, true)
        } else {
            sfr.reduce_split()
        }
    }

    fn reduce_explode(self, level: usize) -> (SFR, Option<usize>, Option<usize>, bool) {
        match self {
            SFR::R(_) => (self, None, None, false),
            SFR::SF(sf) => {
                match (sf.0, sf.1) {
                    (SFR::R(left_regular), SFR::R(right_regular)) if level == 4 => {
                        (SFR::R(0), Some(left_regular), Some(right_regular), true)
                    }
                    (left, right) => {
                        // No explode, reduce_explode and apply carryover on explosion
                        let (left, left_carryover, right_carryover, exploded) =
                            left.reduce_explode(level + 1);
                        if exploded {
                            let right = if let Some(right_carryover) = right_carryover {
                                right.add_right_carryover(Some(right_carryover))
                            } else {
                                right
                            };

                            return (SFR::SF(Box::new((left, right))), left_carryover, None, true);
                        }

                        let (right, left_carryover, right_carryover, exploded) =
                            right.reduce_explode(level + 1);
                        if exploded {
                            let left = if let Some(left_carryover) = left_carryover {
                                left.add_left_carryover(Some(left_carryover))
                            } else {
                                left
                            };

                            return (
                                SFR::SF(Box::new((left, right))),
                                None,
                                right_carryover,
                                true,
                            );
                        }

                        (SFR::SF(Box::new((left, right))), None, None, false)
                    }
                }
            }
        }
    }

    fn add_left_carryover(self, carryover: Option<usize>) -> SFR {
        if let Some(carryover) = carryover {
            match self {
                SFR::SF(sf) => SFR::SF(Box::new((sf.0, sf.1.add_left_carryover(Some(carryover))))),
                SFR::R(regular) => SFR::R(carryover + regular),
            }
        } else {
            self
        }
    }

    fn add_right_carryover(self, carryover: Option<usize>) -> SFR {
        if let Some(carryover) = carryover {
            match self {
                SFR::SF(sf) => SFR::SF(Box::new((sf.0.add_right_carryover(Some(carryover)), sf.1))),
                SFR::R(regular) => SFR::R(carryover + regular),
            }
        } else {
            self
        }
    }

    fn reduce_split(self) -> (SFR, bool) {
        match self {
            SFR::R(r) if r > 9 => (
                {
                    let divided_rounded_down = r / 2;
                    SFR::SF(Box::new((
                        SFR::R(divided_rounded_down),
                        SFR::R(r - divided_rounded_down),
                    )))
                },
                true,
            ),
            SFR::SF(sf) => {
                let (left, changed) = sf.0.reduce_split();
                if changed {
                    (SFR::SF(Box::new((left, sf.1))), true)
                } else {
                    let (right, changed) = sf.1.reduce_split();
                    (SFR::SF(Box::new((left, right))), changed)
                }
            }
            _ => (self, false),
        }
    }
}

#[test]
fn test_reduce() {
    let sfr = "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]"
        .parse::<SFR>()
        .unwrap()
        .reduce();
    assert_eq!(sfr.to_string(), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
}

#[test]
fn test_larger_reduce() {
    let sfr = "[[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]"
        .parse::<SFR>()
        .unwrap()
        .reduce();
    assert_eq!(
        sfr.to_string(),
        "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]"
    );
}

#[test]
fn test_add() {
    assert_eq!(
        ("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]"
            .parse::<SFR>()
            .unwrap()
            + "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]".parse::<SFR>().unwrap())
        .to_string(),
        "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]"
            .parse::<SFR>()
            .unwrap()
            .to_string(),
    );
}

#[test]
fn test_magnitude() {
    assert_eq!(
        "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
            .parse::<SFR>()
            .unwrap()
            .magnitude(),
        3488
    );
}
