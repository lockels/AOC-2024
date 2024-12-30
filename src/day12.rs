use std::{collections::HashSet, fs, usize};

#[allow(dead_code)]
pub fn main() {
    let input = fs::read_to_string("inputs/day12.txt").expect("Unable to open file");
    let garden = parse_garden(&input);
    let solution = solve(&garden);
    println!("{}", solution);
}

fn parse_garden(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

const DIRECTIONS: [(i32, i32); 4] = [
    (-1, 0), // Up
    (1, 0),  // Down
    (0, -1), // Left
    (0, 1),  // Right
];

fn solve(garden: &[Vec<char>]) -> usize {
    let mut visited = HashSet::new();
    let mut sum = 0;

    for i in 0..garden.len() {
        for j in 0..garden[0].len() {
            if !visited.contains(&(i, j)) {
                let (area, perimeter) =
                    calculate_geometry_1(garden, (i, j), garden[i][j], &mut visited);
                sum += area * perimeter;
            }
        }
    }

    sum
}

fn calculate_geometry_1(
    garden: &[Vec<char>],
    current: (usize, usize),
    plot_type: char,
    visited: &mut HashSet<(usize, usize)>,
) -> (usize, usize) {
    let (row, col) = current;

    if visited.contains(&current) || garden[row][col] != plot_type {
        return (0, 0);
    }

    visited.insert(current);
    let (mut area, mut perimeter) = (1, 0);

    for (dx, dy) in DIRECTIONS {
        let (next_row, next_col) = (row as i32 + dx, col as i32 + dy);
        let neighbour = (next_row as usize, next_col as usize);

        if !in_bounds(garden, next_row, next_col) || garden[neighbour.0][neighbour.1] != plot_type {
            perimeter += 1;
        } else if !visited.contains(&neighbour) {
            let (sub_area, sub_perimeter) =
                calculate_geometry_1(garden, neighbour, plot_type, visited);
            area += sub_area;
            perimeter += sub_perimeter;
        }
    }

    (area, perimeter)
}

fn in_bounds(garden: &[Vec<char>], row: i32, col: i32) -> bool {
    row >= 0 && col >= 0 && row < garden.len() as i32 && col < garden[0].len() as i32
}
