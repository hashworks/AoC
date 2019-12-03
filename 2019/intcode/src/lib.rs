#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_tests() {
        for (m, n, v, r) in vec![
            (
                &mut vec![1, 0, 0, 3, 2, 3, 11, 0, 99, 30, 40, 50],
                9,
                10,
                3500,
            ),
            (&mut vec![1, 0, 0, 0, 99], 0, 0, 2),
            (&mut vec![1, 0, 1, 4, 99, 5, 6, 0, 99], 1, 1, 30),
        ] {
            assert_eq!(c(m, n, v), r);
        }
    }
}

pub fn c(m: &mut Vec<usize>, noun: usize, verb: usize) -> usize {
    m[1] = noun;
    m[2] = verb;

    let mut i = 0;
    while i <= m.len() {
        let nv = 4;
        match m[i] {
            99 => break,
            1 => {
                let c = m[i + 3];
                m[c] = m[m[i + 1]] + m[m[i + 2]];
            }
            2 => {
                let c = m[i + 3];
                m[c] = m[m[i + 1]] * m[m[i + 2]];
            }
            _ => panic!("AAAAAH!"),
        }
        i += nv;
    }

    m[0]
}

pub fn bruteforce(m: Vec<usize>, expected_r: usize) -> Option<(usize, usize)> {
    for noun in 0..99 {
        for verb in 0..99 {
            let r = self::c(&mut m.clone(), noun, verb);
            if r == expected_r {
                return Some((noun, verb));
            }
        }
    }
    None
}
