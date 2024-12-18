use std::{collections::HashSet, fs, usize};

#[allow(dead_code)]

pub fn main() {
    let input = fs::read_to_string("inputs/test.txt").expect("Unable to open file");
    let lab_maze = parse_lab(&input);
    let guard = locate_guard(&lab_maze);

    let solution = solve(&lab_maze, &guard);
    println!("{}", solution);
}

fn parse_lab(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn locate_guard(lab_maze: &[Vec<char>]) -> (usize, usize) {
    lab_maze
        .iter()
        .enumerate()
        .find_map(|(row, cols)| cols.iter().position(|&c| c == '^').map(|col| (row, col)))
        .expect("Guard is not in the map")
}

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_90_degrees(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn move_forward(self, row: usize, col: usize) -> (usize, usize) {
        match self {
            Direction::Up => (row - 1, col),
            Direction::Right => (row, col + 1),
            Direction::Down => (row + 1, col),
            Direction::Left => (row, col - 1),
        }
    }
}

fn solve(lab_maze: &[Vec<char>], guard: &(usize, usize)) -> usize {
    let (mut row, mut col) = guard.to_owned();
    let mut cur_dir = Direction::Up;
    let mut visited = HashSet::from([(row, col)]);

    loop {
        let (next_row, next_col) = cur_dir.move_forward(row, col);

        if next_row >= lab_maze.len() || next_col >= lab_maze[0].len() {
            break;
        }

        if lab_maze[next_row][next_col] == '#' {
            cur_dir = cur_dir.turn_90_degrees();
        } else {
            row = next_row;
            col = next_col;
            visited.insert((row, col));
        }
    }

    visited.len()
}
