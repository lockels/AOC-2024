use std::{char, fs, usize};

#[allow(dead_code)]

pub fn main() {
    let input = fs::read_to_string("inputs/day4.txt").expect("Unable to open file");
    let puzzle = parse_input(&input);

    // let solution1 = solve_1(&puzzle, "XMAS");
    // println!("{}", solution1);

    let solution2 = solve_2(&puzzle);
    println!("{}", solution2);
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn solve_1(puzzle: &[Vec<char>], word: &str) -> i32 {
    let dirs = [
        (0, 1),   // Right
        (0, -1),  // Left
        (1, 0),   // Down
        (-1, 0),  // Up
        (1, 1),   // Diagonal down right
        (1, -1),  // Diagonal down left
        (-1, 1),  // Diagonal up-right
        (-1, -1), // Diagonal up-left
    ];

    let mut count = 0;

    for row in 0..puzzle.len() {
        for col in 0..puzzle[0].len() {
            for &(dx, dy) in &dirs {
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

    for i in 0..word.len() {
        let cur_row = row as i32 + i as i32 * dx;
        let cur_col = col as i32 + i as i32 * dy;

        if cur_row < 0 || cur_col < 0 {
            return false;
        }

        let cur_row = cur_row as usize;
        let cur_col = cur_col as usize;

        if cur_row >= puzzle.len() || cur_col >= puzzle.len() {
            return false;
        }

        if puzzle[cur_row][cur_col] != word_chars[i] {
            return false;
        }
    }

    true
}

fn solve_2(puzzle: &[Vec<char>]) -> i32 {
    let dirs = [
        (-1, -1), // Top left
        (-1, 1),  // Top right
        (1, -1),  // Bottom left
        (1, 1),   // Bottom right
    ];

    let mut count = 0;

    for row in 1..puzzle.len() - 1 {
        for col in 1..puzzle[0].len() - 1 {
            if puzzle[row][col] == 'A' {
                let mut s: String = String::default();
                for &(dx, dy) in &dirs {
                    let new_row = row as isize + dx;
                    let new_col = col as isize + dy;
                    s.push(puzzle[new_row as usize][new_col as usize]);
                }

                if s == "MSMS" || s == "SSMM" || s == "MMSS" || s == "SMSM" {
                    count += 1;
                }

                println!("{}", s);
            }
        }
    }

    count
}
