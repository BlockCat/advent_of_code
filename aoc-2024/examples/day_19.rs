use std::collections::HashMap;

use aoc_2024::stopwatch;

type Input = (Vec<String>, Vec<String>);

pub fn main() {
    let numbers = input(include_str!("../input/day_19.txt"));
    // let numbers = input(include_str!("../input/test.txt"));

    let time = stopwatch(|| {
        println!("Exercise 1: {}", exercise_1(&numbers));
        println!("Exercise 2: {}", exercise_2(&numbers));
    });

    println!("time: {:?}", time);
}

fn input(input: &str) -> Input {
    let (patterns, designs) = input.split_once("\n\n").unwrap();
    let patterns = patterns
        .split(", ")
        .map(|x| x.to_string())
        .collect::<Vec<_>>();
    let designs = designs.lines().map(|x| x.to_string()).collect::<Vec<_>>();

    (patterns, designs)
}

fn exercise_1((patterns, designs): &Input) -> usize {
    let mut cache = HashMap::new();
    designs
        .iter()
        .filter(|d| is_design_possible(patterns, *d, &mut cache))
        .count()
}

fn exercise_2((patterns, designs): &Input) -> usize {
    let mut cache = HashMap::new();
    designs
        .iter()
        .map(|d| is_design_possible2(patterns, d, &mut cache))
        .sum()

}

fn is_design_possible(
    patterns: &[String],
    design: &str,
    cache: &mut HashMap<String, bool>,
) -> bool {
    if design.is_empty() {
        return true;
    }

    if let Some(result) = cache.get(design) {
        return *result;
    }

    for i in patterns {
        if design.starts_with(i) {
            let then = is_design_possible(patterns, &design[i.len()..], cache);
            if then {
                cache.insert(design.to_string(), then);
                return true;
            }
        }
    }

    cache.insert(design.to_string(), false);
    return false;
}

fn is_design_possible2(
    patterns: &[String],
    design: &str,
    cache: &mut HashMap<String, usize>,
) -> usize {
    if design.is_empty() {
        return 1;
    }

    if let Some(result) = cache.get(design) {
        return *result;
    }

    let mut sum = 0;
    for i in patterns {
        if design.starts_with(i) {
            let then = is_design_possible2(patterns, &design[i.len()..], cache);

            cache.insert(design.to_string(), then);
            sum += then;
        }
    }

    cache.insert(design.to_string(), sum);
    return sum;
}
