extern crate num_integer;

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

#[derive(Clone)]
struct AsteroidField {
    asteroids: HashSet<(i64, i64)>,
    field_x_max: i64,
    field_y_max: i64,
}

impl AsteroidField {
    fn remove_shadowed_from_b_by_a(&mut self, a: (i64, i64), b: (i64, i64)) {
        let m = b.0 - a.0;
        let n = b.1 - a.1;
        let mn_gcd = num_integer::gcd(m, n);
        let m = m / mn_gcd;
        let n = n / mn_gcd;
        (1..std::cmp::max(self.field_x_max, self.field_y_max))
            .map(|t| (b.0 + m * t, b.1 + n * t))
            .for_each(|(x, y)| {
                if x >= 0 && x <= self.field_x_max && y >= 0 && y <= self.field_y_max {
                    self.asteroids.remove(&(x, y));
                }
            });
    }
}

fn create_asteroid_field_from_file(path: &str) -> AsteroidField {
    let mut asteroids: HashSet<(i64, i64)> = HashSet::new();

    let mut field_x_max = 0;
    let mut field_y_max = 0;

    BufReader::new(File::open(path).unwrap())
        .lines()
        .enumerate()
        .for_each(|(y, l)| {
            l.unwrap()
                .chars()
                .enumerate()
                .filter(|(_, c)| c == &'#')
                .for_each(|(x, _)| {
                    let x = x as i64;
                    let y = y as i64;
                    asteroids.insert((x, y));
                    if x > field_x_max {
                        field_x_max = x;
                    }
                    if y > field_y_max {
                        field_y_max = y;
                    }
                });
        });

    AsteroidField {
        asteroids,
        field_x_max,
        field_y_max,
    }
}

fn main() {
    let s1 = Instant::now();

    let af = create_asteroid_field_from_file("./input");

    let mut monitoring_station = (0, 0);
    let mut monitoring_station_sees: Vec<((i64, i64), i64)> = vec![];
    for a in af.clone().asteroids {
        let mut af_from_a = af.clone();
        af_from_a.asteroids.remove(&a);
        for b in af_from_a.clone().asteroids {
            af_from_a.remove_shadowed_from_b_by_a(a, b);
        }
        if af_from_a.asteroids.len() > monitoring_station_sees.len() {
            monitoring_station_sees = af_from_a
                .asteroids
                .into_iter()
                .map(|pos| (pos, 0))
                .collect();
            monitoring_station = a;
        }
    }

    println!(
        "part1: {:?} sees {} asteroids, ({}µs)",
        monitoring_station,
        monitoring_station_sees.len(),
        s1.elapsed().as_micros()
    );

    let s2 = Instant::now();

    assert!(monitoring_station_sees.len() >= 200);

    let mut i = 0;
    while i < monitoring_station_sees.len() - 1 {
        let mut angle = ((monitoring_station.1 - (monitoring_station_sees[i].0).1) as f64)
            .atan2((monitoring_station.0 - (monitoring_station_sees[i].0).0) as f64);
        if angle < std::f64::consts::PI / 2.0 {
            angle += std::f64::consts::PI * 1.5;
        }
        monitoring_station_sees[i].1 = (angle * 1000.0) as i64;
        i += 1;
    }
    monitoring_station_sees.sort_by(|a, b| a.1.cmp(&b.1));

    println!(
        "part2: {}, ({}µs)",
        (monitoring_station_sees[199].0).0 * 100 + (monitoring_station_sees[199].0).1,
        s2.elapsed().as_micros()
    );

    println!("time: {}µs", s1.elapsed().as_micros());
}
