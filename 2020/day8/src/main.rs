use std::collections::HashSet;
use std::fs::read_to_string;
use std::time::Instant;

fn parse(ins: &str, pc: usize, acc: i32) -> (usize, i32) {
    match &ins[0..5] {
        "acc +" => (pc + 1, acc + ins[5..].parse::<i32>().unwrap()),
        "acc -" => (pc + 1, acc - ins[5..].parse::<i32>().unwrap()),
        "jmp +" => (pc + ins[5..].parse::<usize>().unwrap(), acc),
        "jmp -" => (pc - ins[5..].parse::<usize>().unwrap(), acc),
        _ => (pc + 1, acc),
    }
}

fn part1(bc: &Vec<&str>) -> i32 {
    let mut seen_pc: HashSet<usize> = HashSet::new();
    let mut reg = (0, 0);
    loop {
        if seen_pc.insert(reg.0) {
            reg = parse(bc.get(reg.0).unwrap(), reg.0, reg.1);
        } else {
            return reg.1;
        }
    }
}

fn parse_alternate(ins: &str, pc: usize, acc: i32) -> Option<(usize, i32)> {
    match &ins[0..5] {
        "jmp +" => Some((pc + 1, acc)),
        "jmp -" => Some((pc + 1, acc)),
        "nop +" => Some((pc + ins[5..].parse::<usize>().unwrap(), acc)),
        "nop -" => Some((pc - ins[5..].parse::<usize>().unwrap(), acc)),
        _ => None,
    }
}

fn part2_run(
    funcs: &Vec<&str>,
    pc: usize,
    acc: i32,
    altered: &bool,
    seen_pc: &mut HashSet<usize>,
) -> Option<i32> {
    if !seen_pc.insert(pc) {
        None
    } else if let Some(line) = funcs.get(pc) {
        let (new_pc, new_acc) = parse(line, pc, acc);
        let mut result = part2_run(funcs, new_pc, new_acc, altered, seen_pc);
        if result.is_none() && !altered {
            if let Some((new_pc, new_acc)) = parse_alternate(line, pc, acc) {
                result = part2_run(funcs, new_pc, new_acc, &true, seen_pc);
            }
        }
        result
    } else {
        Some(acc)
    }
}

fn part2(bc: &Vec<&str>) -> i32 {
    part2_run(bc, 0, 0, &false, &mut HashSet::new()).unwrap()
}

fn main() {
    let s1 = Instant::now();

    let str = read_to_string("input").unwrap();
    let boot_code: Vec<&str> = str.lines().collect();

    println!(
        "part1: {} ({}µs)",
        part1(&boot_code),
        s1.elapsed().as_micros()
    );

    let s2 = Instant::now();

    println!(
        "part2: {} ({}µs)",
        part2(&boot_code),
        s2.elapsed().as_micros()
    );

    println!("Time: {}µs", s1.elapsed().as_micros());
}

#[test]
fn part1_test() {
    assert_eq!(
        5,
        part1(
            &"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"
                .lines()
                .collect()
        )
    )
}

#[test]
fn part2_test() {
    assert_eq!(
        8,
        part2(
            &"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"
                .lines()
                .collect()
        )
    )
}
