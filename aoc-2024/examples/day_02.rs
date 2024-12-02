use std::collections::HashSet;

use aoc_2024::stopwatch;

type Input = Vec<Vec<u32>>;

// 662
// 708
pub fn main() {
    let input = include_str!("../input/day_02.txt");

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

fn parse_line(input: &str) -> Vec<u32> {
    input
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect()
}

fn exercise_1(input: &Input) -> usize {
    input.iter().filter(|x| is_correct(*x)).count()
}

fn exercise_2(input: &Input) -> usize {
    input
        .iter()
        .filter(|x| is_correct_with_dampening(*x))
        .count()
}

fn is_increasing(x: &[u32]) -> bool {
    x.is_sorted_by(|a, b| b > a)
}
fn is_decreasing(x: &[u32]) -> bool {
    x.is_sorted_by(|a, b| a > b)
}

fn diff_enough(x: &[u32]) -> bool {
    x.windows(2).all(|a| a[0].abs_diff(a[1]) <= 3)
}

fn is_correct(x: &[u32]) -> bool {
    (is_increasing(x) || is_decreasing(x)) && diff_enough(x)
}
fn is_correct_with_dampening(x: &Vec<u32>) -> bool {
    if is_correct(x) {
        return true;
    }
    for i in 0..x.len() {
        let mut x = x.clone();
        x.remove(i);
        if (is_increasing(&x) || is_decreasing(&x)) && diff_enough(&x) {
            return true;
        }
    }
    false
}
