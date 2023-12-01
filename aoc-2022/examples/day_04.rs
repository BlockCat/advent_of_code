#![feature(iter_next_chunk)]

use rayon::prelude::*;

pub fn main() {
    let numbers = input();

    println!("Exercise 1: {}", exercise_1(&numbers));
    println!("Exercise 2: {}", exercise_2(&numbers));
}

fn input() -> Vec<[[i16; 2]; 2]> {
    let numbers = include_str!("../input/day_04.txt")
        .lines()
        .map(parse_line)
        .collect::<Vec<_>>();
    numbers
}
fn parse_line(line: &str) -> [[i16; 2]; 2] {
    line.split(',')
        .map(|x| {
            x.split('-')
                .map(|a| a.parse().unwrap())
                .next_chunk::<2>()
                .unwrap()
        })
        .next_chunk::<2>()
        .unwrap()
}
fn exercise_1(input: &Vec<[[i16; 2]; 2]>) -> usize {
    input
        .par_iter()
        .filter(|[a, b]| (a[0] - b[0]) * (a[1] - b[1]) <= 0) //(a[0] >= b[0] && a[1] <= b[1]) || (b[0] >= a[0] && b[1] <= a[1]))
        .count()
}
fn exercise_2(input: &Vec<[[i16; 2]; 2]>) -> usize {
    input
        .par_iter()
        .filter(|[a, b]| !(a[1] < b[0] || a[0] > b[1]))
        .count()
}
