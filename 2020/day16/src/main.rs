use std::fs::read_to_string;
use std::ops::RangeInclusive;
use std::time::Instant;

fn parse_ticket_line(ticket_line: &str) -> Vec<u32> {
    ticket_line
        .split(',')
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

fn parse(
    ticket_data: String,
) -> (
    Vec<(String, RangeInclusive<u32>, RangeInclusive<u32>)>,
    Vec<u32>,
    Vec<Vec<u32>>,
) {
    let mut parts = ticket_data.split("\n\n");
    let ranges = parts
        .next()
        .unwrap()
        .lines()
        .map(|range_line| {
            let mut split = range_line.split(": ");
            let key = split.next().unwrap();
            let mut ranges = split.next().unwrap().split(" or ").map(|range_string| {
                let mut range_split = range_string
                    .split('-')
                    .map(str::parse::<u32>)
                    .map(Result::unwrap);
                let from = range_split.next().unwrap();
                let to = range_split.next().unwrap();
                from..=to
            });
            (
                key.to_string(),
                ranges.next().unwrap(),
                ranges.next().unwrap(),
            )
        })
        .collect();
    let my_ticket = parse_ticket_line(parts.next().unwrap().lines().skip(1).next().unwrap());
    let nearby_tickets: Vec<Vec<u32>> = parts
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(parse_ticket_line)
        .collect();
    (ranges, my_ticket, nearby_tickets)
}

fn part1(
    ranges: &Vec<(String, RangeInclusive<u32>, RangeInclusive<u32>)>,
    nearby_tickets: &Vec<Vec<u32>>,
) -> u32 {
    nearby_tickets
        .iter()
        .map(|ticket_numbers| {
            ticket_numbers
                .iter()
                .map(|ticket_number| {
                    for (_, range_a, range_b) in ranges {
                        if range_a.contains(ticket_number) || range_b.contains(ticket_number) {
                            return 0;
                        }
                    }
                    return *ticket_number;
                })
                .sum::<u32>()
        })
        .sum()
}

fn part2(
    ranges: &Vec<(String, RangeInclusive<u32>, RangeInclusive<u32>)>,
    my_ticket: &Vec<u32>,
    nearby_tickets: &Vec<Vec<u32>>,
) -> u64 {
    let nearby_tickets: Vec<&Vec<u32>> = nearby_tickets
        .iter()
        .filter(|ticket_numbers| {
            ticket_numbers.iter().all(|ticket_number| {
                for (_, range_a, range_b) in ranges {
                    if range_a.contains(ticket_number) || range_b.contains(ticket_number) {
                        return true;
                    }
                }
                return false;
            })
        })
        .collect();

    let mut fields: Vec<(usize, Vec<usize>)> = Vec::with_capacity(ranges.len());

    for row in 0..ranges.len() {
        let (_, range_a, range_b) = &ranges[row];
        let mut valid_columns = vec![];
        for column in 0..nearby_tickets[0].len() {
            if nearby_tickets
                .iter()
                .map(|tcs| tcs[column])
                .all(|tn| range_a.contains(&tn) || range_b.contains(&tn))
            {
                valid_columns.push(column);
            }
        }
        fields.push((row, valid_columns));
    }

    fields.sort_by(|a, b| a.1.len().cmp(&b.1.len()));

    let mut departure_product = 1;
    let mut columns_done: Vec<usize> = vec![];
    for (range_row, columns) in fields {
        let column = *columns
            .iter()
            .filter(|c| !columns_done.contains(c))
            .next()
            .unwrap();
        columns_done.push(column.clone());
        if ranges[range_row].0.starts_with("departure") {
            departure_product *= my_ticket[column] as u64;
        }
    }

    departure_product
}

fn main() {
    let s1 = Instant::now();

    let (range_map, my_ticket, nearby_tickets) = parse(read_to_string("input").unwrap());

    println!("parse time: {}µs", s1.elapsed().as_micros());

    println!(
        "part1: {} ({}µs)",
        part1(&range_map, &nearby_tickets),
        s1.elapsed().as_micros()
    );

    let s2 = Instant::now();

    println!(
        "part2: {} ({}µs)",
        part2(&range_map, &my_ticket, &nearby_tickets),
        s2.elapsed().as_micros()
    );

    println!("Time: {}µs", s1.elapsed().as_micros());
}

#[test]
fn test_part1() {
    let (range_map, _, nearby_tickets) = parse(
        "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12"
            .to_string(),
    );
    assert_eq!(71, part1(&range_map, &nearby_tickets));
}
