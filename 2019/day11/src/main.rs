// Note: Since the "intcode computer" should be reused later
// it is implemented in the crate `../intcode`.
extern crate intcode;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str;
use std::time::Instant;

fn emergency_hull_painting_robot(
    m: Vec<i64>,
    start: i64,
) -> (HashMap<(i64, i64), i64>, i64, i64, i64, i64) {
    let mut min_x = 0;
    let mut min_y = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    let mut painted_panels: HashMap<(i64, i64), i64> = HashMap::new();
    let mut x = 0;
    let mut y = 0;
    let mut d = 0;

    let (_, input_tx, output_rx, result_rx) = intcode::compute_threaded(m, None, None, None);
    let input_tx = input_tx.unwrap();
    let output_rx = output_rx.unwrap();
    let result_rx = result_rx.unwrap();

    input_tx.send(start).unwrap();

    while result_rx.try_recv().is_err() {
        if let Ok(paint) = output_rx.try_recv() {
            d = match output_rx.recv().unwrap() {
                0 => (d - 1) % 4, // left
                1 => (d + 1) % 4, // right
                _ => panic!("Unknown direction"),
            };
            if d < 0 {
                d += 4;
            }
            painted_panels.insert((x, y), paint);
            match d {
                0 => y += 1, // up
                1 => x += 1, // right
                2 => y -= 1, // down
                3 => x -= 1, // left
                d => panic!(format!("Unknown direction '{}'", d)),
            }
            input_tx
                .send(*painted_panels.get(&(x, y)).unwrap_or(&0))
                .unwrap();
            if x > max_x {
                max_x = x;
            } else if x < min_x {
                min_x = x;
            }
            if y > max_y {
                max_y = y;
            } else if y < min_y {
                min_y = y;
            }
        }
    }

    (painted_panels, min_x, min_y, max_x, max_y)
}

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

    let (painted_panels, _, _, _, _) = emergency_hull_painting_robot(m.clone(), 0);

    println!(
        "part1: {:?}, ({}µs)",
        painted_panels.len(),
        s1.elapsed().as_micros()
    );

    let s2 = Instant::now();

    let (painted_panels, min_x, min_y, max_x, max_y) = emergency_hull_painting_robot(m.clone(), 1);

    for y in (min_y..max_y + 1).rev() {
        for x in min_x..max_x + 1 {
            print!(
                "{}",
                if painted_panels.get(&(x, y)).unwrap_or(&0) == &1 {
                    "█"
                } else {
                    " "
                }
            );
        }
        println!("");
    }

    println!("part2: read it!, ({}µs)", s2.elapsed().as_micros());

    println!("time: {}µs", s1.elapsed().as_micros());
}
