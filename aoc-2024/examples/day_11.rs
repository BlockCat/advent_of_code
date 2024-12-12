use aoc_2024::stopwatch;
use num_traits::{CheckedEuclid, Euclid};
use std::collections::HashMap;

type Input = Vec<usize>;

// wrong:

pub fn main() {
    let input = include_str!("../input/day_11.txt");
    // let input = include_str!("../input/test.txt");
    let l = stopwatch(|| {
        let input = parse(input);

        println!("Ex1: {}", exercise_2(&input, 25));

        let a2 = exercise_2(&input, 75);
        println!("Ex2: {}", a2);
    });

    println!("Time: {:?}", l);
}

fn parse(input: &str) -> Input {
    input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn exercise_2(input: &Input, blinks: usize) -> usize {
    let mut results = HashMap::<(usize, usize), usize>::new();

    input
        .iter()
        .map(|stone| count_stones_resulting_from(*stone, blinks, &mut results))
        .sum()
}

fn count_stones_resulting_from(
    stone: usize,
    blinks: usize,
    results: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if blinks == 0 {
        return 1;
    }

    if let Some(result) = results.get(&(stone, blinks)) {
        return *result;
    }

    let result = if stone == 0 {
        count_stones_resulting_from(1, blinks - 1, results)
    } else if let Some(c) = Some((stone.ilog10() + 1) as u32).filter(|x| x % 2 == 0) {
        let powed = 10usize.pow(c / 2);

        let sa = stone.div_rem_euclid(&powed);

        let result_a = count_stones_resulting_from(sa.0, blinks - 1, results);
        let result_b = count_stones_resulting_from(sa.1, blinks - 1, results);

        result_a + result_b
    } else {
        count_stones_resulting_from(stone * 2024, blinks - 1, results)
    };

    results.insert((stone, blinks), result);

    return result;
}
