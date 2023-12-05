mod util;

use rayon::prelude::*;
use std::{error::Error, io::BufRead};
use util::{aoc::AoCDay, input::get_reader};

const ID: &str = "day05";
type Input = Almanac;
type Output = usize;

struct Day {}

struct Almanac {
    seeds: Vec<usize>,
    seed_to_soil_map: Vec<AlmanacMapper>,
    soil_to_fertilizer_map: Vec<AlmanacMapper>,
    fertilizer_to_water_map: Vec<AlmanacMapper>,
    water_to_light_map: Vec<AlmanacMapper>,
    light_to_temperature_map: Vec<AlmanacMapper>,
    temperature_to_humidity_map: Vec<AlmanacMapper>,
    humidity_to_location_map: Vec<AlmanacMapper>,
}

#[derive(Debug)]
struct AlmanacMapper {
    destination_start: usize,
    source_start: usize,
    length: usize,
}

impl AoCDay<Input, Output> for Day {
    /*
    seeds: 79 14 55 13

    seed-to-soil map:
    50 98 2
    52 50 48

    soil-to-fertilizer map:
    0 15 37
    37 52 2
    39 0 15

    fertilizer-to-water map:
    49 53 8
    0 11 42
    42 0 7
    57 7 4

    water-to-light map:
    88 18 7
    18 25 70

    light-to-temperature map:
    45 77 23
    81 45 19
    68 64 13

    temperature-to-humidity map:
    0 69 1
    1 0 69

    humidity-to-location map:
    60 56 37
    56 93 4
         */
    fn parse_input(&self, id: &str) -> Result<Input, Box<dyn Error>> {
        let reader = get_reader(id)?;

        let mut lines = reader.lines();

        let seeds = lines
            .next()
            .ok_or("bad input")??
            .split(": ")
            .nth(1)
            .ok_or("bad input")?
            .split(' ')
            .map(|s| s.parse::<usize>())
            .collect::<Result<Vec<_>, _>>()?;

        let mut lines = lines.skip(1);

        let seed_to_soil_map = parse_destination_source_map(&mut lines)?;
        let soil_to_fertilizer_map = parse_destination_source_map(&mut lines)?;
        let fertilizer_to_water_map = parse_destination_source_map(&mut lines)?;
        let water_to_light_map = parse_destination_source_map(&mut lines)?;
        let light_to_temperature_map = parse_destination_source_map(&mut lines)?;
        let temperature_to_humidity_map = parse_destination_source_map(&mut lines)?;
        let humidity_to_location_map = parse_destination_source_map(&mut lines)?;

        Ok(Almanac {
            seeds,
            seed_to_soil_map,
            soil_to_fertilizer_map,
            fertilizer_to_water_map,
            water_to_light_map,
            light_to_temperature_map,
            temperature_to_humidity_map,
            humidity_to_location_map,
        })
    }

    fn part1(&self, almanac: &Input) -> Result<Output, Box<dyn Error>> {
        Ok(almanac
            .seeds
            .iter()
            .map(|s| source_map(*s, &almanac.seed_to_soil_map))
            .map(|s| source_map(s, &almanac.soil_to_fertilizer_map))
            .map(|s| source_map(s, &almanac.fertilizer_to_water_map))
            .map(|s| source_map(s, &almanac.water_to_light_map))
            .map(|s| source_map(s, &almanac.light_to_temperature_map))
            .map(|s| source_map(s, &almanac.temperature_to_humidity_map))
            .map(|s| source_map(s, &almanac.humidity_to_location_map))
            .min()
            .unwrap())
    }

    fn part2(&self, almanac: &Input) -> Result<Output, Box<dyn Error>> {
        let mut stupid_but_possible = Vec::new();

        for ranges in almanac.seeds.chunks_exact(2) {
            for i in ranges[0]..(ranges[0] + ranges[1]) {
                stupid_but_possible.push(i);
            }
        }

        Ok(stupid_but_possible
            .par_iter()
            .map(|s| source_map(*s, &almanac.seed_to_soil_map))
            .map(|s| source_map(s, &almanac.soil_to_fertilizer_map))
            .map(|s| source_map(s, &almanac.fertilizer_to_water_map))
            .map(|s| source_map(s, &almanac.water_to_light_map))
            .map(|s| source_map(s, &almanac.light_to_temperature_map))
            .map(|s| source_map(s, &almanac.temperature_to_humidity_map))
            .map(|s| source_map(s, &almanac.humidity_to_location_map))
            .min()
            .unwrap())
    }
}

fn source_map(source: usize, mappers: &[AlmanacMapper]) -> usize {
    mappers
        .iter()
        .find(|mapper| {
            source >= mapper.source_start && source < mapper.source_start + mapper.length
        })
        .map(|mapper| source - mapper.source_start + mapper.destination_start)
        .unwrap_or(source)
}

fn parse_destination_source_map(
    lines: &mut std::iter::Skip<std::io::Lines<std::io::BufReader<std::fs::File>>>,
) -> Result<Vec<AlmanacMapper>, Box<dyn Error>> {
    // map-name map:
    let lines = lines.skip(1);

    let mut ranges = Vec::new();
    for line in lines {
        let line = line?;
        if line.is_empty() {
            break;
        }

        let mut numbers = line.split(' ').map(|s| s.parse::<usize>());

        let destination_start = numbers.next().ok_or("bad input")??;
        let source_start = numbers.next().ok_or("bad input")??;
        let length = numbers.next().ok_or("bad input")??;

        ranges.push(AlmanacMapper {
            destination_start,
            source_start,
            length,
        });
    }
    Ok(ranges)
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
            35
        );
    }

    #[test]
    fn test_solve_part2() {
        let day = Day {};
        assert_eq!(
            day.parse_and_solve_part2(format!("{}_test1", ID).as_str())
                .unwrap(),
            46
        );
    }
}
