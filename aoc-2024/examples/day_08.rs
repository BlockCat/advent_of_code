use aoc_2024::{grid::StaticGrid, stopwatch, vector::Vector2};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::collections::{HashMap, HashSet};

type Input = StaticGrid<char>;

// wrong:

pub fn main() {
    let input = include_str!("../input/day_08.txt");
    let l = stopwatch(|| {
        let input = parse(input);
        let a1 = exercise_1(&input);
        println!("Ex1: {}", a1);

        let a2 = exercise_2(&input);
        println!("Ex2: {}", a2);
    });

    println!("Time: {:?}", l);
}

fn parse(input: &str) -> Input {
    StaticGrid::from_vec(input.lines().map(parse_line).collect())
}

fn parse_line(input: &str) -> Vec<char> {
    input.chars().collect()
}

fn exercise_1(input: &Input) -> usize {
    let frequency_map = get_frequency_map(input);

    let mut antinodes = HashSet::new();

    frequency_map
        .iter()
        .filter(|(_, pos)| pos.len() > 1)
        .for_each(|(_, pos)| {
            for &p1 in pos {
                for &p2 in pos {
                    if p1 == p2 {
                        continue;
                    }
                    let diff = p2 - p1;
                    let pos = p2 + diff;
                    if input.in_bounds(pos) {
                        antinodes.insert(pos);
                    }
                }
            }
        });

    antinodes.len()
}

fn exercise_2(input: &Input) -> usize {
    let frequency_map = get_frequency_map(input);

    let mut antinodes = HashSet::new();
    frequency_map
        .iter()
        .filter(|(_, pos)| pos.len() > 1)
        .for_each(|(_, pos)| {
            for &p1 in pos {
                for &p2 in pos {
                    if p1 == p2 {
                        continue;
                    }
                    let diff = p2 - p1;

                    let mut pos = p2;

                    while input.in_bounds(pos) {
                        antinodes.insert(pos);
                        pos += diff;
                    }
                }
            }
        });

    antinodes.len()
}

fn get_frequency_map(input: &StaticGrid<char>) -> HashMap<char, Vec<aoc_2024::vector::VectorN<2>>> {
    let mut frequency_map = HashMap::<char, Vec<Vector2>>::new();
    input
        .iter()
        .filter(|x| *x.1 != '.')
        .for_each(|(pos, freq)| {
            frequency_map.entry(*freq).or_default().push(pos);
        });
    frequency_map
}
