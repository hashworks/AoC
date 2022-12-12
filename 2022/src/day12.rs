mod util;

use pathfinding::prelude::dijkstra;
use std::{error::Error, io::BufRead};
use util::{aoc::AoCDay, input::get_reader};

const ID: &str = "day12";
type Input = HeightMap;
type Output = usize;

#[derive(Default)]
struct HeightMap {
    tiles: Vec<u8>,
    height: usize,
    width: usize,
    start: usize,
    end: usize,
}

struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl HeightMap {
    #[allow(dead_code)]
    fn print(&self) {
        for y in 0..self.tiles.len() / self.width {
            for x in 0..self.width {
                print!(
                    "{}",
                    self.tiles[self.point2d_to_index(Point::new(x, y))] as char
                );
            }
            println!();
        }
    }

    fn point2d_to_index(&self, point: Point) -> usize {
        point.y * self.width + point.x
    }

    fn index_to_point2d(&self, idx: usize) -> Point {
        Point {
            x: idx % self.width,
            y: idx / self.width,
        }
    }

    fn valid_exit(&self, location_idx: usize, target_idx: usize) -> Option<usize> {
        let location = self.tiles[location_idx];
        let target = self.tiles[target_idx];

        if target <= location + 1 {
            Some(target_idx)
        } else {
            None
        }
    }

    fn get_available_exits(&self, location_idx: usize) -> Vec<(usize, usize)> {
        let l = self.index_to_point2d(location_idx);
        let mut exits = Vec::with_capacity(4);

        if l.x > 0 {
            if let Some(idx) = self.valid_exit(
                location_idx,
                self.point2d_to_index(Point::new(l.x - 1, l.y)),
            ) {
                exits.push((idx, 1))
            }
        }
        if l.y > 0 {
            if let Some(idx) = self.valid_exit(
                location_idx,
                self.point2d_to_index(Point::new(l.x, l.y - 1)),
            ) {
                exits.push((idx, 1))
            }
        }
        if l.x < self.width - 1 {
            if let Some(idx) = self.valid_exit(
                location_idx,
                self.point2d_to_index(Point::new(l.x + 1, l.y)),
            ) {
                exits.push((idx, 1))
            }
        }
        if l.y < self.height - 1 {
            if let Some(idx) = self.valid_exit(
                location_idx,
                self.point2d_to_index(Point::new(l.x, l.y + 1)),
            ) {
                exits.push((idx, 1))
            }
        }

        exits
    }
}

struct Day {}

impl AoCDay<Input, Output> for Day {
    fn parse_input(&self, id: &str) -> Result<Input, Box<dyn Error>> {
        let reader = get_reader(id)?;

        let mut map = HeightMap::default();

        for (y, row) in reader.lines().enumerate() {
            let line = row?;
            map.height = y + 1;
            map.width = line.len();
            map.tiles.reserve(map.width);
            for (x, column) in line.bytes().enumerate() {
                if column == b'S' {
                    map.start = map.point2d_to_index(Point::new(x, y));
                    map.tiles.push(b'a');
                } else if column == b'E' {
                    map.end = map.point2d_to_index(Point::new(x, y));
                    map.tiles.push(b'z');
                } else {
                    map.tiles.push(column)
                }
            }
        }

        Ok(map)
    }

    fn part1(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        if let Some((_, path_len)) = dijkstra(
            &input.start,
            |idx| input.get_available_exits(*idx),
            |&idx| idx == input.end,
        ) {
            Ok(path_len)
        } else {
            Err("No path found".into())
        }
    }

    fn part2(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        input
            .tiles
            .iter()
            .enumerate()
            .filter(|&(_, &tile)| tile == b'a')
            .filter_map(|(idx, _)| {
                dijkstra(
                    &idx,
                    |idx| input.get_available_exits(*idx),
                    |&idx| idx == input.end,
                )
                .map(|(_, path_cost)| path_cost)
            })
            .min()
            .ok_or("No path found".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        let day = Day {};
        assert_eq!(
            day.parse_and_solve_part1(format!("{}_test1", ID).as_str())
                .unwrap(),
            31
        );
    }

    #[test]
    fn test_solve_part2() {
        let day = Day {};
        assert_eq!(
            day.parse_and_solve_part2(format!("{}_test1", ID).as_str())
                .unwrap(),
            29
        );
    }
}

fn main() {
    Day {}.run(ID);
}
