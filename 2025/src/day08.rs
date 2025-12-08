mod util;

use std::{collections::HashSet, error::Error, io::BufRead};
use util::{aoc::AoCDay, input::get_reader};

const ID: &str = "day08";
type Input = Vec<[usize; 3]>;
type Output = usize;

struct Day {}

impl AoCDay<Input, Output> for Day {
    fn parse_input(&self, id: &str) -> Result<Input, Box<dyn Error>> {
        get_reader(id)?
            .lines()
            .map(|l| {
                let l = l?;
                let mut split = l.splitn(3, ',');
                let x = split.next().ok_or("Invalid input: missing x")?.parse()?;
                let y = split.next().ok_or("Invalid input: missing y")?.parse()?;
                let z = split.next().ok_or("Invalid input: missing z")?.parse()?;
                Ok([x, y, z])
            })
            .collect()
    }

    fn part1(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        let limit = match input.len() {
            20 => 10, // Test mode
            _ => 1000,
        };

        let jb_distances = measure_distances(input);

        let mut circuits: Vec<HashSet<&[usize; 3]>> = vec![];

        for (_, jb_a, jb_b) in jb_distances.iter().take(limit) {
            connect(&mut circuits, jb_a, jb_b)?;
        }

        circuits.sort_by(|s1, s2| s2.len().cmp(&s1.len()));

        Ok(circuits.iter().take(3).map(|s| s.len()).product())
    }

    fn part2(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        let jb_distances = measure_distances(input);

        let mut circuits: Vec<HashSet<&[usize; 3]>> = vec![];

        for (_, jb_a, jb_b) in jb_distances.iter().cycle() {
            connect(&mut circuits, jb_a, jb_b)?;

            if let Some(circuit) = circuits.get(0) {
                if circuit.len() == input.len() {
                    return Ok(jb_a[0] * jb_b[0]);
                }
            }
        }
        Err("Unexpected loop exit".into())
    }
}

fn connect<'a>(
    circuits: &mut Vec<HashSet<&'a [usize; 3]>>,
    jb_a: &'a [usize; 3],
    jb_b: &'a [usize; 3],
) -> Result<(), Box<dyn Error + 'static>> {
    let jb_a_i = circuits
        .iter()
        .enumerate()
        .find(|(_, circuit)| circuit.contains(jb_a))
        .map(|(jb_a_i, _)| jb_a_i);
    let jb_b_j = circuits
        .iter()
        .enumerate()
        .find(|(_, circuit)| circuit.contains(jb_b))
        .map(|(jb_b_j, _)| jb_b_j);

    Ok(match (jb_a_i, jb_b_j) {
        (Some(i), Some(j)) => {
            if i != j {
                let set_to_merge = circuits.remove(i.max(j));
                circuits[i.min(j)].extend(set_to_merge);
            }
        }
        (Some(i), None) => {
            circuits
                .get_mut(i)
                .ok_or("Failed to access circuit contains_jb_a_i")?
                .insert(jb_b);
        }
        (None, Some(j)) => {
            circuits
                .get_mut(j)
                .ok_or("Failed to access circuit contains_jb_b_j")?
                .insert(jb_a);
        }
        (None, None) => circuits.push(HashSet::from([jb_a, jb_b])),
    })
}

fn measure_distances(input: &Vec<[usize; 3]>) -> Vec<(f64, &[usize; 3], &[usize; 3])> {
    let mut jb_distances = input
        .iter()
        .enumerate()
        .flat_map(|(i, jb)| {
            input
                .iter()
                .skip(i)
                .filter(move |&other| other != jb)
                .map(move |other| (euclidian_distance(jb, other), jb, other))
        })
        .collect::<Vec<_>>();

    jb_distances.sort_by(|(d1, _, _), (d2, _, _)| d1.total_cmp(d2));
    jb_distances
}

fn euclidian_distance(p: &[usize; 3], q: &[usize; 3]) -> f64 {
    let xd = (p[0] as isize - q[0] as isize).pow(2);
    let yd = (p[1] as isize - q[1] as isize).pow(2);
    let zd = (p[2] as isize - q[2] as isize).pow(2);
    ((xd + yd + zd) as f64).sqrt()
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
            40
        );
    }

    #[test]
    fn test_solve_part2() {
        let day = Day {};
        assert_eq!(
            day.parse_and_solve_part2(format!("{}_test1", ID).as_str())
                .unwrap(),
            25272
        );
    }
}
