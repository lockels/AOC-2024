use std::fs;

#[allow(dead_code)]

pub fn main() {
    let input = fs::read_to_string("inputs/day7.txt").expect("Unable to open file");
    let equations = parse_equations(&input);

    let solution_1 = solve(&equations, is_possible_result_1);
    println!("Solution: {:?}", solution_1);

    let solution_2 = solve(&equations, is_possible_result_2);
    println!("Solution: {:?}", solution_2);
}

fn parse_equations(input: &str) -> Vec<(u64, Vec<u64>)> {
    input
        .lines()
        .map(|line| {
            let (answer, values) = line.split_once(':').unwrap();
            let values = values
                .split_whitespace()
                .map(|num| num.parse::<u64>().unwrap())
                .collect();
            (answer.parse().unwrap(), values)
        })
        .collect()
}

fn solve<F>(equations: &[(u64, Vec<u64>)], filter_fn: F) -> u64
where
    F: Fn(u64, &[u64]) -> bool,
{
    equations
        .iter()
        .filter(|(answer, values)| filter_fn(*answer, values))
        .map(|(answer, _)| *answer)
        .sum()
}

fn is_possible_result_1(answer: u64, values: &[u64]) -> bool {
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

fn is_possible_result_2(answer: u64, values: &[u64]) -> bool {
    if values.is_empty() {
        return false;
    }

    fn concat(current: u64, next: u64) -> u64 {
        let next_str = next.to_string();
        let current_str = current.to_string();
        format!("{}{}", current_str, next_str).parse().unwrap()
    }

    fn helper(answer: u64, index: usize, current: u64, values: &[u64]) -> bool {
        if index == values.len() {
            return current == answer;
        }

        let next_value = values[index];

        helper(answer, index + 1, current + next_value, values)
            || helper(answer, index + 1, current * next_value, values)
            || helper(answer, index + 1, concat(current, next_value), values)
    }

    helper(answer, 1, values[0], values)
}
