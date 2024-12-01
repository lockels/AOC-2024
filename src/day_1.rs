use std::fs;

pub fn main() {
    let input = fs::read_to_string("inputs/day_1.txt")
        .expect("Unable to read file");

    let (left_list, right_list) = parse_input(&input);
    let solution = solve(left_list, right_list);

    println!("Solution: {}", solution);
}

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();

    for line in input.lines() {
        if let Some((left, right)) = parse_line(line) {
            left_list.push(left);
            right_list.push(right);
        }
    }

    (left_list, right_list)
}

fn parse_line(line: &str) -> Option<(i32, i32)> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() == 2 {
        if let (Ok(left), Ok(right)) = (parts[0].parse(), parts[1].parse()){
            return Some((left, right));
        }
    }
    None
}

// O(n log n) solution:
fn solve(mut left_list: Vec<i32>, mut right_list: Vec<i32>) -> i32 {
    left_list.sort();
    right_list.sort();

    let mut sum: i32 = 0;
    for i in 0..left_list.len() {
        let left_val = left_list[i];
        let right_val = right_list[i];

        let difference = i32::abs(left_val - right_val);
        sum += difference;
    }
    sum
}
