use std::{collections::HashSet, fs};

#[allow(dead_code)]
pub fn main() {
    let input = fs::read_to_string("inputs/test.txt").expect("Unable to open file");
    let garden = parse_garden(&input);

    let solution_1 = solve(&garden, calculate_geometry_1);
    println!("{}", solution_1);

    let solution_2 = solve(&garden, calculate_geometry_2);
    println!("{}", solution_2);
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

fn solve(
    garden: &[Vec<char>],
    geometry_function: fn(
        &[Vec<char>],
        (usize, usize),
        char,
        &mut HashSet<(usize, usize)>,
    ) -> (usize, usize),
) -> usize {
    let mut visited = HashSet::new();
    (0..garden.len())
        .flat_map(|i| (0..garden[0].len()).map(move |j| (i, j)))
        .map(|cur| geometry_function(garden, cur, garden[cur.0][cur.1], &mut visited))
        .map(|(area, perimeter)| area * perimeter)
        .sum()
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

fn calculate_geometry_2(
    garden: &[Vec<char>],
    current: (usize, usize),
    plot_type: char,
    visited: &mut HashSet<(usize, usize)>,
) -> (usize, usize) {
    let (row, col) = current;

    if visited.contains(&current) || garden[row][col] != plot_type {
        return (0, 0);
    }

    let (mut area, mut perimeter) = (1, 0);
    visited.insert(current);

    for (dx, dy) in DIRECTIONS {
        let (next_row, next_col) = (row as i32 + dx, col as i32 + dy);
        let neighbour = (next_row as usize, next_col as usize);

        if !in_bounds(garden, next_row, next_col) {
            continue;
        }

        if !visited.contains(&neighbour) {
            let (sub_area, sub_perimeter) =
                calculate_geometry_2(garden, neighbour, plot_type, visited);
            area += sub_area;
            perimeter += sub_perimeter;
        }

        let (next_row, next_col) = (next_row as usize, next_col as usize);

        if in_bounds(garden, next_row as i32 + 1, next_col as i32 + 1) {
            let sub_grid = [
                garden[next_row][next_col],         // top left
                garden[next_row][next_col + 1],     // top right
                garden[next_row + 1][next_col],     // bottom left
                garden[next_row + 1][next_col + 1], // bottom right
            ];

            let mut match_count = 0;
            for i in 0..4 {
                for j in i + 1..4 {
                    if sub_grid[i] == sub_grid[j] {
                        match_count += 1;
                    }
                }
            }

            if match_count == 3 {
                perimeter += 1;
            }
        }
    }

    (area, perimeter)
}

fn in_bounds(garden: &[Vec<char>], row: i32, col: i32) -> bool {
    row >= 0 && col >= 0 && row < garden.len() as i32 && col < garden[0].len() as i32
}
