use std::collections::HashMap;

use num_bigint::BigUint;

type Input = (Vec<char>, HashMap<(char, char), char>);

pub fn main() {
    let input = parse_input(include_str!("../input/test.txt"));
    println!("Ex1: {}", exercise_2(&input, 10));

    
    let start = std::time::SystemTime::now();
    // println!("Ex2: {}", exercise_2(&input, 1_000_000));
    
    let end = std::time::SystemTime::now();

    println!("D: {:?}", end.duration_since(start));
}

fn parse_input(input: &str) -> Input {
    let mut lines = input.lines();

    let polymer = lines.next().unwrap().chars().collect();
    lines.next().unwrap();
    let map = lines
        .map(|line| {
            let mut split = line.split(" -> ");
            let left = split.next().unwrap().to_string();
            let right = split.next().unwrap().to_string();

            let mut left = left.chars();
            let mut right = right.chars();

            (
                (left.next().unwrap(), left.next().unwrap()),
                right.next().unwrap(),
            )
        })
        .collect();

    (polymer, map)
}

fn exercise_2((polymer, map): &Input, steps: usize) -> BigUint {
    let last_char = *polymer.last().unwrap();

    let mut pairs = polymer.windows(2).fold(HashMap::new(), |mut acc, x| {
        *acc.entry((x[0], x[1])).or_insert(BigUint::from(0u32)) += BigUint::from(1u32);
        acc
    });

    for _ in 0..steps {
        let mut new_pairs = HashMap::new();
        for (a, b) in map {
            if let Some(p) = pairs.get(a) {
                *new_pairs.entry((a.0, *b)).or_insert(BigUint::from(0u32)) += p;
                *new_pairs.entry((*b, a.1)).or_insert(BigUint::from(0u32)) += p;
            }
        }
        pairs = new_pairs;
    }

    let mut count = pairs.iter().fold(HashMap::new(), |mut acc, ((x, _), p)| {
        *acc.entry(*x).or_insert(BigUint::from(0u32)) += p;
        acc
    });

    *count.entry(last_char).or_insert(BigUint::from(0u32)) += BigUint::from(1u32);

    let max = count.values().max().unwrap();
    let min = count.values().min().unwrap();

    let mut pairs = pairs.into_iter().collect::<Vec<_>>();
    pairs.sort_by_key(|x| x.0.clone());
    for pair in pairs {
        println!("{:?}", pair);
    }
    max - min
}
