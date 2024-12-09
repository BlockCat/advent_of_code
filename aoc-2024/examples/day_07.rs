use aoc_2024::stopwatch;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::collections::HashMap;

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
        .map(|(a, _)| *a)
        .sum()
}

fn exercise_2(input: &Input) -> u64 {
    input
        .par_iter()
        .filter(|(a, b)| is_ok2(**a, *b))
        .map(|(a, _)| *a)
        .sum()
}

fn is_ok(result: u64, line: &[u64]) -> bool {
    is_ok_mul2(result, line, is_ok) || is_ok_add2(result, line, is_ok)
}
fn is_ok2(result: u64, line: &[u64]) -> bool {
    is_ok_mul2(result, line, is_ok2)
        || is_ok_add2(result, line, is_ok2)
        || is_ok_concat2(result, line, is_ok2)
}

fn is_ok_mul2(result: u64, line: &[u64], eval_further: fn(u64, &[u64]) -> bool) -> bool {
    if let Some(&last) = line.last() {
        if result % last != 0 {
            return false;
        }

        (eval_further)(result / last, &line[..line.len() - 1])
    } else {
        result == 0
    }
}

fn is_ok_add2(result: u64, line: &[u64], eval_further: fn(u64, &[u64]) -> bool) -> bool {
    if let Some(&last) = line.last() {
        if last > result {
            return false;
        }

        (eval_further)(result - last, &line[..line.len() - 1])
    } else {
        result == 0
    }
}

fn is_ok_concat2(result: u64, line: &[u64], eval_further: fn(u64, &[u64]) -> bool) -> bool {
    if let Some(&last) = line.last() {
        let count_tenths = last.ilog10() + 1;
        let pow = 10u64.pow(count_tenths);

        if (result - last) % pow != 0 {
            return false;
        }

        (eval_further)(result / pow, &line[..line.len() - 1])
    } else {
        result == 0
    }
}
