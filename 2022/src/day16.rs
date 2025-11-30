mod util;

use nom::branch::alt;
use nom::bytes::complete as nom_bytes;
use nom::character::complete as nom_char;
use nom::multi::separated_list1;
use nom::sequence::pair;
use nom::{sequence::tuple, IResult};
use pathfinding::prelude::bfs;

use std::io::BufRead;
use std::{collections::HashMap, error::Error};
use util::{aoc::AoCDay, input::get_reader};

const ID: &str = "day16";
type Key = (char, char);
type Input = HashMap<Key, (u64, Vec<Key>)>;
type InputWithCost = HashMap<Key, (u64, Vec<(Key, u64)>)>;
type Output = u64;

fn parse_sensor_report(i: &str) -> IResult<&str, (Key, u64, Vec<Key>)> {
    let (i, (_, valve, _, flow_rate, _, leads_to_valves)) = tuple((
        nom_bytes::tag("Valve "),
        pair(nom_char::anychar, nom_char::anychar),
        nom_bytes::tag(" has flow rate="),
        nom_char::u64,
        alt((
            nom_bytes::tag("; tunnels lead to valves "),
            nom_bytes::tag("; tunnel leads to valve "),
        )),
        separated_list1(
            nom_bytes::tag(", "),
            pair(nom_char::anychar, nom_char::anychar),
        ),
    ))(i)?;
    Ok((i, (valve, flow_rate, leads_to_valves)))
}

struct Day {}

impl AoCDay<Input, Output> for Day {
    fn parse_input(&self, id: &str) -> Result<Input, Box<dyn Error>> {
        get_reader(id)?
            .lines()
            .map(|l| match parse_sensor_report(&l?) {
                Ok((_, (key, flow_rate, leads_to_valves))) => {
                    Ok((key, (flow_rate, leads_to_valves)))
                }
                Err(e) => Err(e.to_string().into()),
            })
            .collect()
    }

    fn part1(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        // Drop all zero flow rate "valves", we need only paths from valve to valve
        let working_vales = drop_zero_flow_valves(input);

        Ok(max_pressure(&working_vales, ('A', 'A'), 30, vec![]))
    }

    fn part2(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        // Drop all zero flow rate "valves", we need only paths from valve to valve
        let working_vales = drop_zero_flow_valves(input);

        Ok(max_pressure_pair(
            &working_vales,
            ('A', 'A'),
            30,
            ('A', 'A'),
            30,
            vec![],
        ))
    }
}

fn drop_zero_flow_valves(input: &Input) -> InputWithCost {
    let working_vales = input
        .iter()
        .filter(|&(&key, &(flow_rate, _))| key == ('A', 'A') || flow_rate > 0)
        .map(|(key, &(flow_rate, _))| {
            (
                *key,
                (
                    flow_rate,
                    input
                        .iter()
                        .filter(|&(_, &(flow_rate, _))| flow_rate > 0)
                        .filter(|&(other_key, (_, _))| other_key != key)
                        .filter_map(|(&other_key, (_, _))| {
                            let path = bfs(
                                key,
                                |n| input.get(n).unwrap().1.clone(),
                                |&n| n == other_key,
                            );
                            path.map(|path| (other_key, path.len() as u64 - 1))
                        })
                        .collect::<Vec<_>>(),
                ),
            )
        })
        .collect();
    working_vales
}

fn max_pressure(input: &InputWithCost, position: Key, minute: u64, opened_valves: Vec<Key>) -> u64 {
    if minute == 1 {
        return 0;
    }

    let (flow_rate, leads_to_valves) = input.get(&position).unwrap();

    // Open the valve, since we don't walk to valves without reason
    let mut opened_valves = opened_valves;
    opened_valves.push(position);

    // Add maximum of next paths
    minute * flow_rate
        + leads_to_valves
            .iter()
            .filter(|(valve, cost)| !opened_valves.contains(valve) && *cost < minute)
            .map(|&(valve, cost)| {
                max_pressure(input, valve, minute - cost - 1, opened_valves.clone())
            })
            .max()
            .unwrap_or(0)
}

fn max_pressure_pair(
    input: &InputWithCost,
    position_one: Key,
    minute_one: u64,
    position_two: Key,
    minute_two: u64,
    opened_valves: Vec<Key>,
) -> u64 {
    // Open the valve, since we don't walk to valves without reason
    let mut opened_valves = opened_valves;
    opened_valves.push(position_one);

    let &(flow_rate, _) = input.get(&position_one).unwrap();

    minute_one * flow_rate
        + [
            (position_one, minute_one, position_two, minute_two),
            (position_two, minute_two, position_one, minute_one),
        ]
        .iter()
        .filter(|(_, minute_current, _, _)| *minute_current > 1)
        .map(
            |(position_current, minute_current, position_other, minute_other)| {
                let (_, leads_to_valves) = input.get(position_current).unwrap();
                leads_to_valves
                    .iter()
                    .filter(|(valve, cost)| {
                        !opened_valves.contains(valve) && *cost < *minute_current
                    })
                    .map(|&(valve, cost)| {
                        max_pressure_pair(
                            input,
                            valve,
                            *minute_current - cost - 1,
                            *position_other,
                            *minute_other,
                            opened_valves.clone(),
                        )
                    })
                    .max()
                    .unwrap_or(0)
            },
        )
        .max()
        .unwrap_or(0)
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
            1651
        );
    }

    #[test]
    fn test_solve_part2() {
        let day = Day {};
        assert_eq!(
            day.parse_and_solve_part2(format!("{}_test1", ID).as_str())
                .unwrap(),
            1707
        );
    }
}

fn main() {
    Day {}.run(ID);
}
