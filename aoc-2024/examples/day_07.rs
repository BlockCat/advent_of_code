use aoc_2024::{
    direction::Direction,
    grid::{DynamicGrid, Grid, StaticGrid},
    stopwatch,
    vector::Vector2,
};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    hash::Hash,
};

type Input = HashMap<u64, Vec<u64>>;

// wrong: 2151

pub fn main() {
    let input = include_str!("../input/day_07.txt");
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
    input.lines().map(parse_line).collect()
}

fn parse_line(input: &str) -> (u64, Vec<u64>) {
    let (l, r) = input.split_once(": ").unwrap();
    let l = l.parse().unwrap();
    let r = r.split_whitespace().map(|x| x.parse().unwrap()).collect();
    (l, r)
}

fn exercise_1(input: &Input) -> u64 {
    input
        .iter()
        .filter(|(a, b)| is_ok(**a, *b))
        .map(|(a, b)| *a)
        .sum()
}

fn exercise_2(input: &Input) -> u64 {
    input
        .iter()
        .filter(|(a, b)| is_ok2(**a, *b))
        .map(|(a, b)| *a)
        .sum()
}

fn is_ok(result: u64, line: &Vec<u64>) -> bool {
    let mut is_k_mul = false;
    let mut is_k_add = false;
    rayon::scope(|s| {
        s.spawn(|_| is_k_mul = is_ok_mul(result, line[0], &line[1..]));
        s.spawn(|_| is_k_add = is_ok_add(result, line[0], &line[1..]));
    });

    is_k_mul || is_k_add
}
fn is_ok2(result: u64, line: &Vec<u64>) -> bool {
    let mut is_k_mul = false;
    let mut is_k_add = false;
    let mut is_k_concat = false;
    rayon::scope(|s| {
        s.spawn(|_| is_k_mul = is_ok_mul2(result, line[0], &line[1..]));
        s.spawn(|_| is_k_add = is_ok_add2(result, line[0], &line[1..]));
        s.spawn(|_| is_k_concat = is_ok_concat2(result, line[0], &line[1..]));
    });

    is_k_mul || is_k_add || is_k_concat
}

fn is_ok_mul(result: u64, acc: u64, line: &[u64]) -> bool {
    if line.is_empty() {
        return acc == result;
    }

    let new_acc = acc * line[0];

    if new_acc > result {
        return false;
    }

    is_ok_mul(result, new_acc, &line[1..]) || is_ok_add(result, new_acc, &line[1..])
}

fn is_ok_add(result: u64, acc: u64, line: &[u64]) -> bool {
    if line.is_empty() {
        return acc == result;
    }

    let new_acc = acc + line[0];

    if new_acc > result {
        return false;
    }

    is_ok_mul(result, new_acc, &line[1..]) || is_ok_add(result, new_acc, &line[1..])
}

fn is_ok_mul2(result: u64, acc: u64, line: &[u64]) -> bool {
    if line.is_empty() {
        return acc == result;
    }

    let new_acc = acc * line[0];

    if new_acc > result {
        return false;
    }

    is_ok_mul2(result, new_acc, &line[1..])
        || is_ok_add2(result, new_acc, &line[1..])
        || is_ok_concat2(result, new_acc, &line[1..])
}

fn is_ok_add2(result: u64, acc: u64, line: &[u64]) -> bool {
    if line.is_empty() {
        return acc == result;
    }

    let new_acc = acc + line[0];

    if new_acc > result {
        return false;
    }

    is_ok_mul2(result, new_acc, &line[1..])
        || is_ok_add2(result, new_acc, &line[1..])
        || is_ok_concat2(result, new_acc, &line[1..])
}

fn is_ok_concat2(result: u64, acc: u64, line: &[u64]) -> bool {
    if line.is_empty() {
        return acc == result;
    }

    let count_tenths = line[0].ilog10() + 1;
    let new_acc = (acc * 10u64.pow(count_tenths)) + line[0];
    if new_acc > result {
        return false;
    }

    is_ok_mul2(result, new_acc, &line[1..])
        || is_ok_add2(result, new_acc, &line[1..])
        || is_ok_concat2(result, new_acc, &line[1..])
}
