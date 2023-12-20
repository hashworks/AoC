mod util;

use std::collections::{HashSet, VecDeque};
use std::fmt::Debug;
use std::{collections::HashMap, error::Error, io::BufRead};
use util::{aoc::AoCDay, input::get_reader};

const ID: &str = "day20";
type Input = HashMap<String, Module>;
type Output = usize;

struct Day {}

const BUTTON: &str = "button";
const BROADCASTER: &str = "broadcaster";
const RECEIVER: &str = "rx";
const EMPTY_OUTPUTS: (Pulse, &Vec<String>) = (Pulse::Low, &vec![]);

#[derive(Debug, Clone)]
enum Module {
    Broadcaster {
        inputs: HashMap<String, Pulse>,
        outputs: Vec<String>,
    },
    FlipFlop {
        state: State,
        inputs: HashMap<String, Pulse>,
        outputs: Vec<String>,
    },
    Conjunction {
        inputs: HashMap<String, Pulse>,
        outputs: Vec<String>,
    },
}

#[derive(Debug, Clone, Copy)]
enum State {
    On,
    Off,
}

#[derive(Debug, Clone, PartialEq)]
enum Pulse {
    High,
    Low,
}

impl Module {
    fn handle_pulse(&mut self, input: String, pulse: &Pulse) -> (Pulse, &Vec<String>) {
        match self {
            Module::Broadcaster { outputs, .. } => (pulse.clone(), outputs),
            Module::FlipFlop { state, outputs, .. } => match (pulse, *state) {
                (Pulse::Low, State::Off) => {
                    *state = State::On;
                    (Pulse::High, outputs)
                }
                (Pulse::Low, State::On) => {
                    *state = State::Off;
                    (Pulse::Low, outputs)
                }
                _ => EMPTY_OUTPUTS,
            },
            Module::Conjunction { inputs, outputs } => {
                if let Some(connected_input) = inputs.get_mut(&input) {
                    *connected_input = pulse.clone();
                }
                if inputs.values().all(|p| p == &Pulse::High) {
                    (Pulse::Low, outputs)
                } else {
                    (Pulse::High, outputs)
                }
            }
        }
    }

    fn set_inputs(&mut self, connected_inputs: Vec<String>) {
        match self {
            Module::Broadcaster { inputs, .. }
            | Module::FlipFlop { inputs, .. }
            | Module::Conjunction { inputs, .. } => {
                *inputs = connected_inputs
                    .iter()
                    .map(|s| (s.clone(), Pulse::Low))
                    .collect();
            }
        }
    }

    fn get_inputs(&self) -> HashSet<String> {
        match self {
            Module::Broadcaster { inputs, .. }
            | Module::FlipFlop { inputs, .. }
            | Module::Conjunction { inputs, .. } => inputs.keys().cloned().collect(),
        }
    }

    fn get_outputs(&self) -> HashSet<String> {
        match self {
            Module::Broadcaster { outputs, .. }
            | Module::FlipFlop { outputs, .. }
            | Module::Conjunction { outputs, .. } => outputs.iter().cloned().collect(),
        }
    }
}

impl AoCDay<Input, Output> for Day {
    fn parse_input(&self, id: &str) -> Result<Input, Box<dyn Error>> {
        let mut connections = vec![];

        let mut modules = get_reader(id)?
            .lines()
            .map(|l| {
                let l = l?;
                let (left, right) = l.split_once(" -> ").ok_or("Invalid input")?;
                let module_type = left.chars().next().ok_or("Invalid input")?;
                let outputs = right.split(", ").map(|s| s.to_string()).collect::<Vec<_>>();
                let module: (String, Module) = match module_type {
                    '%' => {
                        let name = left[1..].to_string();
                        connections.push((name.clone(), outputs.clone()));
                        (
                            name,
                            Module::FlipFlop {
                                inputs: HashMap::new(),
                                outputs,
                                state: State::Off,
                            },
                        )
                    }
                    '&' => {
                        let name = left[1..].to_string();
                        connections.push((name.clone(), outputs.clone()));
                        (
                            name,
                            Module::Conjunction {
                                inputs: HashMap::new(),
                                outputs,
                            },
                        )
                    }
                    _ => (
                        left.to_string(),
                        Module::Broadcaster {
                            inputs: HashMap::new(),
                            outputs,
                        },
                    ),
                };
                Ok(module)
            })
            .collect::<Result<Input, Box<dyn Error>>>()?;

        modules.iter_mut().for_each(|(name, module)| {
            let inputs = connections
                .iter()
                .filter(|(_, outputs)| outputs.contains(name))
                .map(|(input, _)| input.clone())
                .collect();
            module.set_inputs(inputs);
        });

        Ok(modules)
    }

    fn part1(&self, modules: &Input) -> Result<Output, Box<dyn Error>> {
        let mut modules = modules.clone();

        let mut high_pulses = 0;
        let mut low_pulses = 0;
        let mut stack = VecDeque::new();

        for _ in 0..1000 {
            stack.push_back((BUTTON.to_string(), BROADCASTER.to_string(), Pulse::Low));

            while let Some((input, output, pulse)) = stack.pop_front() {
                if pulse == Pulse::High {
                    high_pulses += 1;
                } else {
                    low_pulses += 1;
                }
                if let Some(module) = modules.get_mut(&output) {
                    let (new_pulse, outputs) = module.handle_pulse(input.clone(), &pulse);
                    stack.extend(
                        outputs
                            .iter()
                            .map(|s| (output.clone(), s.clone(), new_pulse.clone())),
                    );
                }
            }
        }

        Ok(high_pulses * low_pulses)
    }

    fn part2(&self, modules: &Input) -> Result<Output, Box<dyn Error>> {
        let mut counters = vec![];

        let rx_nand = modules
            .iter()
            .find(|(_, module)| module.get_outputs().contains(RECEIVER))
            .ok_or("No rx_nand")?
            .0
            .clone();
        let rx_nand_inputs = modules.get(&rx_nand).unwrap().get_inputs();

        for element in rx_nand_inputs {
            let mut modules = modules.clone();
            let mut stack = VecDeque::new();
            let mut counter = 0;
            'foo: loop {
                stack.push_back((BUTTON.to_string(), BROADCASTER.to_string(), Pulse::Low));
                counter += 1;

                while let Some((input, output, pulse)) = stack.pop_front() {
                    if input == element && pulse == Pulse::High {
                        counters.push(counter);
                        break 'foo;
                    }
                    if let Some(module) = modules.get_mut(&output) {
                        let (new_pulse, outputs) = module.handle_pulse(input.clone(), &pulse);
                        stack.extend(
                            outputs
                                .iter()
                                .map(|s| (output.clone(), s.clone(), new_pulse.clone())),
                        );
                    }
                }
            }
        }

        Ok(counters.iter().fold(1, |a, b| lcm(a, *b)))
    }
}

fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    Day {}.run(ID);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1_test1() {
        let day = Day {};
        assert_eq!(
            day.parse_and_solve_part1(format!("{}_test1", ID).as_str())
                .unwrap(),
            32000000
        );
    }

    #[test]
    fn test_solve_part1_test2() {
        let day = Day {};
        assert_eq!(
            day.parse_and_solve_part1(format!("{}_test2", ID).as_str())
                .unwrap(),
            11687500
        );
    }
}
