use std::{collections::HashSet, fs};

#[allow(dead_code)]

pub fn main() {
    let input = fs::read_to_string("inputs/test.txt").expect("Unable to open file");
    let lab_maze = parse_lab(&input);

    let guard = locate_guard(&lab_maze);
    println!("{:?}", guard);

    let solution_1 = solve_1(&lab_maze);
    println!("{}", solution_1.len());

    let solution_2 = solve_2(&lab_maze);
    println!("{}", solution_2.len());
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

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
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
            Direction::Up => (row.checked_sub(1).unwrap_or(row), col),
            Direction::Right => (row, col + 1),
            Direction::Down => (row + 1, col),
            Direction::Left => (row, col.checked_sub(1).unwrap_or(col)),
        }
    }
}

fn solve_1(lab_maze: &[Vec<char>]) -> HashSet<(usize, usize)> {
    let (mut row, mut col) = locate_guard(&lab_maze);
    let mut visited = HashSet::from([(row, col)]);

    let mut cur_dir = Direction::Up;
    loop {
        let (next_row, next_col) = cur_dir.move_forward(row, col);

        if next_row >= lab_maze.len()
            || next_col >= lab_maze[0].len()
            || next_row == usize::MAX
            || next_col == usize::MAX
        {
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

    visited
}

fn solve_2(lab_maze: &[Vec<char>]) -> HashSet<(usize, usize)> {
    let mut result = HashSet::new();

    for (i, row) in lab_maze.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            if obstacle_causes_loop(lab_maze, (i, j)) {
                result.insert((i, j));
            }
        }
    }

    result
}


fn obstacle_causes_loop(lab_maze: &[Vec<char>], pos: (usize, usize)) -> bool {
    let (mut guard_row, mut guard_col) = locate_guard(&lab_maze);
    if pos == (guard_row, guard_col) {
        return false;
    }

    let mut lab_maze = lab_maze.to_vec();
    lab_maze[pos.0][pos.1] = '#';

    let mut cur_dir = Direction::Up;
    let mut visited = HashSet::new();
    visited.insert((guard_row, guard_col, cur_dir));

    loop {
        let (next_row, next_col) = cur_dir.move_forward(guard_row, guard_col);

        if visited.contains(&(guard_row, guard_col, cur_dir)) {
            return true;
        }

        if next_row >= lab_maze.len()
            || next_col >= lab_maze[0].len()
            || next_row == usize::MAX
            || next_col == usize::MAX
        {
            return false;
        }

        if lab_maze[next_row][next_col] == '#' {
            cur_dir = cur_dir.turn_90_degrees();
        } else {
            guard_row = next_row;
            guard_col = next_col;
            visited.insert((guard_row, guard_col, cur_dir));
        }
    }
}

