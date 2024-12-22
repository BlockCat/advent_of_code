#![feature(iter_map_windows)]

use aoc_2024::stopwatch;
use rayon::prelude::*;
use std::{collections::HashMap, ops::BitXor};

type Input = Vec<usize>;

pub fn main() {
    let numbers = input(include_str!("../input/day_22.txt"));
    // let numbers = input(include_str!("../input/test.txt"));

    let time = stopwatch(|| {
        println!("Exercise 1: {}", exercise_1(&numbers));
        println!("Exercise 2: {}", exercise_2(&numbers));
    });

    let t = BananaGener { number: 123 }
        .take_while(|x| x != &123)
        .count();

    println!("c: {}", t);

    println!("time: {:?}", time);
}

fn input(input: &str) -> Input {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> usize {
    line.parse().unwrap()
}

fn exercise_1(input: &Input) -> usize {
    input
        .iter()
        .map(|x| BananaGener { number: *x })
        .map(|mut x| x.nth(1999).unwrap())
        .sum()
}

fn exercise_2(input: &Input) -> usize {
    let map = input.par_iter().map(|x| profit_finder(*x)).reduce(
        || HashMap::new(),
        |mut acc, x| {
            for (k, v) in x.into_iter() {
                *acc.entry(k).or_default() += v;
            }

            acc
        },
    );

    *map.values().max().unwrap()
}

fn evolve(mut secret: usize) -> usize {
    secret = prune(mix(secret, secret * 64));
    secret = prune(mix(secret, secret / 32));
    secret = prune(mix(secret, secret * 2048));
    secret
}

fn mix(a: usize, b: usize) -> usize {
    a.bitxor(b)
}

fn prune(a: usize) -> usize {
    a % 16777216
}

fn profit_finder(secret: usize) -> HashMap<[isize; 4], usize> {
    BananaGener { number: secret }
        .take(2000)
        .map(|x| x % 10)
        .map_windows(|[a, b]| (*b, *b as isize - *a as isize))
        .map_windows(|a: &[(usize, isize); 4]| {
            let x = [a[0].1, a[1].1, a[2].1, a[3].1];

            (x, a[3].0)
        })
        .fold(HashMap::with_capacity(2000 / 4), |mut acc, x| {
            if !acc.contains_key(&x.0) {
                acc.insert(x.0, x.1);
            }
            acc
        })
}

struct BananaGener {
    number: usize,
}

impl Iterator for BananaGener {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.number = evolve(self.number);

        Some(self.number)
    }
}
