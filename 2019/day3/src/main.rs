use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn part1_part2(f: File) -> (i32, i32) {
    let mut lowest_manhattan_distance = std::i32::MAX;
    let mut lowest_step_count_sum = std::i32::MAX;
    let mut position_map: HashMap<(i32, i32), (usize, i32)> = HashMap::new();

    BufReader::new(f)
        .lines()
        .take(2)
        .map(|lr| lr.unwrap())
        .enumerate()
        .for_each(|(current_wire_index, current_wire)| {
            let mut step_count = 0;
            let mut current_point = (0i32, 0i32);

            current_wire
                .split(',')
                .map(|direction| {
                    let mut chars = direction.chars();
                    let direction = chars.next().unwrap();
                    let length = chars.collect::<String>().parse::<i32>().unwrap();
                    match direction {
                        'U' => ((0, 1), (0, length)),
                        'D' => ((0, -1), (0, -length)),
                        'R' => ((1, 0), (length, 0)),
                        'L' => ((-1, 0), (-length, 0)),
                        _ => panic!("THE FAILURE, IT BURNS"),
                    }
                })
                .for_each(|((step_x, step_y), (target_x, target_y))| {
                    let target = (current_point.0 + target_x, current_point.1 + target_y);
                    while current_point != target {
                        step_count += 1;
                        current_point = (current_point.0 + step_x, current_point.1 + step_y);
                        if current_point.0 != 0 && current_point.1 != 0 {
                            if let Some((existing_wire_index, existing_wire_step_count)) =
                                position_map.get(&current_point)
                            {
                                if existing_wire_index != &current_wire_index {
                                    // intersection
                                    let md = current_point.0.abs() + current_point.1.abs();
                                    let step_count_sum = existing_wire_step_count + step_count;
                                    if md < lowest_manhattan_distance {
                                        lowest_manhattan_distance = md;
                                    }
                                    if step_count_sum < lowest_step_count_sum {
                                        lowest_step_count_sum = step_count_sum;
                                    }
                                }
                            } else {
                                position_map
                                    .insert(current_point, (current_wire_index, step_count));
                            }
                        }
                    }
                });
        });

    (lowest_manhattan_distance, lowest_step_count_sum)
}

fn main() {
    let s1 = Instant::now();

    let (lowest_manhattan_distance, lowest_step_count) =
        part1_part2(File::open("./input").unwrap());
    println!("part1: {}", lowest_manhattan_distance);
    println!("part2: {}", lowest_step_count);

    println!("Time: {}µs", s1.elapsed().as_micros());
}
