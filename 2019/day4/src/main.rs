use std::env;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().take(3).collect();
    assert!(args.len() == 3);
    let from = args[1].parse::<u32>().unwrap();
    let to = args[2].parse::<u32>().unwrap();
    assert!(to >= 100_000);
    assert!(from >= 100_000);
    assert!(from <= 999_999);
    assert!(to <= 999_999);
    assert!(from < to);

    let s1 = Instant::now();

    let mut p1c = 0u32;
    let mut p2c = 0u32;
    for a in from / 100_000..to / 100_000 {
        for b in a..10 {
            for c in b..10 {
                for d in c..10 {
                    for e in d..10 {
                        for f in e..10 {
                            let number =
                                a * 100_000 + b * 10_000 + c * 1_000 + d * 100 + e * 10 + f;
                            if number >= from && number <= to {
                                if a == b || b == c || c == d || d == e || e == f {
                                    p1c += 1;
                                    if a == b && b != c
                                        || a != b && b == c && c != d
                                        || b != c && c == d && d != e
                                        || c != d && d == e && e != f
                                        || d != e && e == f
                                    {
                                        p2c += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    println!("Part1: {}", p1c);
    println!("Part2: {}", p2c);
    println!("Time: {}µs", s1.elapsed().as_micros());
}
