use std::{fs, usize};

#[allow(dead_code)]
pub fn main() {
    let input = fs::read_to_string("inputs/test.txt").expect("Unable to open file");
    let blocks = parse_file_system(&input);

    let solution_1 = solve_1(&blocks);
    println!("{}", solution_1);
    let solution_2 = solve_2(&blocks, &input);
    println!("{}", solution_2);
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


fn solve_2(blocks: &[Option<usize>], input: &str) -> usize {
    let mut ordered_blocks = blocks.to_vec();
    let mut right = ordered_blocks.len();
    let mut num_processed = 0;

    while right > 0 {
        right -= 1;

        if num_processed == (input.len() - 5) / 2 {
            break;
        }

        if ordered_blocks[right].is_none() {
            continue;
        }

        let file_id = ordered_blocks[right];
        let mut start = right;

        while start > 0 && ordered_blocks[start - 1] == file_id {
            start -= 1;
        }

        let block_size = right - start + 1;

        right -= block_size - 1;

        let mut left = 0;
        let mut free_start = None;
        let mut free_size = 0;

        while left < ordered_blocks.len() {
            if ordered_blocks[left].is_none() {
                if free_start.is_none() {
                    free_start = Some(left);
                }
                free_size += 1;
                if free_size == block_size {
                    break;
                }
            } else {
                free_size = 0;
                free_start = None;
            }
            left += 1;
        }

        if free_size >= block_size {
            let free_start = free_start.unwrap();
            for i in 0..block_size {
                ordered_blocks[free_start + i] = Some(file_id.unwrap());
            }

            for i in start..start + block_size {
                ordered_blocks[i] = None;
            }
        }

        num_processed += 1;
        println!("{}", num_processed);

        println!("{:?}", ordered_blocks);
        println!("");
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
