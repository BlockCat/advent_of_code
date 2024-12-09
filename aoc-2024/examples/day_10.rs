use aoc_2024::{grid::StaticGrid, stopwatch, vector::Vector2};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::collections::{HashMap, HashSet};

type Input = Vec<Vec<char>>;

// wrong:

pub fn main() {
    let input = include_str!("../input/day_08.txt");
    let l = stopwatch(|| {
        let input = parse(input);
        let a1 = exercise_1(&input);
        println!("Ex1: {}", a1);

        // let a2 = exercise_2(&input);
        // println!("Ex2: {}", a2);
    });

    println!("Time: {:?}", l);
}

fn parse(input: &str) -> Input {
    input.lines().map(parse_line).collect()
}

fn parse_line(input: &str) -> Vec<char> {
    input.chars().collect()
}

fn exercise_1(input: &Input) -> usize {}

// fn exercise_2(input: &Input) -> usize {

// }
