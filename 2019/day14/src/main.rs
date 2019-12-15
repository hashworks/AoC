use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

type ReactionMap = HashMap<String, ChemicalReaction>;

#[derive(Clone)]
struct ChemicalReaction {
    amount_provided: u64,
    chemicals_needed: Vec<(u64, String)>,
}

impl ChemicalReaction {
    fn get_required_ore_count(
        &self,
        multiplier: u64,
        reaction_map: &ReactionMap,
        storage: &mut HashMap<String, u64>,
    ) -> u64 {
        self.chemicals_needed
            .clone()
            .into_iter()
            .map(|(amount_required, chemical)| (amount_required * multiplier, chemical))
            .fold(0, |ore, (amount_required, chemical)| {
                ore + if chemical == "ORE" {
                    amount_required
                } else {
                    let amount_required = if let Some(&amount_in_storage) = storage.get(&chemical) {
                        if amount_in_storage > amount_required {
                            storage.insert(chemical.clone(), amount_in_storage - amount_required);
                            0
                        } else {
                            storage.remove(&chemical.clone());
                            amount_required - amount_in_storage
                        }
                    } else {
                        amount_required
                    };

                    let chemical_reaction = reaction_map.get(&chemical).unwrap();

                    let mut to_produce = amount_required / chemical_reaction.amount_provided;
                    let missing_amount =
                        amount_required - to_produce * chemical_reaction.amount_provided;
                    if missing_amount > 0 {
                        to_produce += 1;
                    }

                    let leftovers =
                        to_produce * chemical_reaction.amount_provided - amount_required;
                    if leftovers > 0 {
                        if let Some(&amount_in_storage) = storage.get(&chemical) {
                            storage.insert(chemical.to_string(), amount_in_storage + leftovers);
                        } else {
                            storage.insert(chemical.to_string(), leftovers);
                        }
                    }

                    chemical_reaction.get_required_ore_count(to_produce, reaction_map, storage)
                }
            })
    }
}

fn binary_search(f: &dyn Fn(u64) -> u64, upper: u64, target: u64) -> u64 {
    let mut l = 0;
    let mut r = upper;
    while l <= r {
        let m = (l + r) / 2;
        let result = f(m);
        if result < target {
            l = m + 1;
        } else if result > target {
            r = m - 1;
        } else {
            return m;
        }
    }
    r
}

fn main() {
    let s1 = Instant::now();

    let mut reaction_map: ReactionMap = HashMap::new();
    BufReader::new(File::open("./input").unwrap())
        .lines()
        .map(|l| {
            let l = l.unwrap();
            let needed_produces = l.split(" => ").take(2).collect::<Vec<&str>>();
            let provided_produces = needed_produces[1].split(' ').take(2).collect::<Vec<&str>>();

            (
                provided_produces[1].to_string(),
                ChemicalReaction {
                    amount_provided: provided_produces[0].parse::<u64>().unwrap(),
                    chemicals_needed: needed_produces[0]
                        .split(", ")
                        .map(|required_amount_chemical| {
                            let required_amount_chemical = required_amount_chemical
                                .split(' ')
                                .take(2)
                                .collect::<Vec<&str>>();
                            (
                                required_amount_chemical[0].parse::<u64>().unwrap(),
                                required_amount_chemical[1].to_string(),
                            )
                        })
                        .collect::<Vec<(u64, String)>>()
                        .clone(),
                },
            )
        })
        .for_each(|(chemical, chemical_reaction)| {
            reaction_map.insert(chemical, chemical_reaction);
        });

    println!(
        "part1: {}, ({}µs)",
        reaction_map.get("FUEL").unwrap().get_required_ore_count(
            1,
            &reaction_map,
            &mut HashMap::new()
        ),
        s1.elapsed().as_micros()
    );

    let s2 = Instant::now();

    println!(
        "part2: {}, ({}µs)",
        binary_search(
            &|fuel| reaction_map.get("FUEL").unwrap().get_required_ore_count(
                fuel,
                &reaction_map,
                &mut HashMap::new()
            ),
            5_000_000,
            1_000_000_000_000
        ),
        s2.elapsed().as_micros()
    );

    println!("time: {}µs", s1.elapsed().as_micros());
}
