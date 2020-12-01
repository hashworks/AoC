// Note: Since the "intcode computer" should be reused later
// it is implemented in the crate `../intcode`.
extern crate intcode;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str;
use std::sync::mpsc::{Receiver, Sender};
use std::time::Instant;

type Map = HashMap<(i64, i64), u8>;

fn traverse(
    input_tx: &Sender<i64>,
    output_rx: &Receiver<i64>,
    previous: Option<&i64>,
    map: &mut Map,
    pos: (i64, i64),
) -> (u64, i64, (i64, i64)) {
    const OPPOSITE_DIRECTION: [i64; 4] = [2, 1, 4, 3];
    const XY_DIRECTION: [(i64, i64); 4] = [(0, 1), (0, -1), (-1, 0), (1, 0)];
    let mut pos = pos;

    for d in [1, 2, 3, 4].iter().filter(|d| {
        // Don't go back from where we came
        if let Some(previous) = previous {
            *d != &OPPOSITE_DIRECTION[(previous - 1) as usize]
        } else {
            true
        }
    }) {
        input_tx.send(*d).unwrap();
        match output_rx.recv().unwrap() {
            0 => {
                // hit wall, next direction
                map.insert(
                    (
                        pos.0 + XY_DIRECTION[(d - 1) as usize].0,
                        pos.1 + XY_DIRECTION[(d - 1) as usize].1,
                    ),
                    0,
                );
            }
            1 => {
                // moved, continue
                pos.0 += XY_DIRECTION[(d - 1) as usize].0;
                pos.1 += XY_DIRECTION[(d - 1) as usize].1;
                map.insert(pos, 1);

                let (steps, result, new_pos) = traverse(&input_tx, &output_rx, Some(d), map, pos);
                match result {
                    0 => {
                        // no way found, reverse step
                        input_tx.send(OPPOSITE_DIRECTION[(d - 1) as usize]).unwrap();
                        assert_eq!(output_rx.recv().unwrap(), 1);
                    }
                    2 => {
                        // oxygen system found, return
                        return (steps + 1, result, new_pos);
                    }
                    usc => panic!(format!("Unexpected status code '{}'", usc)),
                }
            }
            2 => {
                // oxygen system found
                pos.0 += XY_DIRECTION[(d - 1) as usize].0;
                pos.1 += XY_DIRECTION[(d - 1) as usize].1;
                map.insert(pos, 2);
                return (1, 2, pos);
            }
            usc => panic!(format!("Unexpected status code '{}'", usc)),
        }
    }
    // no way found, return
    (0, 0, pos)
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

    let (_, input_tx, output_rx, _) = intcode::compute_threaded(m.clone(), None, None, None);

    let mut map: Map = HashMap::new();
    map.insert((0, 0), 3);
    let (steps, _, _) = traverse(
        &input_tx.unwrap(),
        &output_rx.unwrap(),
        None,
        &mut map,
        (0, 0),
    );

    println!("part1: {:?}, ({}µs)", steps, s1.elapsed().as_micros());

    let s2 = Instant::now();

    for y in (-15..25).rev() {
        for x in (-30..25).rev() {
            print!(
                "{}",
                if let Some(point) = map.get(&(x, y)) {
                    match point {
                        0 => "█",
                        1 => ".",
                        2 => "o",
                        3 => "S",
                        usc => panic!(format!("Unexpected map code '{}'", usc)),
                    }
                } else {
                    "?"
                }
            );
        }
        println!("");
    }

    println!("part2: {}, ({}µs)", 0, s2.elapsed().as_micros());

    println!("time: {}µs", s1.elapsed().as_micros());
}
