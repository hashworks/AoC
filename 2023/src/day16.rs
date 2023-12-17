mod util;

use rayon::iter::{ParallelBridge, ParallelIterator};
use std::{collections::HashSet, error::Error, io::BufRead};
use util::{aoc::AoCDay, input::get_reader};

const ID: &str = "day16";
type Input = Vec<Vec<Tile>>;
type Output = usize;

#[derive(Debug)]
enum Tile {
    Empty,
    RightAngleMirror,
    LeftAngleMirror,
    VerticalSplitter,
    HorizontalSplitter,
}

#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]
struct Beam {
    direction: BeamDirection,
    position: (isize, isize),
}

#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]
enum BeamDirection {
    Up,
    Down,
    Left,
    Right,
}

struct Day {}

impl AoCDay<Input, Output> for Day {
    fn parse_input(&self, id: &str) -> Result<Input, Box<dyn Error>> {
        get_reader(id)?
            .lines()
            .map(|l| {
                let line = l.unwrap();
                let row = line
                    .chars()
                    .map(|c| match c {
                        '.' => Ok(Tile::Empty),
                        '/' => Ok(Tile::RightAngleMirror),
                        '\\' => Ok(Tile::LeftAngleMirror),
                        '|' => Ok(Tile::VerticalSplitter),
                        '-' => Ok(Tile::HorizontalSplitter),
                        _ => Err("invalid character"),
                    })
                    .collect::<Result<_, _>>()?;
                Ok(row)
            })
            .collect()
    }

    fn part1(&self, map: &Input) -> Result<Output, Box<dyn Error>> {
        let mut movements = HashSet::new();
        let beam = Beam {
            direction: BeamDirection::Right,
            position: (0, 0),
        };
        traverse(beam, map, &mut movements);
        Ok(movements
            .iter()
            .map(|b| b.position)
            .collect::<HashSet<_>>()
            .len())
    }

    fn part2(&self, map: &Input) -> Result<Output, Box<dyn Error>> {
        let max_y = map.len();
        let max_x = map.get(0).map(|r| r.len()).unwrap_or(0);
        Ok((0..max_y)
            .zip(0..max_x)
            .par_bridge()
            .flat_map(|(y, x)| {
                [
                    Beam {
                        direction: BeamDirection::Down,
                        position: (0, x as isize),
                    },
                    Beam {
                        direction: BeamDirection::Up,
                        position: (max_y as isize, x as isize),
                    },
                    Beam {
                        direction: BeamDirection::Right,
                        position: (y as isize, 0),
                    },
                    Beam {
                        direction: BeamDirection::Left,
                        position: (y as isize, max_x as isize),
                    },
                ]
            })
            .map(|b| {
                let mut movements = HashSet::new();
                traverse(b, map, &mut movements);
                movements
                    .iter()
                    .map(|b| b.position)
                    .collect::<HashSet<_>>()
                    .len()
            })
            .max()
            .unwrap_or(0))
    }
}

fn traverse(beam: Beam, map: &Input, movements: &mut HashSet<Beam>) {
    if beam.position.0 < 0
        || beam.position.0 >= map.len() as isize
        || beam.position.1 < 0
        || beam.position.1 >= map.get(0).map(|r| r.len()).unwrap_or(0) as isize
    {
        return;
    }

    if !movements.insert(beam) {
        return;
    }

    let tile = &map[beam.position.0 as usize][beam.position.1 as usize];

    match (beam.direction, tile) {
        (BeamDirection::Up, Tile::Empty) | (BeamDirection::Up, Tile::VerticalSplitter) => traverse(
            Beam {
                direction: BeamDirection::Up,
                position: (beam.position.0 - 1, beam.position.1),
            },
            map,
            movements,
        ),
        (BeamDirection::Up, Tile::RightAngleMirror) => traverse(
            Beam {
                direction: BeamDirection::Right,
                position: (beam.position.0, beam.position.1 + 1),
            },
            map,
            movements,
        ),
        (BeamDirection::Up, Tile::LeftAngleMirror) => traverse(
            Beam {
                direction: BeamDirection::Left,
                position: (beam.position.0, beam.position.1 - 1),
            },
            map,
            movements,
        ),
        (BeamDirection::Up, Tile::HorizontalSplitter) => {
            traverse(
                Beam {
                    direction: BeamDirection::Left,
                    position: (beam.position.0, beam.position.1 - 1),
                },
                map,
                movements,
            );
            traverse(
                Beam {
                    direction: BeamDirection::Right,
                    position: (beam.position.0, beam.position.1 + 1),
                },
                map,
                movements,
            );
        }
        (BeamDirection::Down, Tile::Empty) | (BeamDirection::Down, Tile::VerticalSplitter) => {
            traverse(
                Beam {
                    direction: BeamDirection::Down,
                    position: (beam.position.0 + 1, beam.position.1),
                },
                map,
                movements,
            )
        }
        (BeamDirection::Down, Tile::RightAngleMirror) => traverse(
            Beam {
                direction: BeamDirection::Left,
                position: (beam.position.0, beam.position.1 - 1),
            },
            map,
            movements,
        ),
        (BeamDirection::Down, Tile::LeftAngleMirror) => traverse(
            Beam {
                direction: BeamDirection::Right,
                position: (beam.position.0, beam.position.1 + 1),
            },
            map,
            movements,
        ),
        (BeamDirection::Down, Tile::HorizontalSplitter) => {
            traverse(
                Beam {
                    direction: BeamDirection::Left,
                    position: (beam.position.0, beam.position.1 - 1),
                },
                map,
                movements,
            );
            traverse(
                Beam {
                    direction: BeamDirection::Right,
                    position: (beam.position.0, beam.position.1 + 1),
                },
                map,
                movements,
            );
        }
        (BeamDirection::Left, Tile::Empty) | (BeamDirection::Left, Tile::HorizontalSplitter) => {
            traverse(
                Beam {
                    direction: BeamDirection::Left,
                    position: (beam.position.0, beam.position.1 - 1),
                },
                map,
                movements,
            )
        }
        (BeamDirection::Left, Tile::RightAngleMirror) => traverse(
            Beam {
                direction: BeamDirection::Down,
                position: (beam.position.0 + 1, beam.position.1),
            },
            map,
            movements,
        ),
        (BeamDirection::Left, Tile::LeftAngleMirror) => traverse(
            Beam {
                direction: BeamDirection::Up,
                position: (beam.position.0 - 1, beam.position.1),
            },
            map,
            movements,
        ),
        (BeamDirection::Left, Tile::VerticalSplitter) => {
            traverse(
                Beam {
                    direction: BeamDirection::Up,
                    position: (beam.position.0 - 1, beam.position.1),
                },
                map,
                movements,
            );
            traverse(
                Beam {
                    direction: BeamDirection::Down,
                    position: (beam.position.0 + 1, beam.position.1),
                },
                map,
                movements,
            );
        }
        (BeamDirection::Right, Tile::Empty) | (BeamDirection::Right, Tile::HorizontalSplitter) => {
            traverse(
                Beam {
                    direction: BeamDirection::Right,
                    position: (beam.position.0, beam.position.1 + 1),
                },
                map,
                movements,
            )
        }
        (BeamDirection::Right, Tile::RightAngleMirror) => traverse(
            Beam {
                direction: BeamDirection::Up,
                position: (beam.position.0 - 1, beam.position.1),
            },
            map,
            movements,
        ),
        (BeamDirection::Right, Tile::LeftAngleMirror) => traverse(
            Beam {
                direction: BeamDirection::Down,
                position: (beam.position.0 + 1, beam.position.1),
            },
            map,
            movements,
        ),
        (BeamDirection::Right, Tile::VerticalSplitter) => {
            traverse(
                Beam {
                    direction: BeamDirection::Up,
                    position: (beam.position.0 - 1, beam.position.1),
                },
                map,
                movements,
            );
            traverse(
                Beam {
                    direction: BeamDirection::Down,
                    position: (beam.position.0 + 1, beam.position.1),
                },
                map,
                movements,
            );
        }
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
            46
        );
    }

    #[test]
    fn test_solve_part2() {
        let day = Day {};
        assert_eq!(
            day.parse_and_solve_part2(format!("{}_test1", ID).as_str())
                .unwrap(),
            51
        );
    }
}
