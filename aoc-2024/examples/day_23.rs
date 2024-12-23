use std::collections::{HashMap, HashSet};

use aoc_2024::stopwatch;
use rayon::prelude::*;

type Input = HashMap<String, HashSet<String>>;

pub fn main() {
    let numbers = input(include_str!("../input/day_23.txt"));
    // let numbers = input(include_str!("../input/test.txt"));

    let time = stopwatch(|| {
        println!("Exercise 1: {}", exercise_1(&numbers));
        println!("Exercise 2: {}", exercise_2(&numbers));
    });

    println!("time: {:?}", time);
}

fn input(input: &str) -> Input {
    let v: Vec<_> = input.lines().map(parse_line).collect();
    let mut map: HashMap<String, HashSet<String>> = HashMap::new();

    for (a, b) in v {
        map.entry(a.to_string()).or_default().insert(b.to_string());
        map.entry(b.to_string()).or_default().insert(a.to_string());
    }

    map
}

fn parse_line(line: &str) -> (&str, &str) {
    let (a, b) = line.split_once("-").unwrap();

    (a, b)
}

fn exercise_1(input: &Input) -> usize {
    let mut counter = 0;
    let mut set = HashSet::new();
    for (k, v) in input {
        for (k2) in v {
            let v2 = input.get(k2).unwrap().clone();
            for combi in v2.intersection(v) {
                if k.starts_with('t') || k2.starts_with('t') || combi.starts_with('t') {
                    counter += 1;
                    let mut x = [k.to_string(), k2.to_string(), combi.to_string()];
                    x.sort();
                    set.insert(x);
                }
            }
        }
    }

    set.len()
}

fn exercise_2(input: &Input) -> String {
    let mut input = input.clone();
    let mut pcs = input.keys().cloned().collect::<Vec<_>>();
    pcs.sort();

    let mut found_largest = Vec::new();
    let mut cache = HashSet::new();

    while let Some(x) = pcs.pop() {
        if let Some(mut grr) = get_largest_network(&[x.clone()], &input, &mut cache) {
            if grr.len() > found_largest.len() {
                grr.sort();
                found_largest = grr;
                println!("{:?}", found_largest);
            }
            remove_from_network(&x, &mut input);
        }
    }
    found_largest.join(",")
}

fn remove_from_network(pc: &str, network: &mut Input) {
    network.remove(pc);
    network.values_mut().for_each(|x| {
        x.remove(pc);
    });
}

fn combined_set(pcs: &[String], input: &Input) -> HashSet<String> {
    pcs.iter()
        .map(|x| input.get(x).unwrap())
        .cloned()
        .reduce(|acc, x| acc.intersection(&x).cloned().collect::<HashSet<String>>())
        .unwrap()
}

fn get_largest_network(
    pcs: &[String],
    input: &Input,
    cache: &mut HashSet<Vec<String>>,
) -> Option<Vec<String>> {
    // The combination
    let combined = combined_set(pcs, input).into_iter().collect::<Vec<_>>();

    if combined.is_empty() {
        return Some(pcs.iter().map(|x| x.to_string()).collect());
    }

    combined
        .iter()
        .filter_map(|x| {
            let mut pcs = pcs.iter().map(|x| x.to_string()).collect::<Vec<_>>();
            pcs.push(x.clone());
            pcs.sort();
            if cache.contains(&pcs) {
                None
            } else {
                cache.insert(pcs.clone());
                get_largest_network(&pcs, input, cache)
            }
        })
        .max_by_key(|x| x.len())
}
