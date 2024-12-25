use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    fs, isize, usize,
};

#[allow(dead_code)]
pub fn main() {
    let input = fs::read_to_string("inputs/day8.txt").expect("Unable to open file");
    let grid = parse_grid(&input);
    let antennas = parse_antennas(&grid);

    let solution_1 = solve_1(&grid, &antennas);
    let solution_2 = solve_2(&grid, &antennas);
    println!("{}", solution_1);
    println!("{}", solution_2);
}

fn parse_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn parse_antennas(grid: &[Vec<char>]) -> HashMap<char, Vec<(usize, usize)>> {
    grid.iter()
        .enumerate()
        .flat_map(|(row, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, &c)| c != '.')
                .map(move |(col, &c)| (c, (row, col)))
        })
        .fold(HashMap::new(), |mut acc, (c, pos)| {
            acc.entry(c).or_insert(Vec::new()).push(pos);
            acc
        })
}

fn solve_1(grid: &[Vec<char>], antennas: &HashMap<char, Vec<(usize, usize)>>) -> usize {
    let mut antinodes = HashSet::new();
    for positions in antennas.values() {
        for pair in positions.iter().permutations(2) {
            let (from, to) = (pair[0], pair[1]);
            if let Some(antinode) = find_antinode(grid, *from, *to) {
                antinodes.insert(antinode);
            }
        }
    }
    antinodes.len()
}

fn solve_2(grid: &[Vec<char>], antennas: &HashMap<char, Vec<(usize, usize)>>) -> usize {
    let mut antinodes = HashSet::new();
    for positions in antennas.values() {
        for pair in positions.iter().permutations(2) {
            let (from, to) = (pair[0], pair[1]);
            if let Some(antinodes_) = find_antinodes(grid, *from, *to) {
                for antinode in antinodes_ {
                    antinodes.insert(antinode);
                }
            }
        }
    }
    antinodes.len()
}

fn find_antinode(
    grid: &[Vec<char>],
    from: (usize, usize),
    to: (usize, usize),
) -> Option<(usize, usize)> {
    let (from_row, from_col) = from;
    let (to_row, to_col) = to;

    let (antinode_row, antinode_col) = (
        from_row as isize - (to_row as isize - from_row as isize),
        from_col as isize - (to_col as isize - from_col as isize),
    );

    if in_bounds(antinode_row, antinode_col, grid) {
        Some((antinode_row as usize, antinode_col as usize))
    } else {
        None
    }
}

fn find_antinodes(
    grid: &[Vec<char>],
    from: (usize, usize),
    to: (usize, usize),
) -> Option<Vec<(usize, usize)>> {
    let (from_row, from_col) = from;
    let (to_row, to_col) = to;

    let mut antinodes = Vec::new();
    let mut current_row = from_row as isize;
    let mut current_col = from_col as isize;

    let row_step = from_row as isize - to_row as isize;
    let col_step = from_col as isize - to_col as isize;

    while in_bounds(current_row, current_col, grid) {
        antinodes.push((current_row as usize, current_col as usize));
        current_row -= row_step;
        current_col -= col_step;
    }

    if antinodes.is_empty() {
        None
    } else {
        Some(antinodes)
    }
}

fn in_bounds(row: isize, col: isize, grid: &[Vec<char>]) -> bool {
    row >= 0 && col >= 0 && row < grid.len() as isize && col < grid[0].len() as isize
}
