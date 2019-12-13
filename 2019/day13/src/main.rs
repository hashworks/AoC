// Note: Since the "intcode computer" should be reused later
// it is implemented in the crate `../intcode`.
extern crate intcode;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str;
use std::time::Instant;

fn main() {
    let s1 = Instant::now();

    let mut m = BufReader::new(File::open("./input").unwrap())
        .split(b',')
        .map(|ops| {
            str::from_utf8(&ops.unwrap())
                .unwrap()
                .trim()
                .parse::<i64>()
                .unwrap()
        })
        .collect::<Vec<i64>>();

    let (_, mut output) = intcode::compute(m.clone(), vec![]);

    let mut counter = 0;

    while let Some(tile_id) = output.pop() {
        if tile_id == 2 {
            counter += 1;
        }
        output.pop().unwrap();
        output.pop().unwrap();
    }

    println!("part1: {:?}, ({}µs)", counter, s1.elapsed().as_micros());

    let s2 = Instant::now();

    m[0] = 2;
    let (_, input_tx, output_rx, result_rx) = intcode::compute_threaded(m, None, None, None);
    let input_tx = input_tx.unwrap();
    let output_rx = output_rx.unwrap();
    let result_rx = result_rx.unwrap();

    let mut paddle_pos = None;
    let mut ball_pos = None;
    let mut score = 0;

    while result_rx.try_recv().is_err() {
        if let Ok(x) = output_rx.try_recv() {
            output_rx.recv().unwrap();
            let value = output_rx.recv().unwrap();
            if x == -1 {
                score = value;
            } else {
                match value {
                    0 | 1 | 2 => {}
                    3 => {
                        paddle_pos = Some(x);
                    }
                    4 => {
                        ball_pos = Some(x);
                    }
                    _ => panic!(format!("Unknown tile id '{}'", value)),
                }
            }
            if let Some(p_pos) = paddle_pos {
                if let Some(b_pos) = ball_pos {
                    input_tx
                        .send(if p_pos < b_pos {
                            1
                        } else if p_pos > b_pos {
                            -1
                        } else {
                            0
                        })
                        .unwrap();
                    ball_pos = None;
                }
            }
        }
    }
    println!("part2: {}, ({}µs)", score, s2.elapsed().as_micros());

    println!("time: {}µs", s1.elapsed().as_micros());
}
