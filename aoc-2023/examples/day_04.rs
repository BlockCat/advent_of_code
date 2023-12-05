use rayon::{prelude::ParallelIterator, str::ParallelString};
use std::collections::HashSet;

type InputType = usize;

pub fn main() {
    let input = parse(include_str!("../input/day_04_big.txt"));
    println!("Exercise 1: {}", exercise_1(&input));
    println!("Exercise 2: {}", exercise_2(&input));
}

fn parse<'a>(input: &'a str) -> Vec<InputType> {
    input.par_lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> InputType {
    let (winning, mine) = line.split_once(":").unwrap().1.split_once(" | ").unwrap();

    let winning = winning
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<HashSet<usize>>();
    mine.split_whitespace()
        .filter(|s| winning.contains(&s.parse().unwrap()))
        .count()
}

fn exercise_1(input: &Vec<InputType>) -> usize {
    input
        .into_iter()
        .filter(|c| **c > 0)
        .map(|c| 2usize.pow(*c as u32 - 1))
        .sum()
}

fn exercise_2(input: &Vec<InputType>) -> usize {
    let mut amounts = vec![1usize; input.len()];

    input.into_iter().enumerate().for_each(|(i, &winnings)| {
        let add = amounts[i];
        for j in (i + 1)..(i + winnings + 1).min(amounts.len()) {
            amounts[j] += add;
        }
    });

    amounts.into_iter().sum()
}
