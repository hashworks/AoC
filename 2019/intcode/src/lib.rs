#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day2_part1_tests() {
        for (memory, noun, verb, expected_result) in vec![
            (
                &mut vec![1, 0, 0, 3, 2, 3, 11, 0, 99, 30, 40, 50],
                9,
                10,
                3500,
            ),
            (&mut vec![1, 0, 0, 0, 99], 0, 0, 2),
            (&mut vec![1, 0, 1, 4, 99, 5, 6, 0, 99], 1, 1, 30),
        ] {
            memory[1] = noun;
            memory[2] = verb;
            let (m0, _) = compute(memory, &mut vec![]);
            assert_eq!(m0, expected_result);
        }
    }

    #[test]
    fn day5_part2_tests() {
        for (memory, input, expected_output) in vec![
            (&mut vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], 8, 1),
            (&mut vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], 9, 0),
            (&mut vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], 7, 1),
            (&mut vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], 8, 0),
            (&mut vec![3, 3, 1108, -1, 8, 3, 4, 3, 99], 8, 1),
            (&mut vec![3, 3, 1108, -1, 8, 3, 4, 3, 99], 9, 0),
            (&mut vec![3, 3, 1107, -1, 8, 3, 4, 3, 99], 7, 1),
            (&mut vec![3, 3, 1107, -1, 8, 3, 4, 3, 99], 8, 0),
        ] {
            let (_, output) = compute(memory, &mut vec![input]);
            assert_eq!(output[0], expected_output);
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
        _ => panic!("unknown parameter mode"),
    }
}

pub fn compute(m: &mut Vec<i32>, input: &mut Vec<i32>) -> (i32, Vec<i32>) {
    let mut output = vec![];

    let mut i = 0;
    while i <= m.len() {
        let (ins, pm1, pm2, _) = intcode_parser(m[i]);

        match ins {
            99 => break,
            1 => {
                // add
                let target = m[i + 3] as usize;
                m[target] = gpv(m, m[i + 1], pm1) + gpv(m, m[i + 2], pm2);
                i += 4;
            }
            2 => {
                // mul
                let target = m[i + 3] as usize;
                m[target] = gpv(m, m[i + 1], pm1) * gpv(m, m[i + 2], pm2);
                i += 4;
            }
            3 => {
                // input
                let target = m[i + 1] as usize;
                m[target] = input.pop().unwrap();
                i += 2;
            }
            4 => {
                // output
                output.push(gpv(m, m[i + 1], pm1));
                i += 2;
            }
            5 => {
                // jump-if-true
                if gpv(m, m[i + 1], pm1) != 0 {
                    i = gpv(m, m[i + 2], pm2) as usize;
                } else {
                    i += 3;
                }
            }
            6 => {
                // jump-if-false
                if gpv(m, m[i + 1], pm1) == 0 {
                    i = gpv(m, m[i + 2], pm2) as usize;
                } else {
                    i += 3;
                }
            }
            7 => {
                // less than
                let target = m[i + 3] as usize;
                m[target] = if gpv(m, m[i + 1], pm1) < gpv(m, m[i + 2], pm2) {
                    1
                } else {
                    0
                };
                i += 4;
            }
            8 => {
                // equals
                let target = m[i + 3] as usize;
                m[target] = if gpv(m, m[i + 1], pm1) == gpv(m, m[i + 2], pm2) {
                    1
                } else {
                    0
                };
                i += 4;
            }
            _ => panic!("unknown instruction"),
        }
    }

    (m[0], output)
}
