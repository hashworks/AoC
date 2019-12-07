use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn part1(input: &str, orbits_map: &mut HashMap<String, String>) -> i32 {
    BufReader::new(File::open(input).unwrap())
        .lines()
        .for_each(|line| {
            let line = line.unwrap();
            let data = line.split(')').take(2).collect::<Vec<&str>>();
            let (object, orbits) = (data[1].to_string(), data[0].to_string());
            orbits_map.insert(object, orbits);
        });

    let mut cache: HashMap<String, i32> = HashMap::new();
    orbits_map.clone().into_iter().fold(0, |acc, (_, orbit)| {
        acc + part_1_get_orbit_count(&orbits_map, &mut cache, &orbit)
    })
}

fn part_1_get_orbit_count(
    orbits_map: &HashMap<String, String>,
    cache: &mut HashMap<String, i32>,
    orbit: &String,
) -> i32 {
    if orbit == "COM" {
        1
    } else if let Some(orbit_cache) = cache.get(orbit) {
        *orbit_cache
    } else {
        let orbit_count =
            part_1_get_orbit_count(orbits_map, cache, orbits_map.get(orbit).unwrap()) + 1;
        cache.insert(orbit.to_string(), orbit_count);
        orbit_count
    }
}

fn part2(orbits_map: &mut HashMap<String, String>) -> Option<i32> {
    let mut you_hashset: HashSet<&String> = HashSet::new();
    let mut you: Vec<&String> = vec![orbits_map.get("YOU").unwrap()];
    while you[you.len() - 1] != "COM" {
        let object = orbits_map.get(you[you.len() - 1]).unwrap();
        you.push(object);
        you_hashset.insert(object);
    }
    let mut count = 0;
    let mut san: Vec<&String> = vec![orbits_map.get("SAN").unwrap()];
    while san[san.len() - 1] != "COM" {
        let mut you_orbit_count = 0;
        if you_hashset.contains(&san[san.len() - 1]) {
            for you_orbit in &you {
                if you_orbit == &san[san.len() - 1] {
                    return Some(count + you_orbit_count);
                }
                you_orbit_count += 1;
            }
            return None;
        }
        count += 1;
        san.push(orbits_map.get(san[san.len() - 1]).unwrap())
    }
    None
}

fn main() {
    let s1 = Instant::now();

    let mut orbits_map: HashMap<String, String> = HashMap::new();

    println!(
        "{} ({}µs)",
        part1("./input", &mut orbits_map),
        s1.elapsed().as_micros()
    );

    let s2 = Instant::now();

    println!(
        "{} ({}µs)",
        part2(&mut orbits_map).unwrap(),
        s2.elapsed().as_micros()
    );

    println!("Time: {}µs", s1.elapsed().as_micros());
}
