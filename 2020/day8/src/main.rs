use std::collections::HashSet;
use std::fs::read_to_string;
use std::time::Instant;

fn part1_parse(ins: &str, pc: usize, acc: u32) -> (usize, u32) {
    match &ins[0..5] {
        "acc +" => (pc + 1, acc + *&ins[5..].parse::<u32>().unwrap()),
        "acc -" => (pc + 1, acc - *&ins[5..].parse::<u32>().unwrap()),
        "jmp +" => (pc + *&ins[5..].parse::<usize>().unwrap(), acc),
        "jmp -" => (pc - *&ins[5..].parse::<usize>().unwrap(), acc),
        _ => (pc + 1, acc),
    }
}

fn part1(bc: &Vec<&str>) -> u32 {
    let mut seen_pc: HashSet<usize> = HashSet::new();
    let mut pc = 0;
    let mut acc = 0;
    loop {
        if !seen_pc.contains(&pc) {
            seen_pc.insert(pc);
            let (new_pc, new_bar) = part1_parse(bc.get(pc).unwrap(), pc, acc);
            pc = new_pc;
            acc = new_bar;
        } else {
            return acc;
        }
    }
}

fn part2_parse(ins: &str, pc: usize, acc: i32) -> ((usize, i32), Option<(usize, i32)>) {
    let count = *&ins[5..].parse::<usize>().unwrap();
    match &ins[0..5] {
        "acc +" => ((pc + 1, acc + count as i32), None),
        "acc -" => ((pc + 1, acc - count as i32), None),
        "jmp +" => ((pc + count, acc), Some((pc + 1, acc))),
        "jmp -" => ((pc - count, acc), Some((pc + 1, acc))),
        "nop +" => ((pc + 1, acc), Some((pc + count, acc))),
        "nop -" => ((pc + 1, acc), Some((pc - count, acc))),
        _ => panic!("Unknown instruction"),
    }
}

fn part2_run(
    funcs: &Vec<&str>,
    pc: usize,
    acc: i32,
    altered: &bool,
    mut seen_pc: HashSet<usize>,
) -> Option<i32> {
    if seen_pc.contains(&pc) {
        return None;
    }
    if let Some(line) = funcs.get(pc) {
        seen_pc.insert(pc);
        let ((new_pc, new_acc), alternate) = part2_parse(line, pc, acc);
        let mut result = part2_run(funcs, new_pc, new_acc, altered, seen_pc.clone());
        if result.is_none() && !altered && alternate.is_some() {
            let (new_pc, new_acc) = alternate.unwrap();
            result = part2_run(funcs, new_pc, new_acc, &true, seen_pc.clone());
        }
        result
    } else {
        Some(acc)
    }
}

fn part2(bc: &Vec<&str>) -> i32 {
    part2_run(bc, 0, 0, &false, HashSet::new()).unwrap()
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
