mod util;

use std::{collections::HashMap, error::Error, io::BufRead};
use util::{aoc::AoCDay, input::get_reader};

const ID: &str = "day07";
type Input = Vec<(Vec<String>, usize)>;
type Output = usize;

struct Day {}

impl AoCDay<Input, Output> for Day {
    fn parse_input(&self, id: &str) -> Result<Input, Box<dyn Error>> {
        let reader = get_reader(id)?;

        let mut files = vec![];
        let mut current_path = vec![];

        for line in reader.lines() {
            let line = line?;

            let command = &line[2..4];
            match command {
                "cd" => match &line[5..] {
                    "/" => {
                        current_path.clear();
                    }
                    ".." => {
                        current_path.pop();
                    }
                    subdir => {
                        current_path.push(subdir.to_string());
                    }
                },
                "ls" => {}
                _ => {
                    if line[0..3] != *"dir" {
                        let (size_str, filename) =
                            line.split_once(' ').ok_or("bad input: bad ls")?;
                        let mut path = current_path.clone();
                        path.push(filename.to_string());
                        files.push((path, size_str.parse()?));
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
            .filter(|&(_, &size)| size <= 100000)
            .map(|(_, size)| size)
            .sum())
    }

    fn part2(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        let required_space =
            30000000 - (70000000 - input.iter().map(|(_, size)| *size).sum::<usize>());
        let dir_sizes = get_dir_sizes(input);

        dir_sizes
            .iter()
            .filter(|&(_, &size)| size >= required_space)
            .map(|(_, size)| size)
            .min()
            .ok_or("no removable dir found".into())
            .copied()
    }
}

fn get_dir_sizes(input: &Input) -> HashMap<String, usize> {
    let mut dir_sizes = HashMap::new();
    for (path_parts, size) in input {
        for i in 0..path_parts.len() {
            let dir_size = dir_sizes.entry(path_parts[0..i].join("/")).or_insert(0);
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
            95437
        );
    }

    #[test]
    fn test_solve_part2() {
        let day = Day {};
        assert_eq!(
            day.parse_and_solve_part2(format!("{}_test1", ID).as_str())
                .unwrap(),
            24933642
        );
    }
}

fn main() {
    Day {}.run(ID);
}
