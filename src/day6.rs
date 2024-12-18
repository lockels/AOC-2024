use std::{collections::HashSet, fs};

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

#[derive(Debug, Clone, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}


fn solve(lab_maze: &[Vec<char>], guard: &(usize, usize)) -> u32 {
    let (mut row, mut col) = guard.to_owned();
    let mut cur_dir = Direction::Up;

    let mut visited = HashSet::from([(row, col)]);

    while row < lab_maze.len() && col < lab_maze[0].len() {
        match cur_dir {
            Direction::Up => {
                if row == 0 {
                    break;
                } else if lab_maze[row - 1][col] == '#' {
                    cur_dir = Direction::Right;
                } else {
                    row -= 1;
                    visited.insert((row, col));
                }
            }
            Direction::Right => {
                if col + 1 >= lab_maze[0].len() {
                    break;
                } else if lab_maze[row][col + 1] == '#' {
                    cur_dir = Direction::Down;
                } else {
                    col += 1;
                    visited.insert((row, col));
                }
            }
            Direction::Down => {
                if row + 1 >= lab_maze.len() {
                    break;
                } else if lab_maze[row + 1][col] == '#' {
                    cur_dir = Direction::Left;
                } else {
                    row += 1;
                    visited.insert((row, col));
                }
            }
            Direction::Left => {
                if col == 0 {
                    break;
                } else if lab_maze[row][col - 1] == '#' {
                    cur_dir = Direction::Up;
                } else {
                    col -= 1;
                    visited.insert((row, col));
                }
            }
        }
    }

    visited.len() as u32
}

