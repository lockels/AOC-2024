use std::fs;

#[allow(dead_code)]

pub fn main() {
    let input = fs::read_to_string("inputs/day2.txt").expect("Unable to read file");

    let reports = parse_input(&input);

    let solution_1 = solve(&reports, is_safe_report_1);
    println!("{}", solution_1);

    let solution_2 = solve(&reports, is_safe_report_2);
    println!("{}", solution_2);
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .map(|num| num.parse::<i32>().unwrap())
        .collect()
}

fn solve(reports: &[Vec<i32>], is_safe_fn: fn(&[i32]) -> bool) -> i32 {
    reports
        .iter()
        .filter(|report| is_safe_fn(report))
        .count() as i32
}

fn is_safe_report_1(report: &[i32]) -> bool {
    let is_increasing = report.windows(2).all(|pair| pair[0] < pair[1]);
    let is_decreasing = report.windows(2).all(|pair| pair[0] > pair[1]);

    let is_valid_range = report.windows(2).all(|pair| {
        let diff = (pair[0] - pair[1]).abs();
        diff >= 1 && diff <= 3
    });

    (is_increasing || is_decreasing) && is_valid_range
}

fn is_safe_report_2(report: &[i32]) -> bool {
    if is_safe_report_1(report) {
        return true;
    }

    for i in 0..report.len() {
        let mut modified_report = report.to_vec();
        modified_report.remove(i);
        if is_safe_report_1(&modified_report) {
            return true;
        }
    }

    false
}
