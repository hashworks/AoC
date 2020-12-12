use std::fs::File;
use std::io::prelude::*;
use std::str::from_utf8;
use std::time::Instant;

enum Action {
    NORTH,
    EAST,
    SOUTH,
    WEST,
    LEFT,
    RIGHT,
    FORWARD,
}

fn match_action_line(action_line: &[u8]) -> (i32, Action) {
    let action = match &action_line[0] {
        b'N' => Action::NORTH,
        b'E' => Action::EAST,
        b'S' => Action::SOUTH,
        b'W' => Action::WEST,
        b'F' => Action::FORWARD,
        b'L' => Action::LEFT,
        b'R' => Action::RIGHT,
        _ => panic!("Unknown action type {:?}", action_line[0] as char),
    };
    (
        match action {
            Action::LEFT | Action::RIGHT => match &action_line[1..] {
                b"90" => 1,
                b"180" => 2,
                b"270" => 3,
                _ => panic!(
                    "Unknown direction {:?}",
                    from_utf8(&action_line[1..]).unwrap()
                ),
            },
            _ => from_utf8(&action_line[1..]).unwrap().parse().unwrap(),
        },
        action,
    )
}

fn part1(actions: &Vec<u8>) -> i32 {
    let move_in_direction = |d, td, x, y, v| match d {
        0 => (td, x + v, y),
        1 => (td, x, y + v),
        2 => (td, x - v, y),
        3 => (td, x, y - v),
        _ => panic!("Unknown direction {}", d),
    };

    let final_state = actions
        .split(|&c| c == b'\n')
        .filter(|line| !line.is_empty())
        .map(match_action_line)
        .fold((1, 0, 0), |(d, x, y), (value, action)| match action {
            Action::LEFT => ((((d - value) % 4) + 4) % 4, x, y),
            Action::RIGHT => ((d + value) % 4, x, y),
            Action::NORTH => move_in_direction(0, d, x, y, value),
            Action::EAST => move_in_direction(1, d, x, y, value),
            Action::SOUTH => move_in_direction(2, d, x, y, value),
            Action::WEST => move_in_direction(3, d, x, y, value),
            Action::FORWARD => move_in_direction(d, d, x, y, value),
        });
    (final_state.1 as i32).abs() + (final_state.2 as i32).abs()
}

fn part2(actions: &Vec<u8>) -> i32 {
    let final_state = actions
        .split(|&c| c == b'\n')
        .filter(|line| !line.is_empty())
        .map(match_action_line)
        .fold((0, 0, 1, 10), |(x, y, wx, wy), (value, action)| {
            match (action, value) {
                (Action::RIGHT, 1) => (x, y, -wy, wx),
                (Action::RIGHT, 2) => (x, y, -wx, -wy),
                (Action::RIGHT, 3) => (x, y, wy, -wx),
                (Action::LEFT, 3) => (x, y, -wy, wx),
                (Action::LEFT, 2) => (x, y, -wx, -wy),
                (Action::LEFT, 1) => (x, y, wy, -wx),
                (Action::NORTH, value) => (x, y, wx + value, wy),
                (Action::EAST, value) => (x, y, wx, wy + value),
                (Action::SOUTH, value) => (x, y, wx - value, wy),
                (Action::WEST, value) => (x, y, wx, wy - value),
                (Action::FORWARD, value) => (x + (wx * value), y + (wy * value), wx, wy),
                _ => panic!("Unknown value {}", value),
            }
        });
    (final_state.0 as i32).abs() + (final_state.1 as i32).abs()
}

fn main() {
    let s1 = Instant::now();

    let mut file = File::open("input").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    println!("part1: {} ({}µs)", part1(&buffer), s1.elapsed().as_micros());

    let s2 = Instant::now();

    println!("part2: {} ({}µs)", part2(&buffer), s2.elapsed().as_micros());

    println!("Time: {}µs", s1.elapsed().as_micros());
}

#[test]
fn test_part1() {
    assert_eq!(
        25,
        part1(
            &"F10
N3
F7
R90
F11"
            .bytes()
            .collect()
        )
    );
}

#[test]
fn test_part2() {
    assert_eq!(
        286,
        part2(
            &"F10
N3
F7
R90
F11"
            .bytes()
            .collect()
        )
    );
}

/*

#[test]
fn test_part2() {
    assert_eq!(
        26,
        count_final_seats(parse("input_test"), &part2_get_new_seat_state)
    );
}
*/
