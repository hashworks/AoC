use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

fn parse(path: &str) -> Vec<Vec<Option<bool>>> {
    let mut file = File::open(path).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    buffer
        .split(|&c| c == b'\n')
        .map(|cs| {
            cs.iter()
                .map(|c| match c {
                    b'L' => Some(false),
                    b'.' => None,
                    _ => panic!("Unknown seat type '{}'", *c as char),
                })
                .collect::<Vec<Option<bool>>>()
        })
        .filter(|r| !r.is_empty())
        .collect()
}

fn part2_get_new_seat_state(
    rows: &Vec<Vec<Option<bool>>>,
    seat_occupied: &Option<bool>,
    row: usize,
    column: usize,
) -> (bool, Option<bool>) {
    if seat_occupied.is_none() {
        return (false, None);
    }
    let seat_occupied = seat_occupied.unwrap();
    let adjacent_seats_occupied = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ]
    .iter()
    .map(|(rd, cd)| {
        (
            match rd {
                -1 => row.checked_sub(1),
                0 => Some(row),
                1 => Some(row + 1),
                _ => panic!(),
            },
            match cd {
                -1 => column.checked_sub(1),
                0 => Some(column),
                1 => Some(column + 1),
                _ => panic!(),
            },
        )
    })
    .filter(|(rd, cd)| rd.is_some() && cd.is_some())
    .filter(|(r, c)| {
        if let Some(seats) = rows.get(r.unwrap()) {
            if let Some(Some(true)) = seats.get(c.unwrap()) {
                true
            } else {
                false
            }
        } else {
            false
        }
    });

    if seat_occupied {
        if adjacent_seats_occupied.take(4).count() == 4 {
            (true, Some(false))
        } else {
            (false, Some(true))
        }
    } else if adjacent_seats_occupied.take(1).count() == 0 {
        (true, Some(true))
    } else {
        (false, Some(false))
    }
}

fn part1_get_new_seat_state(
    rows: &Vec<Vec<Option<bool>>>,
    seat_occupied: &Option<bool>,
    row: usize,
    column: usize,
) -> (bool, Option<bool>) {
    if seat_occupied.is_none() {
        return (false, None);
    }
    let seat_occupied = seat_occupied.unwrap();
    let adjacent_seats_occupied = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ]
    .iter()
    .map(|(rd, cd)| {
        (
            match rd {
                -1 => row.checked_sub(1),
                0 => Some(row),
                1 => Some(row + 1),
                _ => panic!(),
            },
            match cd {
                -1 => column.checked_sub(1),
                0 => Some(column),
                1 => Some(column + 1),
                _ => panic!(),
            },
        )
    })
    .filter(|(rd, cd)| rd.is_some() && cd.is_some())
    .filter(|(r, c)| {
        if let Some(seats) = rows.get(r.unwrap()) {
            if let Some(Some(true)) = seats.get(c.unwrap()) {
                true
            } else {
                false
            }
        } else {
            false
        }
    });

    if seat_occupied {
        if adjacent_seats_occupied.take(4).count() == 4 {
            (true, Some(false))
        } else {
            (false, Some(true))
        }
    } else if adjacent_seats_occupied.take(1).count() == 0 {
        (true, Some(true))
    } else {
        (false, Some(false))
    }
}

fn count_final_seats(
    mut rows: Vec<Vec<Option<bool>>>,
    get_new_seat_state: &dyn Fn(
        &Vec<Vec<Option<bool>>>,
        &Option<bool>,
        usize,
        usize,
    ) -> (bool, Option<bool>),
) -> usize {
    let mut changes = true;

    while changes {
        changes = false;

        rows = rows
            .iter()
            .enumerate()
            .map(|(row, seats)| {
                seats
                    .iter()
                    .enumerate()
                    .map(|(column, seat_occupied)| {
                        get_new_seat_state(&rows, seat_occupied, row, column)
                    })
                    .map(|(changed, new_seat_state)| {
                        if changed {
                            changes = true;
                        }
                        new_seat_state
                    })
                    .collect::<Vec<Option<bool>>>()
            })
            .collect();
    }

    rows.iter()
        .map(|seats| seats.iter().filter(|&s| s == &Some(true)).count())
        .sum()
}

fn main() {
    let s1 = Instant::now();
    println!(
        "part1: {} ({}ms)",
        count_final_seats(parse("input"), &part1_get_new_seat_state),
        s1.elapsed().as_millis()
    );

    let s2 = Instant::now();

    println!(
        "part2: {} ({}ms)",
        count_final_seats(parse("input"), &part2_get_new_seat_state),
        s2.elapsed().as_millis()
    );

    println!("Time: {}ms", s1.elapsed().as_millis());
}

#[test]
fn test_part1() {
    assert_eq!(
        37,
        count_final_seats(parse("input_test"), &part1_get_new_seat_state)
    );
}

#[test]
fn test_part2() {
    assert_eq!(
        26,
        count_final_seats(parse("input_test"), &part2_get_new_seat_state)
    );
}
