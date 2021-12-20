use core::fmt;
use std::fmt::{Display, Formatter};

pub struct GameOfTrench {
    algorithm: [bool; 512],
    map: Vec<Vec<bool>>,
    step_state: StepState,
}

enum StepState {
    Even,
    Odd,
}

impl From<Vec<String>> for GameOfTrench {
    fn from(input: Vec<String>) -> Self {
        let mut algorithm = [false; 512];
        if let Some(algo_str) = input.get(0) {
            let mut algo_iter = algo_str.chars();
            for i in 0..512 {
                algorithm[i] = algo_iter.next().unwrap_or('.') == '#';
            }
        }
        GameOfTrench {
            algorithm,
            map: input
                .iter()
                .skip(2)
                .map(|line| line.chars().map(|c| c == '#').collect())
                .collect(),
            step_state: StepState::Even,
        }
    }
}

impl Display for GameOfTrench {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for line in &self.map {
            for c in line {
                write!(f, "{}", if *c { '#' } else { '.' })?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl GameOfTrench {
    fn next_step_state(&mut self) {
        self.step_state = match self.step_state {
            StepState::Even => StepState::Odd,
            StepState::Odd => StepState::Even,
        };
    }

    fn get_new_pixel_state(&self, x: usize, y: usize) -> bool {
        let x = x as i64;
        let y = y as i64;
        let algo_pos: usize = [
            (x + 1, y + 1),
            (x, y + 1),
            (x - 1, y + 1),
            (x + 1, y),
            (x, y),
            (x - 1, y),
            (x + 1, y - 1),
            (x, y - 1),
            (x - 1, y - 1),
        ]
        .iter()
        .enumerate()
        .map(|(i, (x_of_nine, y_of_nine))| {
            let state = self
                .map
                .get(*y_of_nine as usize)
                .and_then(|row| row.get(*x_of_nine as usize));

            match (state, &self.step_state) {
                (Some(true), _) => (1 << i),
                (None, StepState::Odd) if self.algorithm[0] && !self.algorithm[511] => (1 << i),
                _ => 0,
            }
        })
        .sum();

        self.algorithm[algo_pos]
    }

    pub fn play(&mut self, steps: usize) {
        let target_size = self.map.len() + steps * 2;
        let mut target_map = vec![vec![false; target_size]; target_size];
        for (i, row) in self.map.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                target_map[i + steps][j + steps] = cell;
            }
        }
        self.map = target_map;

        for _ in 0..steps {
            let mut new_map = vec![vec![false; target_size]; target_size];
            for y in 0..target_size {
                for x in 0..target_size {
                    new_map[y][x] = self.get_new_pixel_state(x, y);
                }
            }
            self.map = new_map;

            self.next_step_state();
        }
    }

    pub fn count_pixels(&self) -> usize {
        self.map.iter().flatten().filter(|&&c| c).count()
    }
}
