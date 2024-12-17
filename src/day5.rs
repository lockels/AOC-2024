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
    let mut sum = 0;

    for update in updates {
        if follows_rules(rules, update) {
            sum += update[update.len() / 2];
        }
    }

    sum
}

fn follows_rules(rules: &[(u32, u32)], update: &[u32]) -> bool {
    let mut map: HashMap<u32, u32> = HashMap::new();

    for (i, num) in update.iter().enumerate() {
        map.insert(*num, i as u32);
    }

    for (rule_a, rule_b) in rules {
        if map.contains_key(rule_a) && map.contains_key(rule_b) && map.get(rule_a) > map.get(rule_b)
        {
            return false;
        }
    }

    true
}
