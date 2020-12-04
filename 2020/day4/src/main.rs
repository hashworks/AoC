use std::fs::read_to_string;
use std::ops::RangeInclusive;
use std::time::Instant;

fn is_number_in_range(v: &str, range: RangeInclusive<u16>) -> bool {
    v.parse().map_or(false, |n: u16| range.contains(&n))
}

fn is_number_in_range_suffixed(v: &str, range: RangeInclusive<u16>, suffix: &str) -> bool {
    v.strip_suffix(suffix)
        .map_or(false, |rest| is_number_in_range(rest, range))
}

fn validate_batch_data(batch_data: &String, passport_validator: &dyn Fn(&&str) -> bool) -> usize {
    batch_data.split("\n\n")
              .filter(passport_validator)
              .count()
}

fn part1_passport_validator(passport: &&str) -> bool {
    passport.split_whitespace()
            .map(|field| field.split(":").next())
            .filter(|&key| key != Some("cid"))
            .count() == 7
}

fn part2_passport_validator(passport: &&str) -> bool {
    passport.split_whitespace().filter(|field| {
        let mut kv = field.split(":");
        let k = kv.next().unwrap();
        let v = kv.next().unwrap();
        match k {
            "byr" => is_number_in_range(v,1920..=2002),
            "iyr" => is_number_in_range(v,2010..=2020),
            "eyr" => is_number_in_range(v,2020..=2030),
            "hgt" => is_number_in_range_suffixed(v, 150..=193, "cm") || is_number_in_range_suffixed(v, 59..=76, "in"),
            "hcl" => v.len() == 7 && v.strip_prefix("#").map_or(false, |r| r.chars().all(|c| c.is_ascii_hexdigit())),
            "ecl" => matches!(v, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"),
            "pid" => v.len() == 9 && v.chars().all(|c| c.is_numeric()),
            _ => false
        }
    }).count() == 7
}

fn main() {
    let s1 = Instant::now();

    let batch_data = read_to_string("input").unwrap();

    println!("part1: {} ({}µs)", validate_batch_data(&batch_data, &part1_passport_validator), s1.elapsed().as_micros());

    let s2 = Instant::now();

    println!("part2: {} ({}µs)", validate_batch_data(&batch_data, &part2_passport_validator), s2.elapsed().as_micros());

    println!("Time: {}µs", s1.elapsed().as_micros());
}