use std::{fs, usize};

#[allow(dead_code)]
pub fn main() {
    let input = fs::read_to_string("inputs/test.txt").expect("Unable to open file");
    let blocks = parse_disk_map(&input);

    let solution = solve(&blocks);
    println!("{}", solution);
}

fn parse_disk_map(input: &str) -> Vec<Option<usize>> {
    let mut result = Vec::new();

    for (i, c) in input.trim().chars().enumerate() {
        if let Some(num) = c.to_digit(10) {
            let fill = if i % 2 == 0 { Some(i / 2) } else { None };
            for _ in 0..num {
                result.push(fill);
            }
        }
    }

    result
}

fn solve(blocks: &[Option<usize>]) -> usize {
    let mut first_free = 0;
    while let Some(_) = blocks[first_free] {
        first_free += 1;
    }

    let mut moveable = blocks.len() - 1;
    while let None = blocks[moveable] {
        moveable -= 1;
    }

    let mut coalesced_blocks = blocks.to_vec();

    while moveable > first_free {
        coalesced_blocks[first_free] = coalesced_blocks[moveable];
        coalesced_blocks[moveable] = None;

        while let Some(_) = coalesced_blocks[first_free] {
            first_free += 1;
        }

        while let None = coalesced_blocks[moveable] {
            moveable -= 1;
        }
    }

    calculate_checksum(&coalesced_blocks)
}

fn calculate_checksum(ordered_blocks: &[Option<usize>]) -> usize {
    ordered_blocks
        .iter()
        .enumerate()
        .filter_map(|(i, block)| block.map(|b| i * b))
        .sum()
}
