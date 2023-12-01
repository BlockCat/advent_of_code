use std::collections::{HashSet, VecDeque};

use aoc_2022::{
    direction::ALL_DIRECTIONS,
    grid::{Grid, StaticGrid},
};

type InputType = StaticGrid<char>;

pub fn main() {
    let numbers = input();

    println!("Exercise 1: {}", exercise_1(numbers.clone()));
    println!("Exercise 2: {}", exercise_2(numbers));
}

fn input() -> InputType {
    StaticGrid::from_vec(
        include_str!("../input/day_12.txt")
            .lines()
            .map(parse_line)
            .collect(),
    )
}

fn parse_line(line: &str) -> Vec<char> {
    line.chars().collect()
}

fn exercise_1(input: InputType) -> usize {
    let start = input.iter().find(|x| x.1 == &'S').unwrap();
    let end = input.iter().find(|x| x.1 == &'E').unwrap();

    let mut visited = HashSet::new();

    let mut stack = VecDeque::new();
    stack.push_back((start.0, 'a', 0usize));

    while let Some((pos, elev, len)) = stack.pop_front() {
        if !visited.insert(pos) {
            continue;
        }

        if pos == end.0 {
            return len;
        }

        for next in ALL_DIRECTIONS
            .iter()
            .filter_map(|x| {
                let next_pos = pos + *x;
                input.get_vec(&next_pos).map(|np| (next_pos, np))
            })
            .filter(|(_, nele)| **nele as i16 - elev as i16 <= 1)
        {
            stack.push_back((next.0, *next.1, len + 1));
        }
    }

    unreachable!()
}
fn exercise_2(input: InputType) -> usize {
    let start = input.iter().filter(|x| x.1 == &'S' || x.1 == &'a');
    let end = input.iter().find(|x| x.1 == &'E').unwrap();

    let mut visited = HashSet::new();

    let mut stack = VecDeque::new();

    for (start, ele) in start {
        stack.push_front((start, *ele, 0usize));
    }

    while let Some((pos, elev, len)) = stack.pop_front() {
        if !visited.insert(pos) {
            continue;
        }

        if pos == end.0 {
            return len;
        }

        for next in ALL_DIRECTIONS
            .iter()
            .filter_map(|x| {
                let next_pos = pos + *x;
                input.get_vec(&next_pos).map(|np| (next_pos, np))
            })
            .filter(|(_, nele)| **nele as i16 - elev as i16 <= 1)
        {
            stack.push_back((next.0, *next.1, len + 1));
        }
    }

    unreachable!()
}
