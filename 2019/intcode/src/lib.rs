use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::thread::JoinHandle;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day2_part1_tests() {
        for (mut memory, noun, verb, expected_result) in vec![
            (vec![1, 0, 0, 3, 2, 3, 11, 0, 99, 30, 40, 50], 9, 10, 3500),
            (vec![1, 0, 0, 0, 99], 0, 0, 2),
            (vec![1, 0, 1, 4, 99, 5, 6, 0, 99], 1, 1, 30),
        ] {
            memory[1] = noun;
            memory[2] = verb;
            let (m, _) = compute(memory, vec![]);
            assert_eq!(m[0], expected_result);
        }
    }
    #[test]
    fn day2_part1_tests_threaded() {
        for (mut memory, noun, verb, expected_result) in vec![
            (vec![1, 0, 0, 3, 2, 3, 11, 0, 99, 30, 40, 50], 9, 10, 3500),
            (vec![1, 0, 0, 0, 99], 0, 0, 2),
            (vec![1, 0, 1, 4, 99, 5, 6, 0, 99], 1, 1, 30),
        ] {
            memory[1] = noun;
            memory[2] = verb;
            let (_, _, _, return_rx) = compute_threaded(memory, None, None, None);

            let (m, _, _) = return_rx.unwrap().recv().unwrap();
            assert_eq!(m[0], expected_result);
        }
    }

    #[test]
    fn day5_part2_tests() {
        for (memory, input, expected_output) in vec![
            (vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], 8, 1),
            (vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], 9, 0),
            (vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], 7, 1),
            (vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], 8, 0),
            (vec![3, 3, 1108, -1, 8, 3, 4, 3, 99], 8, 1),
            (vec![3, 3, 1108, -1, 8, 3, 4, 3, 99], 9, 0),
            (vec![3, 3, 1107, -1, 8, 3, 4, 3, 99], 7, 1),
            (vec![3, 3, 1107, -1, 8, 3, 4, 3, 99], 8, 0),
        ] {
            let (_, output) = compute(memory, vec![input]);
            assert_eq!(output[0], expected_output);
        }
    }

    #[test]
    fn day5_part2_tests_threaded() {
        for (memory, input, expected_output) in vec![
            (vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], 8, 1),
            (vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], 9, 0),
            (vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], 7, 1),
            (vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], 8, 0),
            (vec![3, 3, 1108, -1, 8, 3, 4, 3, 99], 8, 1),
            (vec![3, 3, 1108, -1, 8, 3, 4, 3, 99], 9, 0),
            (vec![3, 3, 1107, -1, 8, 3, 4, 3, 99], 7, 1),
            (vec![3, 3, 1107, -1, 8, 3, 4, 3, 99], 8, 0),
        ] {
            if let (_, Some(input_tx), Some(output_rx), _) =
                compute_threaded(memory, None, None, None)
            {
                input_tx.send(input).unwrap();
                assert_eq!(output_rx.recv().unwrap(), expected_output);
            }
        }
    }

    #[test]
    fn intcode_parser_tests() {
        for (intcode, expected_result) in vec![
            (00099, (99, 0, 0, 0)),
            (00101, (1, 1, 0, 0)),
            (01102, (2, 1, 1, 0)),
            (10003, (3, 0, 0, 1)),
            (10104, (4, 1, 0, 1)),
            (11105, (5, 1, 1, 1)),
        ] {
            assert_eq!(intcode_parser(intcode), expected_result);
        }
    }
}

fn intcode_parser(intcode: i32) -> (i32, i32, i32, i32) {
    assert!(intcode >= 0);
    assert!(intcode < 100000);
    let mut instruction = intcode;

    let parameter_mode_3 = instruction / 10000;
    instruction -= parameter_mode_3 * 10000;
    let parameter_mode_2 = instruction / 1000;
    instruction -= parameter_mode_2 * 1000;
    let parameter_mode_1 = instruction / 100;
    instruction -= parameter_mode_1 * 100;

    (
        instruction,
        parameter_mode_1,
        parameter_mode_2,
        parameter_mode_3,
    )
}

// get_parameter_value
fn gpv(m: &Vec<i32>, memory_cell: i32, parameter_mode: i32) -> i32 {
    match parameter_mode {
        0 => m[memory_cell as usize],
        1 => memory_cell,
        pm => panic!(format!("unknown parameter mode '{}'", pm)),
    }
}

fn c_internal(
    m: Vec<i32>,
    get_input: &dyn Fn(usize) -> i32,
    post_output: &dyn Fn(&mut Vec<i32>, i32),
) -> (Vec<i32>, Vec<i32>) {
    let mut m = m;

    let mut input_index = 0;
    let mut outputs = vec![];

    let mut i = 0;
    while i <= m.len() {
        let (ins, pm1, pm2, _) = intcode_parser(m[i]);
        match ins {
            99 => break,
            1 => {
                // add
                let target = m[i + 3] as usize;
                m[target] = gpv(&m, m[i + 1], pm1) + gpv(&m, m[i + 2], pm2);
                i += 4;
            }
            2 => {
                // mul
                let target = m[i + 3] as usize;
                m[target] = gpv(&m, m[i + 1], pm1) * gpv(&m, m[i + 2], pm2);
                i += 4;
            }
            3 => {
                // input
                let target = m[i + 1] as usize;
                m[target] = get_input(input_index);
                input_index += 1;
                i += 2;
            }
            4 => {
                // output
                post_output(&mut outputs, gpv(&m, m[i + 1], pm1));
                i += 2;
            }
            5 => {
                // jump-if-true
                if gpv(&m, m[i + 1], pm1) != 0 {
                    i = gpv(&m, m[i + 2], pm2) as usize;
                } else {
                    i += 3;
                }
            }
            6 => {
                // jump-if-false
                if gpv(&m, m[i + 1], pm1) == 0 {
                    i = gpv(&m, m[i + 2], pm2) as usize;
                } else {
                    i += 3;
                }
            }
            7 => {
                // less than
                let target = m[i + 3] as usize;
                m[target] = if gpv(&m, m[i + 1], pm1) < gpv(&m, m[i + 2], pm2) {
                    1
                } else {
                    0
                };
                i += 4;
            }
            8 => {
                // equals
                let target = m[i + 3] as usize;
                m[target] = if gpv(&m, m[i + 1], pm1) == gpv(&m, m[i + 2], pm2) {
                    1
                } else {
                    0
                };
                i += 4;
            }
            ins => panic!(format!("unknown instruction '{}'", ins)),
        }
    }
    (m, outputs)
}

pub fn compute(m: Vec<i32>, inputs: Vec<i32>) -> (Vec<i32>, Vec<i32>) {
    let (m, outputs) = c_internal(m, &|i| inputs[i], &|outputs, o| outputs.push(o));

    (m, outputs)
}

pub fn compute_threaded(
    m: Vec<i32>,
    input_rx: Option<Receiver<i32>>,
    output_tx: Option<Sender<i32>>,
    return_tx: Option<Sender<(Vec<i32>, Receiver<i32>, Sender<i32>)>>,
) -> (
    JoinHandle<()>,
    Option<Sender<i32>>,
    Option<Receiver<i32>>,
    Option<Receiver<(Vec<i32>, Receiver<i32>, Sender<i32>)>>,
) {
    let (maybe_input_tx, input_rx) = if let Some(input_rx) = input_rx {
        (None, input_rx)
    } else {
        let (input_tx, input_rx) = mpsc::channel::<i32>();
        (Some(input_tx), input_rx)
    };
    let (output_tx, maybe_output_rx) = if let Some(output_tx) = output_tx {
        (output_tx, None)
    } else {
        let (output_tx, output_rx) = mpsc::channel::<i32>();
        (output_tx, Some(output_rx))
    };
    let (return_tx, maybe_return_rx) = if let Some(return_tx) = return_tx {
        (return_tx, None)
    } else {
        let (return_tx, return_rx) = mpsc::channel::<(Vec<i32>, Receiver<i32>, Sender<i32>)>();
        (return_tx, Some(return_rx))
    };
    (
        thread::spawn(move || {
            let (m, _) = c_internal(
                m,
                &|_| {
                    input_rx
                        .recv()
                        .expect("Tried to receive an input but it failed")
                },
                &|_, o| {
                    output_tx
                        .send(o)
                        .expect("Tried to send an output but it failed")
                },
            );
            match return_tx.send((m, input_rx, output_tx)) {
                _ => {}
            }
        }),
        maybe_input_tx,
        maybe_output_rx,
        maybe_return_rx,
    )
}
