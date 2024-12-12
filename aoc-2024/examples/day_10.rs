use aoc_2024::{
    grid::{Grid, StaticGrid},
    stopwatch,
    vector::Vector2,
};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::collections::{HashMap, HashSet, VecDeque};

type Input = StaticGrid<u8>;

// wrong:

pub fn main() {
    let input = include_str!("../input/day_10.txt");
    // let input = include_str!("../input/test.txt");
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

fn parse_line(input: &str) -> Vec<u8> {
    input.chars().map(|x| x as u8 - b'0').collect()
}

fn exercise_1(input: &Input) -> usize {
    input
        .iter()
        .filter(|x| x.1 == &0)
        .map(|x| x.0)
        .map(|x| can_go_to_9(input, x))
        .sum()
}

fn exercise_2(input: &Input) -> usize {
    input
        .iter()
        .filter(|x| x.1 == &0)
        .map(|x| x.0)
        .map(|x| can_go_to_9_rating(input, x))
        .sum()
}

fn can_go_to_9(input: &Input, start: Vector2) -> usize {
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));

    let mut visited = HashSet::new();
    let mut count = 0;

    while let Some((pos, level)) = queue.pop_back() {
        if !visited.insert(pos) {
            continue;
        }

        if level == 9 {
            count += 1;
        }
 
        for n in pos.neighbours_4() {
            if let Some(nl) = input.get_vec(&n) {
                if *nl == level + 1 {
                    // println!("pos: {:?}, nl: {}", n, nl);
                    queue.push_back((n, *nl));
                }
            }
        }
    }
    count
}

fn can_go_to_9_rating(input: &Input, start: Vector2) -> usize {
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));

    let mut count = 0;

    while let Some((pos, level)) = queue.pop_back() {
        if level == 9 {
            count += 1;
        }

        for n in pos.neighbours_4() {
            if let Some(nl) = input.get_vec(&n) {
                if *nl == level + 1 {
                    // println!("pos: {:?}, nl: {}", n, nl);
                    queue.push_back((n, *nl));
                }
            }
        }
    }
    count
}

// fn exercise_2(input: &Input) -> usize {

// }
