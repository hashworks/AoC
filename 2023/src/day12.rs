mod util;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::{collections::HashMap, error::Error, io::BufRead};
use util::{aoc::AoCDay, input::get_reader};

const ID: &str = "day12";
type Input = Vec<(Vec<Option<State>>, Vec<usize>)>;
type Output = usize;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum State {
    Operational,
    Broken,
}

struct Day {}

impl AoCDay<Input, Output> for Day {
    fn parse_input(&self, id: &str) -> Result<Input, Box<dyn Error>> {
        get_reader(id)?
            .lines()
            .flatten()
            .map(|line| {
                let (spring_states_str, groups_str) =
                    line.split_once(' ').ok_or("Invalid input")?;

                let spring_states = spring_states_str
                    .chars()
                    .map(|c| match c {
                        '.' => Some(State::Operational),
                        '#' => Some(State::Broken),
                        _ => None,
                    })
                    .collect::<Vec<_>>();

                let groups = groups_str
                    .split(',')
                    .map(|c| c.parse::<usize>().map_err(|_| "Invalid input"))
                    .collect::<Result<Vec<_>, _>>()?;

                Ok((spring_states, groups))
            })
            .collect::<Result<Vec<_>, _>>()
    }

    fn part1(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        Ok(input
            .par_iter()
            .map(|(spring_states, groups)| calulate_arrangements_rec(spring_states, groups))
            .sum())
    }

    fn part2(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        Ok(input
            .par_iter()
            .map(|(spring_states, groups)| {
                let (spring_states, groups) = multiply(spring_states, groups);
                calulate_arrangements_rec(&spring_states, &groups)
            })
            .sum())
    }
}

fn multiply(
    spring_states: &[Option<State>],
    broken_group_sizes: &[usize],
) -> (Vec<Option<State>>, Vec<usize>) {
    let spring_states = [
        spring_states.to_owned(),
        vec![None],
        spring_states.to_owned(),
        vec![None],
        spring_states.to_owned(),
        vec![None],
        spring_states.to_owned(),
        vec![None],
        spring_states.to_owned(),
    ]
    .iter()
    .flatten()
    .cloned()
    .collect::<Vec<_>>();

    let broken_group_sizes = (0..5)
        .flat_map(|_| broken_group_sizes.iter().cloned())
        .collect::<Vec<_>>();
    (spring_states, broken_group_sizes)
}

fn calulate_arrangements_rec(
    spring_states: &Vec<Option<State>>,
    broken_group_sizes: &Vec<usize>,
) -> usize {
    // Only one cache per spring states, since it doesn't now about "operational_allowed" and
    // it is too expensive to copy it in every iterator anyway
    let mut cache = HashMap::new();
    calulate_arrangements_rec_internal(&mut cache, spring_states, broken_group_sizes, true)
}

fn calulate_arrangements_rec_internal(
    cache: &mut HashMap<(Vec<Option<State>>, Vec<usize>), usize>,
    spring_states: &Vec<Option<State>>,
    broken_group_sizes: &Vec<usize>,
    operational_allowed: bool,
) -> usize {
    if spring_states.is_empty() && broken_group_sizes.is_empty() {
        return 1;
    }

    if let Some(&arrangements) =
        cache.get(&(spring_states.to_owned(), broken_group_sizes.to_owned()))
    {
        return arrangements;
    }

    let mut arrangements = 0;

    if let Some(current_element) = spring_states.first() {
        match current_element {
            None => {
                if operational_allowed {
                    let mut permutation = spring_states.clone();
                    permutation[0] = Some(State::Operational);
                    arrangements += calulate_arrangements_rec_internal(
                        cache,
                        &permutation,
                        broken_group_sizes,
                        true,
                    );
                }
                let mut permutation = spring_states.clone();
                permutation[0] = Some(State::Broken);
                arrangements += calulate_arrangements_rec_internal(
                    cache,
                    &permutation,
                    broken_group_sizes,
                    true,
                );
            }
            Some(State::Operational) => {
                if operational_allowed {
                    let spring_states = spring_states.iter().skip(1).cloned().collect::<Vec<_>>();
                    arrangements += calulate_arrangements_rec_internal(
                        cache,
                        &spring_states,
                        broken_group_sizes,
                        true,
                    );
                }
            }
            Some(State::Broken) => {
                let mut broken_group_sizes = broken_group_sizes.to_owned();
                if let Some(expected_group_size) = broken_group_sizes.get_mut(0) {
                    if expected_group_size == &1 {
                        if spring_states.get(1) != Some(&Some(State::Broken)) {
                            let spring_states =
                                spring_states.iter().skip(2).cloned().collect::<Vec<_>>();
                            broken_group_sizes.remove(0);
                            arrangements += calulate_arrangements_rec_internal(
                                cache,
                                &spring_states,
                                &broken_group_sizes,
                                true,
                            );
                        }
                    } else if *expected_group_size > 1 {
                        *expected_group_size -= 1;
                        let spring_states =
                            spring_states.iter().skip(1).cloned().collect::<Vec<_>>();
                        arrangements += calulate_arrangements_rec_internal(
                            cache,
                            &spring_states,
                            &broken_group_sizes,
                            false, // [Some(Broken), None, None, None, None] [2, 1] is 2, not 3
                        );
                    }
                }
            }
        }
    }

    cache.insert(
        (spring_states.to_owned(), broken_group_sizes.to_owned()),
        arrangements,
    );

    arrangements
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
            21
        );
    }

    #[test]
    fn test_solve_part2() {
        let day = Day {};
        assert_eq!(
            day.parse_and_solve_part2(format!("{}_test1", ID).as_str())
                .unwrap(),
            525152
        );
    }

    #[test]
    fn test_calculate_arrangements_rec_1() {
        let (spring_states, groups) = (
            &vec![
                None,
                None,
                None,
                Some(State::Operational),
                Some(State::Broken),
                Some(State::Broken),
                Some(State::Broken),
            ],
            &vec![1, 1, 3],
        );
        assert_eq!(calulate_arrangements_rec(spring_states, groups), 1);
    }

    #[test]
    fn test_calculate_arrangements_rec_2() {
        let (spring_states, groups) = (
            &vec![
                Some(State::Operational),
                None,
                None,
                Some(State::Operational),
                Some(State::Operational),
                None,
                None,
                Some(State::Operational),
                Some(State::Operational),
                Some(State::Operational),
                None,
                Some(State::Broken),
                Some(State::Broken),
                Some(State::Operational),
            ],
            &vec![1, 1, 3],
        );
        assert_eq!(calulate_arrangements_rec(spring_states, groups), 4);
    }

    #[test]
    fn test_calculate_arrangements_rec_3() {
        let (spring_states, groups) = (
            &vec![
                None,
                Some(State::Broken),
                None,
                Some(State::Broken),
                None,
                Some(State::Broken),
                None,
                Some(State::Broken),
                None,
                Some(State::Broken),
                None,
                Some(State::Broken),
                None,
                Some(State::Broken),
                None,
            ],
            &vec![1, 3, 1, 6],
        );
        assert_eq!(calulate_arrangements_rec(spring_states, groups), 1);
    }

    #[test]
    fn test_calculate_arrangements_rec_4() {
        let (spring_states, groups) = (
            &vec![
                None,
                None,
                None,
                None,
                Some(State::Operational),
                Some(State::Broken),
                Some(State::Operational),
                Some(State::Operational),
                Some(State::Operational),
                Some(State::Broken),
                Some(State::Operational),
                Some(State::Operational),
                Some(State::Operational),
            ],
            &vec![4, 1, 1],
        );
        assert_eq!(calulate_arrangements_rec(spring_states, groups), 1);
    }

    #[test]
    fn test_calculate_arrangements_rec_5() {
        let (spring_states, groups) = (
            &vec![
                None,
                None,
                None,
                None,
                Some(State::Operational),
                Some(State::Broken),
                Some(State::Broken),
                Some(State::Broken),
                Some(State::Broken),
                Some(State::Broken),
                Some(State::Broken),
                Some(State::Operational),
                Some(State::Operational),
                Some(State::Broken),
                Some(State::Broken),
                Some(State::Broken),
                Some(State::Broken),
                Some(State::Broken),
                Some(State::Operational),
            ],
            &vec![1, 6, 5],
        );
        assert_eq!(calulate_arrangements_rec(spring_states, groups), 4);
    }

    #[test]
    fn test_calculate_arrangements_rec_6() {
        let (spring_states, groups) = (
            &vec![
                None,
                Some(State::Broken),
                Some(State::Broken),
                Some(State::Broken),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            ],
            &vec![3, 2, 1],
        );
        assert_eq!(calulate_arrangements_rec(spring_states, groups), 10);
    }

    #[test]
    fn test_calculate_arrangements_rec_multiply_1() {
        let (spring_states, groups) = multiply(
            &[
                None,
                None,
                None,
                Some(State::Operational),
                Some(State::Broken),
                Some(State::Broken),
                Some(State::Broken),
            ],
            &[1, 1, 3],
        );
        assert_eq!(calulate_arrangements_rec(&spring_states, &groups), 1);
    }

    #[test]
    fn test_calculate_arrangements_rec_multiply_2() {
        let (spring_states, groups) = multiply(
            &[
                Some(State::Operational),
                None,
                None,
                Some(State::Operational),
                Some(State::Operational),
                None,
                None,
                Some(State::Operational),
                Some(State::Operational),
                Some(State::Operational),
                None,
                Some(State::Broken),
                Some(State::Broken),
                Some(State::Operational),
            ],
            &[1, 1, 3],
        );
        assert_eq!(calulate_arrangements_rec(&spring_states, &groups), 16384);
    }

    #[test]
    fn test_calculate_arrangements_rec_multiply_3() {
        let (spring_states, groups) = multiply(
            &[
                None,
                Some(State::Broken),
                None,
                Some(State::Broken),
                None,
                Some(State::Broken),
                None,
                Some(State::Broken),
                None,
                Some(State::Broken),
                None,
                Some(State::Broken),
                None,
                Some(State::Broken),
                None,
            ],
            &[1, 3, 1, 6],
        );
        assert_eq!(calulate_arrangements_rec(&spring_states, &groups), 1);
    }

    #[test]
    fn test_calculate_arrangements_rec_multiply_4() {
        let (spring_states, groups) = multiply(
            &[
                None,
                None,
                None,
                None,
                Some(State::Operational),
                Some(State::Broken),
                Some(State::Operational),
                Some(State::Operational),
                Some(State::Operational),
                Some(State::Broken),
                Some(State::Operational),
                Some(State::Operational),
                Some(State::Operational),
            ],
            &[4, 1, 1],
        );
        assert_eq!(calulate_arrangements_rec(&spring_states, &groups), 16);
    }

    #[test]
    fn test_calculate_arrangements_rec_multiply_5() {
        let (spring_states, groups) = multiply(
            &[
                None,
                None,
                None,
                None,
                Some(State::Operational),
                Some(State::Broken),
                Some(State::Broken),
                Some(State::Broken),
                Some(State::Broken),
                Some(State::Broken),
                Some(State::Broken),
                Some(State::Operational),
                Some(State::Operational),
                Some(State::Broken),
                Some(State::Broken),
                Some(State::Broken),
                Some(State::Broken),
                Some(State::Broken),
                Some(State::Operational),
            ],
            &[1, 6, 5],
        );
        assert_eq!(calulate_arrangements_rec(&spring_states, &groups), 2500);
    }

    #[test]
    fn test_calculate_arrangements_rec_multiply_6() {
        let (spring_states, groups) = multiply(
            &[
                None,
                Some(State::Broken),
                Some(State::Broken),
                Some(State::Broken),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            ],
            &[3, 2, 1],
        );
        assert_eq!(calulate_arrangements_rec(&spring_states, &groups), 506250);
    }

    #[test]
    fn test_calculate_arrangements_rec_edge_case() {
        let (spring_states, groups) = (
            &vec![Some(State::Broken), None, None, None, None],
            &vec![2, 1],
        );
        assert_eq!(calulate_arrangements_rec(spring_states, groups), 2,);
    }
}
