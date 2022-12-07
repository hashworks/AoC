mod util;

use std::{collections::HashMap, error::Error, io::BufRead, path::PathBuf};
use util::{aoc::AoCDay, input::get_reader};

const ID: &str = "day07";
type Input = Vec<(PathBuf, usize)>;
type Output = usize;

struct Day {}

impl AoCDay<Input, Output> for Day {
    fn parse_input(&self, id: &str) -> Result<Input, Box<dyn Error>> {
        let reader = get_reader(id)?;

        let mut files = vec![];
        let mut current_path = PathBuf::from("/");

        for line in reader.lines().skip(2) {
            let line = line?;

            let command = &line[2..4];
            match command {
                "cd" => match &line[5..] {
                    "/" => {
                        current_path = PathBuf::from("/");
                    }
                    ".." => {
                        current_path = current_path.parent().unwrap().to_path_buf();
                    }
                    subdir => {
                        current_path = current_path.join(subdir);
                    }
                },
                "ls" => {}
                _ => {
                    if line[0..3] != *"dir" {
                        let (left, right) = line.split_once(' ').unwrap();
                        let size = left.parse::<usize>().unwrap();
                        let name = current_path.join(right);
                        files.push((name, size));
                    }
                }
            }
        }

        Ok(files)
    }

    fn part1(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        let dir_sizes = get_dir_sizes(input);

        Ok(dir_sizes
            .iter()
            .filter(|(_, size)| **size <= 100000)
            .map(|(_, size)| size)
            .sum())
    }

    fn part2(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        let required_space =
            30000000 - (70000000 - input.iter().map(|(_, size)| *size).sum::<usize>());
        let dir_sizes = get_dir_sizes(input);

        let mut available_for_deletion: Vec<_> = dir_sizes
            .iter()
            .filter(|(_, size)| **size >= required_space)
            .map(|(_, size)| size)
            .collect();

        available_for_deletion.sort();

        Ok(**available_for_deletion.first().unwrap())
    }
}

fn get_dir_sizes(input: &Vec<(PathBuf, usize)>) -> HashMap<String, usize> {
    let mut dir_sizes: HashMap<String, usize> = std::collections::HashMap::new();
    for (path, size) in input {
        for ancestor in path.parent().unwrap().ancestors() {
            let dir_size = dir_sizes
                .entry(ancestor.to_string_lossy().to_string())
                .or_insert(0);
            *dir_size += size;
        }
    }
    dir_sizes
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
            42
        );
    }

    #[test]
    fn test_solve_part2() {
        let day = Day {};
        assert_eq!(
            day.parse_and_solve_part2(format!("{}_test1", ID).as_str())
                .unwrap(),
            42
        );
    }
}

fn main() {
    Day {}.run(ID);
}
