use std::{collections::HashMap, fs};

#[allow(dead_code)]

pub fn main() {
    let input = fs::read_to_string("inputs/day5.txt").expect("Unable to open file");
    let (fst, snd) = input.split_once("\n\n").unwrap();

    let rules = parse_rules(fst);
    let updates = parse_updates(snd);

    let solution_1 = solve_1(&rules, &updates);
    println!("{}", solution_1);

    let solution_2 = solve_2(&rules, &updates);
    println!("{}", solution_2);
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

fn solve_1(rules: &[(u32, u32)], updates: &[Vec<u32>]) -> u32 {
    updates
        .iter()
        .filter(|update| follows_rules(rules, update))
        .map(|update| update[update.len() / 2])
        .sum()
}

fn follows_rules(rules: &[(u32, u32)], update: &[u32]) -> bool {
    let mut positions = HashMap::with_capacity(update.len());

    for (i, page_num) in update.iter().enumerate() {
        positions.insert(*page_num, i as u32);
    }

    for &(a, b) in rules {
        match (positions.get(&a), positions.get(&b)) {
            (Some(&pos_a), Some(&pos_b)) if pos_a > pos_b => return false,
            _ => continue,
        }
    }

    true
}

fn solve_2(rules: &[(u32, u32)], updates: &[Vec<u32>]) -> u32 {
    updates
        .iter()
        .filter(|update| !follows_rules(rules, update))
        .map(|update| {
            let sorted = sort_by_rules(rules, update);
            sorted[sorted.len() / 2]
        })
        .sum()
}


fn sort_by_rules(rules: &[(u32, u32)], update: &[u32]) -> Vec<u32> {
    let relvant_rules: Vec<(u32, u32)> = rules
        .iter()
        .filter(|(a, b)| update.contains(a) && update.contains(b))
        .copied()
        .collect();

    let mut in_degree: HashMap<u32, u32> = HashMap::new();
    for &page in update {
        in_degree.insert(page, 0);
    }

    for &(_a, b) in &relvant_rules {
        if let Some(count) = in_degree.get_mut(&b) {
            *count += 1;
        }
    }

    let mut answer: Vec<u32> = Vec::with_capacity(update.len());
    let mut remaining: Vec<u32> = update.to_vec();

    while !remaining.is_empty() {
        let mut next_round = Vec::new();

        for &page in &remaining {
            if *in_degree.get(&page).unwrap_or(&0) == 0 {
                answer.push(page);
                for &(a, b) in &relvant_rules {
                    if a == page {
                        if let Some(count) = in_degree.get_mut(&b) {
                            *count -= 1;
                        }
                    }
                }
            } else {
                next_round.push(page);
            }
        }

        remaining = next_round;
    }

    answer
}

