// Note: Since the "intcode computer" should be reused later
// it is implemented in the crate `../intcode`.
extern crate intcode;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str;
//use std::sync::mpsc::{Receiver, Sender};
use std::time::Instant;

type Map = HashMap<(i64, i64), i64>;

fn main() {
    let s1 = Instant::now();

    let m = BufReader::new(File::open("./input").unwrap())
        .split(b',')
        .map(|ops| {
            str::from_utf8(&ops.unwrap())
                .unwrap()
                .trim()
                .parse::<i64>()
                .unwrap()
        })
        .collect::<Vec<i64>>();

    let mut map: Map = HashMap::new();

    let mut calibration = 0;
    let mut x = 0;
    let mut y = 0;
    for point in intcode::compute(m.clone(), Vec::new()).1 {
        map.insert(
            (x, y),
            if point == 10 {
                y += 1;
                x = 0;
                continue;
            } else {
                if map.get(&(x + 1, y - 1)) == Some(&35)
                    && map.get(&(x - 1, y - 1)) == Some(&35)
                    && map.get(&(x, y - 1)) == Some(&35)
                    && map.get(&(x, y - 2)) == Some(&35)
                {
                    calibration += x * (y - 1);
                }
                point
            },
        );
        x += 1;
    }

    for y in (0..y).rev() {
        for x in 0..50 {
            if let Some(point) = map.get(&(x, y)) {
                print!("{}", (*point as u8) as char);
            }
        }
        println!("");
    }

    println!("part1: {:?}, ({}µs)", calibration, s1.elapsed().as_micros());

    let s2 = Instant::now();

    println!("part2: {}, ({}µs)", 0, s2.elapsed().as_micros());

    println!("time: {}µs", s1.elapsed().as_micros());
}
