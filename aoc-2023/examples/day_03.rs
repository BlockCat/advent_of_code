use aoc_2023::{grid::StaticGrid, vector::Vector2};
use std::collections::HashMap;

type InputType = StaticGrid<Option<Entry>>;

pub fn main() {
    let input = parse(include_str!("../input/day_03_big.txt"));
    println!("Exercise 1: {}", exercise_1(input.clone()));
    println!("Exercise 2: {}", exercise_2(input));
}

fn parse<'a>(input: &'a str) -> StaticGrid<Option<Entry>> {
    StaticGrid::from_vec(input.lines().map(|x| parse_line(x)).collect())
}

fn parse_line(line: &str) -> Vec<Option<Entry>> {
    let mut vec = Vec::new();
    let mut chars = line.chars();
    let mut local_vec = Vec::new();
    while let Some(c) = chars.next() {
        if c.is_ascii_digit() {
            local_vec.push(c.to_digit(10).unwrap());
        } else {
            if !local_vec.is_empty() {
                let number = local_vec.iter().fold(0, |a, b| a * 10 + b) as usize;
                vec.push(Some(Entry::Number(number)));

                for s in 1..local_vec.len() {
                    vec.push(Some(Entry::Reference(s, number)));
                }
            }
            local_vec.clear();

            if c == '.' {
                vec.push(None);
            } else {
                vec.push(Some(Entry::Symbol(c)));
            }
        }
    }

    if !local_vec.is_empty() {
        let number = local_vec.iter().fold(0, |a, b| a * 10 + b) as usize;
        vec.push(Some(Entry::Number(number)));

        for s in 1..local_vec.len() {
            vec.push(Some(Entry::Reference(s, number)));
        }
    }

    vec
}

fn exercise_1(input: InputType) -> usize {
    let res = input
        .iter()
        .filter(|s| {
            if let Some(Entry::Symbol(_)) = s.1 {
                true
            } else {
                false
            }
        })
        .flat_map(|(a, _)| input.get_neighbours_8(&a))
        .filter_map(|(v, s)| match s {
            Some(Entry::Number(a)) => Some((v, *a)),
            Some(Entry::Reference(offset, r)) => {
                let v = v - Vector2::new([(*offset as isize), 0]);
                Some((v, *r))
            }
            _ => None,
        })
        .collect::<HashMap<_, _>>();

    return res.iter().map(|s| s.1).sum();
}

fn exercise_2(input: InputType) -> usize {
    let res = input
        .iter()
        .filter(|s| s.1 == &Some(Entry::Symbol('*')))
        .map(|(a, _)| {
            let s = input
                .get_neighbours_8(&a)
                .into_iter()
                .filter_map(|(neighbour, value)| match value {
                    Some(Entry::Number(a)) => Some((neighbour, *a)),
                    Some(Entry::Reference(offset, r)) => {
                        let v = neighbour - Vector2::new([(*offset as isize), 0]);
                        Some((v, *r))
                    }
                    _ => None,
                })
                .collect::<HashMap<_, _>>()
                .into_iter()
                .collect::<Vec<_>>();

            if s.len() == 2 {
                s[0].1 * s[1].1
            } else {
                0
            }
        })
        .sum();

    return res;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Entry {
    Number(usize),
    Reference(usize, usize), //offset
    Symbol(char),
}
