use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    fs, usize,
};

#[allow(dead_code)]
pub fn main() {
    let input = fs::read_to_string("inputs/day8.txt").expect("Unable to open file");
    let antennas = parse_antennas(&input);
    let grid = parse_grid(&input);

    let solution = solve(&grid, &antennas);
    println!("{}", solution);
}

fn parse_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn parse_antennas(input: &str) -> HashMap<char, Vec<(usize, usize)>> {
    input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c != '.' && *c != '\n')
                .map(move |(col, c)| (c, (row, col)))
        })
        .fold(HashMap::new(), |mut acc, (c, pos)| {
            acc.entry(c).or_insert(Vec::new()).push(pos);
            acc
        })
}

fn solve(grid: &[Vec<char>], antennas: &HashMap<char, Vec<(usize, usize)>>) -> usize {
    let antinodes: HashSet<(usize, usize)> = antennas
        .values()
        .flat_map(|positions| positions.iter().permutations(2))
        .filter_map(|pair| {
            let (from, to) = (pair[0], pair[1]);
            find_antinode(grid, *from, *to)
        })
        .collect();
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

fn in_bounds(row: isize, col: isize, grid: &[Vec<char>]) -> bool {
    row >= 0 && col >= 0 && row < grid.len() as isize && col < grid[0].len() as isize
}
