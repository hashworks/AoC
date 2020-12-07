use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::time::Instant;

extern crate nom;

use nom::{
    bytes::complete::{tag, take_until},
    character::streaming::digit1,
    error::Error,
    IResult,
};

/**
 * dark olive bags contain 3 faded blue bags, 4 dotted black bags, [...].
 * vibrant plum bags contain 5 faded blue bags.
 * dotted black bags contain no other bags.
 */
fn parser(input: &str) -> IResult<&str, Vec<(u32, &str)>> {
    let (input, color) = take_until(" bags")(input)?;
    let (input, _) = tag(" bags contain ")(input)?;
    let mut vec = vec![];
    for contains in input.split(", ") {
        if let Ok((contains, contains_count)) = digit1::<&str, Error<&str>>(contains) {
            let (contains, _) = tag(" ")(contains)?;
            let (_, contains_color) = take_until(" bag")(contains)?;
            vec.push((contains_count.parse::<u32>().unwrap(), contains_color));
        }
    }
    Ok((color, vec))
}

fn part1_depth_first_search(
    result: &mut HashSet<String>,
    map: &HashMap<&str, Vec<(u32, &str)>>,
    color: &str,
) {
    let mut maybe_vec = map.get(color);
    for (_, parent_color) in maybe_vec.get_or_insert(&vec![]).iter() {
        result.insert(parent_color.to_string());
        part1_depth_first_search(result, map, parent_color);
    }
}

fn part1(qd: &String) -> usize {
    let mut map: HashMap<&str, Vec<(u32, &str)>> = HashMap::new();
    for line in qd.lines() {
        let (color, vec) = parser(line).unwrap();
        for (contains_count, contains_color) in vec {
            map.entry(contains_color)
                .or_insert(vec![])
                .push((contains_count, color));
        }
    }
    let mut result: HashSet<String> = HashSet::new();
    part1_depth_first_search(&mut result, &map, "shiny gold");
    result.len()
}

fn part2_depth_first_search(map: &HashMap<&str, Vec<(u32, &str)>>, color: &str) -> u32 {
    map.get(color)
        .get_or_insert(&vec![])
        .iter()
        .map(|(count, child_color)| count + count * part2_depth_first_search(map, child_color))
        .sum()
}

fn part2(qd: &String) -> u32 {
    let mut map: HashMap<&str, Vec<(u32, &str)>> = HashMap::new();
    for line in qd.lines() {
        let (color, vec) = parser(line).unwrap();
        for tuple in vec {
            map.entry(color).or_insert(vec![]).push(tuple);
        }
    }
    part2_depth_first_search(&map, "shiny gold")
}

fn main() {
    let s1 = Instant::now();

    let question_data = read_to_string("input").unwrap();

    println!(
        "part1: {} ({}µs)",
        part1(&question_data),
        s1.elapsed().as_micros()
    );

    let s2 = Instant::now();

    println!(
        "part2: {} ({}µs)",
        part2(&question_data),
        s2.elapsed().as_micros()
    );

    println!("Time: {}µs", s1.elapsed().as_micros());
}

#[test]
fn part1_test() {
    assert_eq!(
        4,
        part1(
            &"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."
                .to_string()
        )
    )
}

#[test]
fn part2_test() {
    assert_eq!(
        126,
        part2(
            &"shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags."
                .to_string()
        )
    )
}
