use std::collections::BinaryHeap;

pub fn main() {
    let a1 = exercise_1(include_str!("../input/day_01.txt"));
    let a2 = exercise_2(include_str!("../input/day_01.txt"));
    println!("Ex1: {}", a1);
    println!("Ex2: {}", a2);
}

fn exercise_1(input: &str) -> u32 {
    input
        .split_terminator("\n\n")
        .map(|set| set.lines().flat_map(|line| line.parse::<u32>().ok()).sum())
        .max()
        .unwrap()
}

fn exercise_2(input: &str) -> u32 {
    input
        .split_terminator("\n\n")
        .map(|set| set.lines().flat_map(|line| line.parse::<u32>().ok()).sum())
        .collect::<BinaryHeap<u32>>()
        .into_iter()
        .take(3)
        .sum()
}
