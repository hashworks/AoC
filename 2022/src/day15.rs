mod util;

use nom::bytes::complete as nom_bytes;
use nom::character::complete as nom_char;
use nom::{sequence::tuple, IResult};
use rayon::prelude::*;
use std::collections::HashSet;
use std::error::Error;
use std::io::BufRead;
use util::{aoc::AoCDay, input::get_reader};

const ID: &str = "day15";
type Point = (i32, i32);
type Input = Vec<(Point, Point)>;
type Output = i64;

fn parse_sensor_report(i: &str) -> IResult<&str, (Point, Point)> {
    let (i, (_, s_x, _, s_y, _, b_x, _, b_y)) = tuple((
        nom_bytes::tag("Sensor at x="),
        nom_char::i32,
        nom_bytes::tag(", y="),
        nom_char::i32,
        nom_bytes::tag(": closest beacon is at x="),
        nom_char::i32,
        nom_bytes::tag(", y="),
        nom_char::i32,
    ))(i)?;
    Ok((i, ((s_x, s_y), (b_x, b_y))))
}
struct Day {}

impl AoCDay<Input, Output> for Day {
    fn parse_input(&self, id: &str) -> Result<Input, Box<dyn Error>> {
        let reader = get_reader(id)?;

        reader
            .lines()
            .filter_map(|line| line.ok())
            .map(|line| match parse_sensor_report(line.as_str()) {
                Ok((_, data)) => Ok(data),
                Err(e) => Err(e.to_string().into()),
            })
            .collect()
    }

    fn part1(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        #[cfg(test)]
        const Y_FILTER: i32 = 10;

        #[cfg(not(test))]
        const Y_FILTER: i32 = 2_000_000;

        let sensors_and_beacons = input
            .par_iter()
            .flat_map(|(sensor, beacon)| [sensor, beacon])
            .collect::<HashSet<_>>();

        let no_beacon_points = input
            .par_iter()
            .filter_map(|(sensor, beacon)| {
                let manhatten_distance = (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs();
                let y_distance = (sensor.1 - Y_FILTER).abs();
                if manhatten_distance >= y_distance {
                    let x_max_distance = manhatten_distance - y_distance;
                    return Some((sensor.0, x_max_distance));
                }
                None
            })
            .flat_map(|(sensor_x, x_max_distance)| {
                (-x_max_distance..=x_max_distance)
                    .map(|x| sensor_x + x)
                    .collect::<Vec<_>>()
            })
            .filter(|&x| !sensors_and_beacons.contains(&(x, Y_FILTER)))
            .collect::<HashSet<_>>();

        Ok(no_beacon_points.len() as i64)
    }

    fn part2(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        #[cfg(test)]
        const MAX_X_Y: i32 = 20;
        #[cfg(not(test))]
        const MAX_X_Y: i32 = 4_000_000;

        const MAX_X_Y_RANGE: std::ops::RangeInclusive<i32> = 0..=MAX_X_Y;

        // Note: [_; MAX_X_Y as usize + 1] would blow the stack
        let mut no_beacon_ranges_per_y = vec![Vec::<(i32, i32)>::new(); MAX_X_Y as usize + 1];

        for (sensor, beacon) in input {
            let y_distance = (sensor.1 - beacon.1).abs();
            let manhatten_distance = (sensor.0 - beacon.0).abs() + y_distance;

            for y_distance in -manhatten_distance..=manhatten_distance {
                let y = sensor.1 + y_distance;
                if !MAX_X_Y_RANGE.contains(&y) {
                    continue;
                }

                let x_max_distance = manhatten_distance - y_distance.abs();

                let x_min = (sensor.0 - x_max_distance).max(0);
                let x_max = (sensor.0 + x_max_distance).min(MAX_X_Y);

                insert_range(&mut no_beacon_ranges_per_y[y as usize], (x_min, x_max));
            }
        }

        no_beacon_ranges_per_y
            .par_iter()
            .enumerate()
            .find_any(|(_, ranges)| ranges.len() == 2 && ranges[1].0 - ranges[0].1 == 2)
            .map(|(y, ranges)| (ranges[0].1 + 1) as i64 * 4000000 + y as i64)
            .ok_or("no solution found".into())
    }
}

fn insert_range(ranges: &mut Vec<(i32, i32)>, range: (i32, i32)) {
    let mut i = 0;
    while i < ranges.len() {
        let (start, end) = ranges[i];
        if range.0 > end {
            i += 1;
            continue;
        }
        if range.1 < start {
            ranges.insert(i, range);
            return;
        }
        if range.0 < start {
            ranges[i].0 = range.0;
        }
        if range.1 > end {
            ranges[i].1 = range.1;
        }
        let j = i + 1;
        while j < ranges.len() {
            let (start, end) = ranges[j];
            if range.1 < start {
                return;
            }
            if range.1 < end {
                ranges[i].1 = end;
                ranges.remove(j);
                return;
            }
            ranges.remove(j);
        }
        return;
    }
    ranges.push(range);
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
            26
        );
    }

    #[test]
    fn test_solve_part2() {
        let day = Day {};
        assert_eq!(
            day.parse_and_solve_part2(format!("{}_test1", ID).as_str())
                .unwrap(),
            56000011
        );
    }
}

fn main() {
    Day {}.run(ID);
}
