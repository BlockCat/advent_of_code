use std::collections::{HashMap, HashSet};

type InputType = Vec<String>;

pub fn main() {
    let input = parse(include_str!("../input/day_15.txt"));

    println!("Exercise 1: {}", exercise_1(input.clone()));
    println!("Exercise 2: {}", exercise_2(input));
}

#[test]
fn test() {
    let input = parse(include_str!("../input/test.txt"));

    println!("Exercise 1: {}", exercise_1(input.clone()));
    println!("Exercise 2: {}", exercise_2(input));
}

fn parse<'a>(input: &'a str) -> InputType {
    input.split(",").map(|s| s.to_string()).collect()
}

fn exercise_1(input: InputType) -> usize {
    input.iter().map(|s| hash_string(s)).sum::<usize>()
}

fn exercise_2(input: InputType) -> usize {
    let mut boxes: Vec<Vec<(String, usize)>> = vec![Vec::new(); 256];

    for instruction in input {
        if instruction.ends_with('-') {
            let instruction = instruction.trim_end_matches('-');
            let hash = hash_string(instruction);

            if let Some(pos) = boxes[hash].iter().position(|s| s.0 == instruction) {
                boxes[hash].remove(pos);
            }
        } else if instruction.contains("=") {
            let (label, fl) = instruction.split_once("=").unwrap();
            let hash = hash_string(label);

            if let Some(pos) = boxes[hash].iter().position(|s| s.0 == label) {
                boxes[hash][pos].1 = fl.parse::<usize>().unwrap();
            } else {
                boxes[hash].push((label.to_string(), fl.parse::<usize>().unwrap()));
            }
        } else {
            unreachable!();
        }
    }

    boxes
        .iter()
        .enumerate()
        .flat_map(|(abi, abox)| {
            abox.iter()
                .enumerate()
                .map(move |(i, (_, fl))| (abi + 1) * (i + 1) * *fl)
        })
        .sum::<usize>()
}

fn hash_string(input: &str) -> usize {
    input
        .chars()
        .fold(0, |acc, x| ((acc + (x as u8) as usize) * 17) % 256)
}

#[test]
fn hash_est() {
    assert_eq!(hash_string("HASH"), 52);
}
