// Note: Since the "intcode computer" should be reused later
// it is implemented in the crate `../intcode`.
extern crate intcode;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str;
use std::sync::mpsc;
use std::time::Instant;

fn main() {
    let s1 = Instant::now();

    let mut m = BufReader::new(File::open("./input").unwrap())
        .split(b',')
        .map(|ops| {
            str::from_utf8(&ops.unwrap())
                .unwrap()
                .trim()
                .parse::<i32>()
                .unwrap()
        })
        .collect::<Vec<i32>>();

    let mut max = 0;

    for abcde in (0..5).permutations(5) {
        let (nm, a_out) = intcode::compute(m, vec![abcde[0], 0]);
        let (nm, b_out) = intcode::compute(nm, vec![abcde[1], a_out[0]]);
        let (nm, c_out) = intcode::compute(nm, vec![abcde[2], b_out[0]]);
        let (nm, d_out) = intcode::compute(nm, vec![abcde[3], c_out[0]]);
        let (nm, e_out) = intcode::compute(nm, vec![abcde[4], d_out[0]]);
        if e_out[0] > max {
            max = e_out[0];
        }
        m = nm;
    }

    println!("part1: {:?}, ({}µs)", max, s1.elapsed().as_micros());

    let s2 = Instant::now();

    for abcde in (5..10).permutations(5) {
        let (a_input_tx, a_input_rx) = mpsc::channel::<i32>();
        let (b_input_tx, b_input_rx) = mpsc::channel::<i32>();
        let (c_input_tx, c_input_rx) = mpsc::channel::<i32>();
        let (d_input_tx, d_input_rx) = mpsc::channel::<i32>();
        let (e_input_tx, e_input_rx) = mpsc::channel::<i32>();
        a_input_tx.send(abcde[0]).unwrap();
        a_input_tx.send(0).unwrap();
        b_input_tx.send(abcde[1]).unwrap();
        c_input_tx.send(abcde[2]).unwrap();
        d_input_tx.send(abcde[3]).unwrap();
        e_input_tx.send(abcde[4]).unwrap();

        let (_, _, _, result_rx) =
            intcode::compute_threaded(m.clone(), Some(a_input_rx), Some(b_input_tx), None);
        let (_, _, _, _) =
            intcode::compute_threaded(m.clone(), Some(b_input_rx), Some(c_input_tx), None);
        let (_, _, _, _) =
            intcode::compute_threaded(m.clone(), Some(c_input_rx), Some(d_input_tx), None);
        let (_, _, _, _) =
            intcode::compute_threaded(m.clone(), Some(d_input_rx), Some(e_input_tx), None);
        let (handle_e, _, _, _) =
            intcode::compute_threaded(m.clone(), Some(e_input_rx), Some(a_input_tx), None);

        handle_e.join().unwrap();

        let (_, a_input_rx, _) = result_rx.unwrap().recv().unwrap();

        let e_out = a_input_rx.recv().unwrap();

        if e_out > max {
            max = e_out;
        }
    }

    println!("part2: {:?}, ({}µs)", max, s2.elapsed().as_micros());

    println!("time: {}µs", s1.elapsed().as_micros());
}
