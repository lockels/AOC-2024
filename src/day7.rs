use std::fs;

#[allow(dead_code)]

pub fn main() {
    let input = fs::read_to_string("inputs/day7.txt").expect("Unable to open file");
    let equations = parse_equations(&input);

    let solution = solve(&equations);
    println!("Solution: {:?}", solution);
}

fn parse_equations(input: &str) -> Vec<(u64, Vec<u64>)> {
    input.lines().map(|line| {
        let (answer, values) = line.split_once(':').unwrap();
        let values = values
            .split_whitespace()
            .map(|num| num.parse::<u64>().unwrap())
            .collect();
        (answer.parse().unwrap(), values)
    }).collect()
}

fn solve(equations: &[(u64, Vec<u64>)]) -> u64 {
    equations
        .iter()
        .filter(|(answer, values)| is_possible_result(*answer, values))
        .map(|(answer, _)| *answer)
        .sum()
}

fn is_possible_result(answer: u64, values: &[u64]) -> bool {
    if values.is_empty() {
        return false;
    }

    fn helper(answer: u64, index: usize, current: u64, values: &[u64]) -> bool {
        if index == values.len() {
            return current == answer;
        }

        let next_value = values[index];

        helper(answer, index + 1, current + next_value, values)
            || helper(answer, index + 1, current * next_value, values)
    }

    helper(answer, 1, values[0], values)
}
