use std::{fs, usize};

#[allow(dead_code)]
pub fn main() {
    let input = fs::read_to_string("inputs/test.txt").expect("Unable to open file");
    let blocks = parse_file_system(&input);

    println!("{:?}", blocks);
    println!();
    let solution_1 = solve_1(&blocks);
    println!("{}", solution_1);
    // let solution_2 = solve_2(&blocks);
    // println!("{}", solution_2);
}

fn parse_file_system(input: &str) -> Vec<Option<usize>> {
    let mut blocks = Vec::new();

    for (i, c) in input.trim().chars().enumerate() {
        if let Some(num) = c.to_digit(10) {
            let fill = if i % 2 == 0 { Some(i / 2) } else { None };
            for _ in 0..num {
                blocks.push(fill);
            }
        }
    }

    blocks
}

fn solve_1(blocks: &[Option<usize>]) -> usize {
    let mut ordered_blocks = blocks.to_vec();
    let (mut left, mut right) = (0, ordered_blocks.len() - 1);

    while left < right {
        if ordered_blocks[left].is_some() {
            left += 1;
        } else if ordered_blocks[right].is_none() {
            right -= 1;
        } else {
            ordered_blocks.swap(left, right);
            left += 1;
            right -= 1;
        }
    }

    calculate_checksum(&ordered_blocks)
}

fn calculate_checksum(ordered_blocks: &[Option<usize>]) -> usize {
    ordered_blocks
        .iter()
        .enumerate()
        .filter_map(|(i, block)| block.map(|b| i * b))
        .sum()
}
