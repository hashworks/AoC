use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn simulate_universe_step(constellations: Vec<[i16; 6]>) -> Vec<[i16; 6]> {
    let constellations: Vec<[i16; 6]> = constellations
        .clone()
        .into_iter()
        .map(|moon| {
            let mut moon = moon.clone();
            for other_moon in constellations.clone() {
                if moon != other_moon {
                    for i in 0..3 {
                        moon[i + 3] += if moon[i] > other_moon[i] {
                            -1
                        } else if moon[i] < other_moon[i] {
                            1
                        } else {
                            0
                        };
                    }
                }
            }
            moon
        })
        .collect();
    constellations
        .into_iter()
        .map(|moon| {
            let mut moon = moon.clone();
            for i in 0..3 {
                moon[i] += moon[i + 3];
            }
            moon
        })
        .collect()
}

fn main() {
    let s1 = Instant::now();

    let mut constellations: Vec<[i16; 6]> = BufReader::new(File::open("./input").unwrap())
        .lines()
        .map(|l| {
            let xyz = l
                .unwrap()
                .chars()
                .filter(|c| c == &'-' || c == &',' || (c >= &'0' && c <= &'9'))
                .collect::<String>()
                .split(',')
                .take(3)
                .map(|xyz| xyz.parse::<i16>().unwrap())
                .collect::<Vec<i16>>();
            [xyz[0], xyz[1], xyz[2], 0, 0, 0]
        })
        .collect();

    let mut p1_constellations = constellations.clone();
    for _ in 0..1000 {
        p1_constellations = simulate_universe_step(p1_constellations);
    }

    println!(
        "part1: {:?}, ({}µs)",
        p1_constellations.into_iter().fold(0, |a, m| {
            a + (m[0].abs() + m[1].abs() + m[2].abs()) * (m[3].abs() + m[4].abs() + m[5].abs())
        }),
        s1.elapsed().as_micros()
    );

    let s2 = Instant::now();

    println!("part2: TODO, ({}s)", s2.elapsed().as_secs());

    println!("time: {}s", s1.elapsed().as_secs());
}
