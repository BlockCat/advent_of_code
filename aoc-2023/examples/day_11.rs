use aoc_2023::{
    grid::{Grid, StaticGrid},
    vector::Vector2,
};
use rayon::prelude::{ParallelBridge, ParallelIterator};
use std::collections::HashSet;

type InputType = StaticGrid<bool>;

pub fn main() {
    let input = parse(include_str!("../input/day_11.txt"));

    println!("Exercise 1: {}", exercise_1(input.clone(), 2));
    println!("Exercise 2: {}", exercise_1(input, 1000000));
}

fn parse<'a>(input: &'a str) -> InputType {
    StaticGrid::from_vec(input.lines().map(|line| parse_line(line)).collect())
}

fn parse_line(line: &str) -> Vec<bool> {
    line.chars().map(|s| s == '#').collect()
}

fn exercise_1(input: InputType, cost: usize) -> usize {
    let empty_columns = (0..input.width)
        .filter(|x| {
            (0..input.height).all(|y| input.get(*x as isize, y as isize).unwrap() == &false)
        })
        .collect::<HashSet<_>>();

    let empty_rows = (0..input.height)
        .filter(|y| (0..input.width).all(|x| input.get(x as isize, *y as isize).unwrap() == &false))
        .collect::<HashSet<_>>();

    let galaxies = find_galaxies(&input);

    (0..galaxies.len())
        .par_bridge()
        .map(|i| {
            shortest_path(
                galaxies[i],
                &galaxies[i + 1..],
                &empty_columns,
                &empty_rows,
                cost,
            )
        })
        .sum()
}

fn find_galaxies(galaxy: &InputType) -> Vec<Vector2> {
    galaxy
        .iter()
        .filter(|s| s.1 == &true)
        .map(|s| s.0.clone())
        .collect()
}

fn shortest_path(
    source: Vector2,
    destinations: &[Vector2],
    columns: &HashSet<usize>,
    rows: &HashSet<usize>,
    cost: usize,
) -> usize {
    destinations
        .iter()
        .map(|destination| {
            let cols = columns
                .iter()
                .filter(|s| {
                    (source[0].min(destination[0])..=source[0].max(destination[0]))
                        .contains(&(**s as isize))
                })
                .count();
            let rows = rows
                .iter()
                .filter(|s| {
                    (source[1].min(destination[1])..=source[1].max(destination[1]))
                        .contains(&(**s as isize))
                })
                .count();

            let dx = (destination[0] - source[0]).abs() as usize + (cost - 1) * cols;
            let dy = (destination[1] - source[1]).abs() as usize + (cost - 1) * rows;
            dx + dy
        })
        .sum()
}
