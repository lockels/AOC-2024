use std::{fs, result};

use regex::Regex;

#[allow(dead_code)]

pub fn main() {
    let input = fs::read_to_string("inputs/day3.txt").expect("Unable to read file");

    let solution_1: i32 = solve_1(&input);
    println!("{}", solution_1);

    let solution_2: i32 = solve_2(&input);
    println!("{}", solution_2);
}

fn solve_1(input: &str) -> i32 {
    let pattern = r"(?i)mul\((\d+),(\d+)\)";
    let re = Regex::new(pattern).unwrap();

    re.captures_iter(input)
        .map(|cap| {
            let num1: i32 = cap[1].parse().unwrap();
            let num2: i32 = cap[2].parse().unwrap();
            num1 * num2
        })
        .sum()
}

fn solve_2(input: &str) -> i32 {
    let dos: Vec<&str> = input.split("do()").collect();

    let mut sum = 0;

    for i in 0..dos.len() {
        let result: Vec<&str> = dos[i].split("don't()").collect();
        let result_str = result[0];
        sum += solve_1(result_str);
    }
    sum
}
