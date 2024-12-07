use std::{
    collections::{HashMap, HashSet, VecDeque},
    hash::Hash,
};

use aoc_2024::stopwatch;

type Input = (Vec<(u64, u64)>, Vec<Vec<u64>>);

pub fn main() {
    let input = include_str!("../input/day_05.txt");
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
    let (a, b) = input.split_once("\n\n").unwrap();

    let a = a.lines().map(parse_line).collect::<Vec<_>>();
    let b = b.lines().map(parse_line2).collect::<Vec<_>>();
    (a, b)
}

fn parse_line(input: &str) -> (u64, u64) {
    let (l, r) = input.split_once("|").unwrap();

    (l.parse().unwrap(), r.parse().unwrap())
}
fn parse_line2(input: &str) -> Vec<u64> {
    input.split(",").map(|x| x.parse().unwrap()).collect()
}

fn exercise_1(input: &Input) -> u64 {
    let mut map = HashMap::<u64, HashSet<_>>::new();
    input.0.iter().for_each(|(a, b)| {
        map.entry(*b).or_default().insert(*a);
    });

    input
        .1
        .iter()
        .filter(|x| is_valid(*x, &map))
        .map(|s| s[s.len() / 2])
        .sum()
}

fn is_valid(line: &[u64], map: &HashMap<u64, HashSet<u64>>) -> bool {
    for i in 0..line.len() {
        let current = line[i];
        let remaining = &line[i + 1..];

        if let Some(must_be_before) = map.get(&current) {
            for r in remaining {
                if must_be_before.contains(r) {
                    return false;
                }
            }
        }
    }

    true
}

fn exercise_2(input: &Input) -> u64 {
    let mut map = HashMap::<u64, HashSet<_>>::new();
    input.0.iter().for_each(|(a, b)| {
        map.entry(*b).or_default().insert(*a);
    });

    input
        .1
        .iter()
        .filter(|x| !is_valid(*x, &map))
        .map(|x| sort(x, &map))
        .map(|x| x[x.len() / 2])
        .sum()
}

fn sort(line: &[u64], map: &HashMap<u64, HashSet<u64>>) -> Vec<u64> {
    let mut new_line = vec![];
    let mut heap = VecDeque::from_iter(line.iter().copied());

    'a: while let Some(current) = heap.pop_front() {
        if let Some(must_be_before) = map.get(&current) {
            for r in heap.clone() {
                if must_be_before.contains(&r) {
                    heap.push_back(current);
                    continue 'a;
                }
            }
        }
        new_line.push(current);
    }

    new_line
}
