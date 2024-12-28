use std::{collections::HashMap, fs};

#[allow(dead_code)]
pub fn main() {
    let input = fs::read_to_string("inputs/test.txt").expect("Unable to open file");
    let blocks = parse_file_system(&input);

    println!("{:?}", blocks);
    println!();
    println!("{:?}", calculate_file_lengths(&blocks));
    // let solution_1 = solve_1(&blocks);
    // println!("{}", solution_1);
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
    let mut first_free = 0;
    while let Some(_) = blocks[first_free] {
        first_free += 1;
    }

    let mut moveable = blocks.len() - 1;
    while let None = blocks[moveable] {
        moveable -= 1;
    }

    let mut ordered_blocks = blocks.to_vec();

    while moveable > first_free {
        ordered_blocks.swap(first_free, moveable);

        while let Some(_) = ordered_blocks[first_free] {
            first_free += 1;
        }

        while let None = ordered_blocks[moveable] {
            moveable -= 1;
        }
    }

    calculate_checksum(&ordered_blocks)
}

fn solve_2(blocks: &[Option<usize>]) -> usize {
    todo!()
}

fn calculate_checksum(ordered_blocks: &[Option<usize>]) -> usize {
    ordered_blocks
        .iter()
        .enumerate()
        .filter_map(|(i, block)| block.map(|b| i * b))
        .sum()
}

fn calculate_file_lengths(blocks: &[Option<usize>]) -> HashMap<Option<usize>, (usize, usize)> {
    let mut result = HashMap::new();
    let mut current_start = None;
    let mut count = 0;
    let mut cur_block = None;

    for (index, &block) in blocks.iter().enumerate() {
        let mut cur_num = usize::MAX;
        match block {
            Some(num) => {
                if current_start.is_none() {
                    cur_num = num;
                    count = 1;
                    cur_block = block;
                    current_start = Some(index);
                } else {
                    count += 1;
                }
            }
            None => {
                if let Some(start) = current_start {
                    result.insert(cur_block, (start, count));
                    count = 0;
                    cur_block = None;
                    current_start = None;
                }
            }
        }
    }

    if let Some(start) = current_start {
        result.insert(cur_block, (start, count));
    }

    result
}
