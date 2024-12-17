use std::{collections::HashMap, fmt::write, fs, iter::Map};

#[allow(dead_code)]

pub fn main() {
    let input = fs::read_to_string("inputs/day5.txt").expect("Unable to open file");
    let (fst, snd) = input.split_once("\n\n").unwrap();

    let rules = parse_rules(fst);
    let updates = parse_updates(snd);

    let solution = solve(&rules, &updates);
    println!("{}", solution);
}

fn parse_rules(input: &str) -> Vec<(u32, u32)> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split('|').map(|num| num.parse::<u32>().unwrap());
            (parts.next().unwrap(), parts.next().unwrap())
        })
        .collect()
}

fn parse_updates(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.trim()
                .split(',')
                .map(|num| num.parse::<u32>().unwrap())
                .collect()
        })
        .collect()
}

fn solve(rules: &[(u32, u32)], updates: &[Vec<u32>]) -> u32 {
    updates
        .iter()
        .filter(|update| follows_rules(rules, update))
        .map(|update| update[update.len() / 2])
        .sum()
}

fn follows_rules(rules: &[(u32, u32)], update: &[u32]) -> bool {
    let mut positions = HashMap::with_capacity(update.len());

    for (i, num) in update.iter().enumerate() {
        positions.insert(*num, i as u32);
    }

    for &(a, b) in rules {
        match (positions.get(&a), positions.get(&b)) {
            (Some(&pos_a), Some(&pos_b)) if pos_a > pos_b => return false,
            _ => continue,
        }
    }


    true
}
