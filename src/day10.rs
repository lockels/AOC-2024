use std::{collections::HashSet, fs};

#[allow(dead_code)]

pub fn main() {
    let input = fs::read_to_string("inputs/day10.txt").expect("Unable to read file");

    let grid = parse_input(&input);
    let start_points = find_nodes(&grid, '0');
    let end_points = find_nodes(&grid, '9');

    let solution = solve(&grid, &start_points, &end_points);
    println!("{}", solution);
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn find_nodes(grid: &[Vec<char>], node: char) -> Vec<(usize, usize)> {
    grid.iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(j, &cell)| if cell == node { Some((i, j)) } else { None })
        })
        .collect()
}

fn solve(
    grid: &[Vec<char>],
    start_points: &[(usize, usize)],
    end_points: &[(usize, usize)],
) -> u32 {
    start_points
        .iter()
        .flat_map(|&start| end_points.iter().map(move |&end| (start, end)))
        .filter(|&(start, end)| find_path(grid, start, end))
        .count() as u32
}

fn find_path(grid: &[Vec<char>], start: (usize, usize), end: (usize, usize)) -> bool {
    let directions = [
        (-1, 0), // Up
        (1, 0),  // Down
        (0, -1), // Left
        (0, 1),  // Right
    ];
    let mut visited = HashSet::new();
    find_path_dfs(grid, start, end, &mut visited, &directions)
}

fn find_path_dfs(
    grid: &[Vec<char>],
    current: (usize, usize),
    target: (usize, usize),
    visited: &mut HashSet<(usize, usize)>,
    directions: &[(i32, i32)],
) -> bool {
    if current == target {
        return true;
    }

    if !visited.insert(current) {
        return false;
    }

    for (dx, dy) in directions {
        let next_row = current.0 as i32 + dx;
        let next_col = current.1 as i32 + dy;

        if in_bounds(grid, next_row, next_col) {
            let next = (next_row as usize, next_col as usize);

            if grid[next.0][next.1] == '.' {
                continue;
            }

            let (current_val, next_val) = (
                grid[current.0][current.1].to_digit(10).unwrap(),
                grid[next.0][next.1].to_digit(10).unwrap(),
            );

            if next_val == current_val + 1 {
                if find_path_dfs(grid, next, target, visited, directions) {
                    return true;
                }
            }
        }
    }

    false
}

fn in_bounds(grid: &[Vec<char>], row: i32, col: i32) -> bool {
    row >= 0 && col >= 0 && row < grid.len() as i32 && col < grid[0].len() as i32
}
