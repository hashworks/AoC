use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn part1(f: File, h: &dyn Fn((i32, i32), &(usize, i32), (usize, i32)) -> i32) -> i32 {
    let mut ld = std::i32::MAX;
    let mut ps: HashMap<(i32, i32), (usize, i32)> = HashMap::new();

    BufReader::new(f)
        .lines()
        .take(2)
        .map(|lr| lr.unwrap())
        .enumerate()
        .for_each(|l| {
            let mut sc = 0;

            let mut cp = (0i32, 0i32);
            l.1.split(',')
                .map(|dr| {
                    let mut chars = dr.chars();
                    let d = chars.next().unwrap();
                    let l = chars.collect::<String>().parse::<i32>().unwrap();
                    match d {
                        'U' => ((0, 1), (0, l)),
                        'D' => ((0, -1), (0, -l)),
                        'R' => ((1, 0), (l, 0)),
                        'L' => ((-1, 0), (-l, 0)),
                        _ => panic!("THE FAILURE, IT BURNS"),
                    }
                })
                .for_each(|(s, t)| {
                    let t = (cp.0 + t.0, cp.1 + t.1);
                    while cp != t {
                        sc += 1;
                        cp = (cp.0 + s.0, cp.1 + s.1);
                        if cp.0 != 0 && cp.1 != 0 {
                            if let Some(cs) = ps.get(&cp) {
                                if cs.0 != l.0 {
                                    let d = h(cp, cs, (l.0, sc));
                                    if d < ld {
                                        ld = d;
                                    }
                                }
                            } else {
                                ps.insert(cp, (l.0, sc));
                            }
                        }
                    }
                });
        });

    ld
}

fn main() {
    println!(
        "part1: {}",
        part1(File::open("./input").unwrap(), &|cp, _, _| cp.0.abs()
            + cp.1.abs())
    );
    println!(
        "part2: {}",
        part1(File::open("./input").unwrap(), &|_, cs, cs2| cs.1 + cs2.1)
    );
}
