mod util;

use std::{error::Error, io::BufRead};
use util::{aoc::AoCDay, input::get_reader};

const ID: &str = "day15";
type Input = Vec<Vec<u8>>;
type Output = usize;

struct Day {}

fn hash(chunk: &[u8]) -> usize {
    chunk
        .iter()
        .fold(0, |acc, c| (acc + *c as usize) * 17 % 256)
}

const EMPTY_BOX: Vec<(Vec<u8>, u8)> = vec![];

impl AoCDay<Input, Output> for Day {
    fn parse_input(&self, id: &str) -> Result<Input, Box<dyn Error>> {
        let lenses = get_reader(id)?.split(b',').collect::<Result<Vec<_>, _>>()?;
        Ok(lenses)
    }

    fn part1(&self, lenses: &Input) -> Result<Output, Box<dyn Error>> {
        Ok(lenses.iter().map(|lens| hash(lens)).sum())
    }

    fn part2(&self, lenses: &Input) -> Result<Output, Box<dyn Error>> {
        let mut boxes: [Vec<(Vec<u8>, u8)>; 256] = [EMPTY_BOX; 256];

        for lens in lenses {
            if lens.last() != Some(&b'-') {
                let box_specifier = lens[0..lens.len() - 2].to_owned();
                let focal_length = *lens[lens.len() - 1..lens.len()]
                    .first()
                    .ok_or("no focal length")?
                    - 48;
                let lens_box = boxes.get_mut(hash(&box_specifier)).ok_or("invalid box")?;
                if let Some(existing_lens) = lens_box
                    .iter()
                    .enumerate()
                    .find(|(_, (old_box_specifier, _))| old_box_specifier.eq(&box_specifier))
                    .map(|(i, _)| i)
                {
                    lens_box[existing_lens].1 = focal_length;
                } else {
                    lens_box.push((box_specifier, focal_length));
                }
            } else {
                let box_specifier = &lens[0..lens.len() - 1];
                let lens_box = boxes.get_mut(hash(box_specifier)).ok_or("invalid box")?;
                if let Some(existing_lens) = lens_box
                    .iter()
                    .enumerate()
                    .find(|(_, (old_box_specifier, _))| old_box_specifier.eq(&box_specifier))
                    .map(|(i, _)| i)
                {
                    lens_box.remove(existing_lens);
                }
            }
        }

        Ok(boxes
            .iter()
            .enumerate()
            .map(|(i, lens_box)| {
                lens_box
                    .iter()
                    .enumerate()
                    .map(|(j, (_, focal_length))| (i + 1) * (j + 1) * *focal_length as usize)
                    .sum::<usize>()
            })
            .sum())
    }
}

fn main() {
    Day {}.run(ID);
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
            1320
        );
    }

    #[test]
    fn test_solve_part2() {
        let day = Day {};
        assert_eq!(
            day.parse_and_solve_part2(format!("{}_test1", ID).as_str())
                .unwrap(),
            145
        );
    }
}
