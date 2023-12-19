mod util;

use std::{collections::HashMap, error::Error, io::BufRead, ops::RangeInclusive};
use util::{aoc::AoCDay, input::get_reader};

const ID: &str = "day19";
type Input = (HashMap<String, Vec<Rule>>, Vec<Part>);
type Output = usize;

#[derive(Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

struct Rule {
    comparisation: Option<Comparisation>,
    target: String,
}

impl std::fmt::Debug for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(comparisation) = &self.comparisation {
            write!(f, "{:?} -> {}", comparisation, self.target)
        } else {
            write!(f, "any -> {}", self.target)
        }
    }
}

struct Comparisation {
    cat: Cat,
    comperator: Comperator,
    value: usize,
}

impl std::fmt::Debug for Comparisation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cat = match self.cat {
            Cat::X => "x",
            Cat::M => "m",
            Cat::A => "a",
            Cat::S => "s",
        };
        let comperator = match self.comperator {
            Comperator::GT => ">",
            Comperator::LT => "<",
        };
        write!(f, "{}{}{}", cat, comperator, self.value)
    }
}

#[derive(Debug)]
enum Cat {
    X,
    M,
    A,
    S,
}

#[derive(Debug)]
enum Comperator {
    GT,
    LT,
}

impl Rule {
    fn compare(&self, part: &Part) -> Option<&str> {
        if let Some(comparisation) = &self.comparisation {
            let value = match comparisation.cat {
                Cat::X => part.x,
                Cat::M => part.m,
                Cat::A => part.a,
                Cat::S => part.s,
            };
            match comparisation.comperator {
                Comperator::GT => {
                    if value > comparisation.value {
                        return Some(&self.target);
                    }
                }
                Comperator::LT => {
                    if value < comparisation.value {
                        return Some(&self.target);
                    }
                }
            }
        } else {
            return Some(&self.target);
        }
        None
    }
}

struct Day {}

impl AoCDay<Input, Output> for Day {
    fn parse_input(&self, id: &str) -> Result<Input, Box<dyn Error>> {
        let lines = get_reader(id)?.lines().flatten().collect::<Vec<_>>();

        let rules = lines
            .iter()
            .take_while(|l| !l.is_empty())
            .map(parse_workflow)
            .collect::<Result<HashMap<_, _>, _>>()?;

        let parts = lines
            .iter()
            .skip(rules.len() + 1)
            .map(parse_part)
            .collect::<Result<_, _>>()?;

        Ok((rules, parts))
    }

    fn part1(&self, (workflows, parts): &Input) -> Result<Output, Box<dyn Error>> {
        parts
            .iter()
            .map(|part| {
                let mut workflow_name = "in".to_string();
                while let Some(workflow) = workflows.get(&workflow_name) {
                    if let Some(next_workflow_name) = workflow
                        .iter()
                        .find_map(|rule| rule.compare(part))
                        .map(|s| s.to_string())
                    {
                        if next_workflow_name == "A" {
                            return Ok(part.x + part.m + part.a + part.s);
                        } else if next_workflow_name == "R" {
                            return Ok(0);
                        }
                        workflow_name = next_workflow_name;
                    } else {
                        return Err("no rule matched".into());
                    }
                }
                Err("no workflow found".into())
            })
            .sum()
    }

    fn part2(&self, (rules, _): &Input) -> Result<Output, Box<dyn Error>> {
        Ok(calculate_variants(
            rules,
            1..=4000,
            1..=4000,
            1..=4000,
            1..=4000,
            "in",
        ))
    }
}

// px{a<2006:qkq,m>2090:A,rfg}
#[allow(clippy::ptr_arg)]
fn parse_workflow(line: &String) -> Result<(String, Vec<Rule>), Box<dyn Error>> {
    let (name, rest) = line.split_once('{').ok_or("no {")?;

    let rest = rest.strip_suffix('}').ok_or("no }")?;

    let rules = rest
        .split(',')
        .map(|rule_str| {
            if let Some((expression, target)) = rule_str.split_once(':') {
                let cat = match rule_str.chars().next() {
                    Some('x') => Cat::X,
                    Some('m') => Cat::M,
                    Some('a') => Cat::A,
                    Some('s') => Cat::S,
                    _ => return Err("invalid category".into()),
                };
                let comperator = match expression.chars().nth(1) {
                    Some('<') => Comperator::LT,
                    Some('>') => Comperator::GT,
                    _ => return Err("invalid comperator".into()),
                };

                if expression.len() < 3 {
                    return Err("invalid expression".into());
                }

                Ok(Rule {
                    comparisation: Some(Comparisation {
                        cat,
                        comperator,
                        value: expression[2..].parse()?,
                    }),
                    target: target.to_string(),
                })
            } else {
                Ok(Rule {
                    comparisation: None,
                    target: rule_str.to_string(),
                })
            }
        })
        .collect::<Result<_, Box<dyn Error>>>()?;

    Ok((name.to_owned(), rules))
}

// {x=819,m=813,a=1378,s=199}
#[allow(clippy::ptr_arg)]
fn parse_part(line: &String) -> Result<Part, Box<dyn Error>> {
    let line = line.strip_prefix('{').ok_or("no {")?;
    let line = line.strip_suffix('}').ok_or("no }")?;
    let values = line
        .split(',')
        .map(|cat_value| cat_value[2..].parse())
        .collect::<Result<Vec<_>, _>>()?;
    #[allow(clippy::get_first)]
    let x = *values.get(0).ok_or("no x")?;
    let m = *values.get(1).ok_or("no x")?;
    let a = *values.get(2).ok_or("no x")?;
    let s = *values.get(3).ok_or("no x")?;
    Ok(Part { x, m, a, s })
}

fn calculate_variants(
    workflows: &HashMap<String, Vec<Rule>>,
    x_range: RangeInclusive<usize>,
    m_range: RangeInclusive<usize>,
    a_range: RangeInclusive<usize>,
    s_range: RangeInclusive<usize>,
    workflow_name: &str,
) -> usize {
    match workflow_name {
        "A" => x_range.count() * m_range.count() * a_range.count() * s_range.count(),
        "R" => 0,
        workflow_name => {
            if let Some(workflow) = workflows.get(workflow_name) {
                let mut x_range = x_range;
                let mut m_range = m_range;
                let mut a_range = a_range;
                let mut s_range = s_range;
                workflow
                    .iter()
                    .map(|rule| {
                        if let Some(comparisation) = &rule.comparisation {
                            match comparisation.cat {
                                Cat::X => {
                                    let (true_x_range, false_x_range) =
                                        get_ranges(&x_range, comparisation);
                                    x_range = false_x_range;
                                    calculate_variants(
                                        workflows,
                                        true_x_range,
                                        m_range.clone(),
                                        a_range.clone(),
                                        s_range.clone(),
                                        rule.target.as_str(),
                                    )
                                }
                                Cat::M => {
                                    let (true_m_range, false_m_range) =
                                        get_ranges(&m_range, comparisation);
                                    m_range = false_m_range;
                                    calculate_variants(
                                        workflows,
                                        x_range.clone(),
                                        true_m_range,
                                        a_range.clone(),
                                        s_range.clone(),
                                        rule.target.as_str(),
                                    )
                                }
                                Cat::A => {
                                    let (true_a_range, false_a_range) =
                                        get_ranges(&a_range, comparisation);
                                    a_range = false_a_range;
                                    calculate_variants(
                                        workflows,
                                        x_range.clone(),
                                        m_range.clone(),
                                        true_a_range,
                                        s_range.clone(),
                                        rule.target.as_str(),
                                    )
                                }
                                Cat::S => {
                                    let (true_s_range, false_s_range) =
                                        get_ranges(&s_range, comparisation);
                                    s_range = false_s_range;
                                    calculate_variants(
                                        workflows,
                                        x_range.clone(),
                                        m_range.clone(),
                                        a_range.clone(),
                                        true_s_range,
                                        rule.target.as_str(),
                                    )
                                }
                            }
                        } else {
                            calculate_variants(
                                workflows,
                                x_range.clone(),
                                m_range.clone(),
                                a_range.clone(),
                                s_range.clone(),
                                rule.target.as_str(),
                            )
                        }
                    })
                    .sum()
            } else {
                0
            }
        }
    }
}

fn get_ranges(
    range: &RangeInclusive<usize>,
    comparisation: &Comparisation,
) -> (RangeInclusive<usize>, RangeInclusive<usize>) {
    match comparisation.comperator {
        Comperator::GT => (
            (comparisation.value + 1)..=*range.end(),
            *range.start()..=comparisation.value,
        ),
        Comperator::LT => (
            *range.start()..=(comparisation.value - 1),
            comparisation.value..=*range.end(),
        ),
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
            19114
        );
    }

    #[test]
    fn test_solve_part2() {
        let day = Day {};
        assert_eq!(
            day.parse_and_solve_part2(format!("{}_test1", ID).as_str())
                .unwrap(),
            167409079868000
        );
    }

    #[test]
    fn test_get_ranges_lt() {
        let range = 1..=4000;
        let comparisation = Comparisation {
            cat: Cat::S,
            comperator: Comperator::LT,
            value: 1351,
        };
        let (true_range, false_range) = get_ranges(&range, &comparisation);
        assert_eq!(true_range, 1..=1350);
        assert_eq!(false_range, 1351..=4000);
    }

    #[test]
    fn test_get_ranges_gt() {
        let range = 1351..=4000;
        let comparisation = Comparisation {
            cat: Cat::S,
            comperator: Comperator::GT,
            value: 2770,
        };
        let (true_range, false_range) = get_ranges(&range, &comparisation);
        assert_eq!(true_range, 2771..=4000);
        assert_eq!(false_range, 1351..=2770);
    }
}
