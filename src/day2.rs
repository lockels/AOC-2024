use std::fs;

#[allow(dead_code)]

pub fn main() {
    let input = fs::read_to_string("inputs/day2.txt").expect("Unable to read file");

    let reports = parse_input(&input);

    let solution_1 = solve_1(&reports);
    println!("{}", solution_1);

    let solution_2 = solve_2(&reports);
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

fn solve_1(reports: &[Vec<i32>]) -> i32 {
    reports
        .iter()
        .filter(|report| is_safe_report_1(report))
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

fn solve_2(reports: &[Vec<i32>]) -> i32 {
    reports
        .iter()
        .filter(|report| is_safe_report_2(report))
        .count() as i32
}

fn is_safe_report_2(report: &[i32]) -> bool {
    todo!();
}
