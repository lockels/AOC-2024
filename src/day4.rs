use std::{char, fs, usize};

#[allow(dead_code)]

pub fn main() {
    let input = fs::read_to_string("inputs/day4.txt").expect("Unable to open file");
    let puzzle = parse_input(&input);

    let solution = solve(&puzzle, "XMAS");
    println!("{}", solution);
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn solve(puzzle: &[Vec<char>], word: &str) -> i32 {
    let directions = [
        (0, 1),   // Right
        (0, -1),  // Left
        (1, 0),   // Down
        (-1, 0),  // Up
        (1, 1),   // Diagonal down right
        (1, -1),  // Diagonal down left
        (-1, 1),  // Diagonal up-right
        (-1, -1), // Diagonal up-left
    ];

    let rows = puzzle.len() as usize;
    let cols = puzzle[0].len() as usize;

    let mut count = 0;

    for row in 0..rows {
        for col in 0..cols {
            for &(dx, dy) in &directions {
                if found_word_in_cur_direction(word, puzzle, row, col, dx, dy) {
                    count += 1;
                }
            }
        }
    }

    count
}

fn found_word_in_cur_direction(
    word: &str,
    puzzle: &[Vec<char>],
    row: usize,
    col: usize,
    dx: i32,
    dy: i32,
) -> bool {
    let word_chars: Vec<char> = word.chars().collect();
    let word_len = word_chars.len();

    for i in 0..word_len {
        let cur_row = row as i32 + i as i32 * dx;
        let cur_col = col as i32 + i as i32 * dy;

        if cur_row < 0 || cur_col < 0 {
            return false;
        }

        let cur_row = cur_row as usize;
        let cur_col = cur_col as usize;

        if cur_row >= puzzle.len() || cur_col >= puzzle[0].len() {
            return false;
        }

        if puzzle[cur_row][cur_col] != word_chars[i] {
            return false;
        }
    }

    true
}
