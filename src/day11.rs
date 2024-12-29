use std::{collections::HashMap, fs};

#[allow(dead_code)]
pub fn main() {
    let input = fs::read_to_string("inputs/day11.txt").expect("Unable to open file");
    let stones = parse_input(&input);

    let solution_1 = solve(&stones, 25);
    println!("{:?}", solution_1);
    let solution_2 = solve_memo(&stones, 75);
    println!("{:?}", solution_2);
}

fn parse_input(input: &str) -> Vec<u64> {
    input
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect()
}

fn solve_memo(stones: &[u64], blinks: usize) -> usize {
    let mut cache = HashMap::new();
    stones
        .iter()
        .map(|&stone| blink_memo(stone, blinks, &mut cache))
        .sum()
}

fn blink_memo(stone: u64, blinks: usize, cache: &mut HashMap<(u64, usize), usize>) -> usize {
    if blinks == 0 {
        return 1;
    }

    if let Some(&result) = cache.get(&(stone, blinks)) {
        return result;
    }

    let stone_str = stone.to_string();
    let result = if stone == 0 {
        blink_memo(1, blinks - 1, cache)
    } else if stone_str.len() % 2 == 0 {
        let (fst, snd) = stone_str.split_at(stone_str.len() / 2);
        blink_memo(fst.parse().unwrap(), blinks - 1, cache)
            + blink_memo(snd.parse().unwrap(), blinks - 1, cache)
    } else {
        blink_memo(stone * 2024, blinks - 1, cache)
    };

    cache.insert((stone, blinks), result);
    result
}

fn solve(stones: &[u64], blinks: usize) -> usize {
    (0..blinks)
        .fold(stones.to_vec(), |acc, _| blink(&acc))
        .len()
}

fn blink(stones: &[u64]) -> Vec<u64> {
    stones
        .iter()
        .flat_map(|&stone| {
            let stone_str = stone.to_string();
            if stone == 0 {
                vec![1]
            } else if stone_str.len() % 2 == 0 {
                let (fst, snd) = stone_str.split_at(stone_str.len() / 2);
                vec![fst.parse().unwrap(), snd.parse().unwrap()]
            } else {
                vec![stone * 2024]
            }
        })
        .collect()
}
