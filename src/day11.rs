use std::fs;

pub fn main() {
    let input = fs::read_to_string("inputs/day11.txt").expect("Unable to open file");
    let stones = parse_input(&input);

    let solution = solve(&stones);
    println!("{:?}", solution);
}

fn parse_input(input: &str) -> Vec<u64> {
    input
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect()
}

fn solve(stones: &[u64]) -> usize {
    (0..75)
        .fold(stones.to_vec(), |acc, _| blink_once(&acc))
        .len()
}

fn blink_once(stones: &[u64]) -> Vec<u64> {
    let mut result = Vec::new();

    for stone in stones {
        if *stone == 0 {
            result.push(1);
        } else if stone.to_string().len() % 2 == 0 {
            let stone_str = stone.to_string();
            let mid = stone_str.len() / 2;
            let (fst_str, snd_str) = stone_str.split_at(mid);
            let (fst, snd) = (
                fst_str.parse::<u64>().unwrap(),
                snd_str.parse::<u64>().unwrap(),
            );

            result.push(fst);
            result.push(snd);
        } else {
            result.push(stone * 2024);
        }
    }

    result
}
