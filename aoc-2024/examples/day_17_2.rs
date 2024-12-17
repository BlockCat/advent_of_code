use aoc_2024::stopwatch;
use std::{collections::HashMap, usize};

type Input = CPU;

#[derive(Debug, Clone)]
struct CPU {
    register: [usize; 3],
    program: Vec<usize>,
}

pub fn main() {
    let real_input = CPU {
        register: [59590048, 0, 0],
        program: vec![2, 4, 1, 5, 7, 5, 0, 3, 1, 6, 4, 3, 5, 5, 3, 0],
    };
    let numbers = real_input;

    let time = stopwatch(|| {
        println!("Exercise 1: {}", exercise_1(&numbers));
        println!("Exercise 2: {}", exercise_2(&numbers));
    });

    println!("time: {:?}", time);
}

fn exercise_1(input: &Input) -> String {
    let numbers = decompiled(input.register[0]);

    numbers
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn decompiled(a: usize) -> Vec<usize> {
    let mut numbers = Vec::new();
    let mut a = a;

    while a != 0 {
        numbers.push(decompiled2(a));
        a = a >> 3;
    }

    numbers
}

fn decompiled2(a: usize) -> usize {
    let b = (a ^ 0b101) & 0b111;
    let c = (a >> b) ^ 0b110;

    (b ^ c) & 0b111
}

fn exercise_2(input: &Input) -> usize {
    input
        .program
        .iter()
        .rev()
        .cloned()
        .fold(vec![0usize], |acc, x| {
            acc.into_iter()
                .flat_map(|acc| (0..8).map(move |i| (acc << 3) | i))
                .filter(|acc| decompiled2(*acc) == x)
                .collect()
        })
        .into_iter()
        .next()
        .unwrap()
}
