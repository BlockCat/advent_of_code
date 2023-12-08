use rayon::prelude::{ParallelBridge, ParallelIterator};
use std::collections::HashMap;

type InputType = (Vec<bool>, Vec<(String, String, String)>);

pub fn main() {
    let input = parse(include_str!("../input/day_08.txt"));

    println!("Exercise 1: {}", exercise_1(input.clone()));
    println!("Exercise 2: {}", exercise_2(input));
}

fn parse<'a>(input: &'a str) -> InputType {
    let mut lines = input.lines();

    // L = true, R = false
    let instructions = lines
        .next()
        .unwrap()
        .chars()
        .map(|s| s == 'L')
        .collect::<Vec<_>>();

    lines.next().unwrap();

    let network = lines.map(parse_line).collect::<Vec<_>>();

    (instructions, network)
}

fn parse_line(line: &str) -> (String, String, String) {
    let loc = &line[0..3];
    let left = &line[7..10];
    let right = &line[12..15];

    (loc.to_string(), left.to_string(), right.to_string())
}

fn exercise_1(input: InputType) -> usize {
    let left_map = input
        .1
        .iter()
        .map(|(loc, left, _)| (loc.clone(), left.clone()))
        .collect::<HashMap<_, _>>();

    let right_map = input
        .1
        .iter()
        .map(|(loc, _, right)| (loc.clone(), right.clone()))
        .collect::<HashMap<_, _>>();

    let mut loc = "AAA";

    for (i, left) in input.0.iter().cycle().enumerate() {
        if *left {
            loc = &left_map[loc];
        } else {
            loc = &right_map[loc];
        }

        if loc == "ZZZ" {
            return i + 1;
        }
    }

    unreachable!()
}

fn exercise_2((lr, input): InputType) -> usize {
    let left_map = input
        .iter()
        .map(|(loc, left, _)| (loc.clone(), left.clone()))
        .collect::<HashMap<_, _>>();

    let right_map = input
        .iter()
        .map(|(loc, _, right)| (loc.clone(), right.clone()))
        .collect::<HashMap<_, _>>();

    input
        .iter()
        .filter(|s| s.0.ends_with("A"))
        .map(|s| s.0.clone())
        .par_bridge()
        .map(|loc| {
            let mut visited = HashMap::new();

            lr.iter()
                .enumerate()
                .cycle()
                .scan(loc, |acc, (i, left)| {
                    *acc = if *left {
                        left_map[acc].clone()
                    } else {
                        right_map[acc].clone()
                    };

                    Some((i, acc.clone()))
                })
                .enumerate()
                .find_map(|(counter, (i, loc))| {
                    if loc.ends_with("Z") {
                        if let Some(old_cycle) = visited.insert((loc.clone(), i), counter + 1) {
                            Some(counter + 1 - old_cycle)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .unwrap()
        })
        .reduce(|| 1, |acc, x| lcm(acc, x))
}

fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }

    gcd(b, a % b)
}
