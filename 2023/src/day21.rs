mod util;

use pathfinding::directed::dijkstra::dijkstra_all;
use std::{collections::HashSet, error::Error, io::BufRead};
use util::{aoc::AoCDay, input::get_reader};

const ID: &str = "day21";
type Input = InfiniteMap;
type Output = usize;

struct InfiniteMap {
    rocks: HashSet<(isize, isize)>,
    starting_position: (isize, isize),
    limits: (isize, isize),
}

struct Day {}

impl AoCDay<Input, Output> for Day {
    fn parse_input(&self, id: &str) -> Result<Input, Box<dyn Error>> {
        let mut starting_position = (0, 0);
        let mut rocks = HashSet::new();
        let mut max_y = 0;
        let mut max_x = 0;
        get_reader(id)?
            .lines()
            .flatten()
            .enumerate()
            .for_each(|(y, line)| {
                line.chars().enumerate().for_each(|(x, c)| {
                    if y > max_y {
                        max_y = y;
                    }
                    if x > max_x {
                        max_x = x;
                    }
                    match c {
                        '#' => {
                            rocks.insert((y as isize, x as isize));
                        }
                        'S' => {
                            starting_position = (y as isize, x as isize);
                        }
                        _ => {}
                    }
                })
            });

        Ok(InfiniteMap {
            rocks,
            starting_position,
            limits: (max_y as isize, max_x as isize),
        })
    }

    fn part1(&self, map: &Input) -> Result<Output, Box<dyn Error>> {
        Ok(map.count_reachable_garden_plots(64))
    }

    fn part2(&self, _map: &Input) -> Result<Output, Box<dyn Error>> {
        unimplemented!()
        //Ok(map.count_reachable_garden_plots(26501365))
    }
}

impl InfiniteMap {
    // Solution without map-assertions, for low max_steps
    #[allow(dead_code)]
    fn count_reachable_garden_plots_dijkstra(&self, max_steps: isize) -> usize {
        let reachable_nodes = dijkstra_all(
            &(0, self.starting_position.0, self.starting_position.1),
            |(steps, y, x)| {
                let steps = steps + 1;
                if steps <= max_steps {
                    vec![(*y - 1, *x), (*y + 1, *x), (*y, *x - 1), (*y, *x + 1)]
                        .into_iter()
                        .filter(|(y, x)| !self.rocks.contains(&(*y, *x)))
                        .map(|(y, x)| ((steps, y, x), 0))
                        .collect::<Vec<_>>()
                } else {
                    vec![]
                }
            },
        )
        .keys()
        .filter(|(steps, _, _)| steps == &max_steps)
        .map(|(_, y, x)| (*y, *x))
        .collect::<HashSet<_>>();

        reachable_nodes.len()
    }

    fn inifinite_contains(&self, y: isize, x: isize) -> bool {
        self.rocks.contains(&(y % self.limits.0, x % self.limits.1))
    }

    fn count_reachable_garden_plots(&self, max_steps: isize) -> usize {
        let start_y = self.starting_position.0;
        let start_x = self.starting_position.1;

        let possible_garden_pots = (0..=max_steps)
            .flat_map(|y| {
                (y..=(max_steps * 2 - y))
                    .filter(move |x| (x - y) % 2 == 0)
                    .flat_map(move |x| {
                        [
                            (y + start_y, start_x - max_steps + x),
                            (-y + start_y, start_x - max_steps + x),
                        ]
                    })
            })
            .filter(|(y, x)| {
                !self.inifinite_contains(*y, *x)
                    && [(-1, 0), (1, 0), (0, -1), (0, 1)]
                        .iter()
                        .any(|(yd, xd)| !self.inifinite_contains(*y + yd, *x + xd))
            })
            .collect::<HashSet<(isize, isize)>>();

        possible_garden_pots.len()
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

        let map = day.parse_input(format!("{}_test1", ID).as_str()).unwrap();

        assert_eq!(map.count_reachable_garden_plots_dijkstra(6), 16);
    }

    #[test]
    fn test_solve_part2() {
        let day = Day {};
        assert_eq!(
            day.parse_and_solve_part2(format!("{}_test1", ID).as_str())
                .unwrap(),
            0
        );
    }
}
