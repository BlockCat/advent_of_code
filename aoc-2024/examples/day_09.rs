use std::collections::HashSet;

use aoc_2024::stopwatch;

type Input = Vec<u8>;

pub fn main() {
    let input = include_str!("../input/day_09.txt");
    // let input = include_str!("../input/test.txt");
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
    input.chars().map(|c| c as u8 - b'0').collect()
}

fn exercise_1(input: &Input) -> usize {
    // let free_space: usize = input.iter().map(|x| *x as usize).step_by(2).sum();

    // println!("free_space_total: {}", free_space);

    checksum(0, input)
}

fn checksum(mut pos: usize, input: &[u8]) -> usize {
    let space_used: usize = input.iter().map(|x| *x as usize).step_by(2).sum();

    let mut id = 0;

    let mut sum = 0;
    let mut iter = input.iter();

    let mut last_index = input.len() - 1;
    let mut last_id = input.len() / 2;
    let mut last_len = input[last_index] as usize;
    let mut last_remaining = last_len;

    while let Some(first) = iter.next() {
        let first = *first as usize;
        let second = *iter.next().unwrap_or(&0) as usize;

        for p in pos..pos + first {
            if p >= space_used {
                unimplemented!();
                return sum;
            }
            sum += p * id;
        }

        pos += first;
        id += 1;

        for p in pos..pos + second {
            if last_remaining == 0 {
                last_index -= 2;
                last_id -= 1;
                last_len = input[last_index] as usize;
                last_remaining = last_len;
            }
            if p >= space_used {
                return sum;
            }
            sum += p * last_id;
            last_remaining -= 1;
        }

        pos += second;
    }
    sum
}

fn exercise_2(input: &Input) -> usize {
    let mut pos = 0;

    let mut sum = 0;
    let mut id = 0;

    let mut iter = input.iter();

    let mut ids = input
        .iter()
        .enumerate()
        .rev()
        .step_by(2)
        .map(|x| (x.0 / 2, *x.1))
        .collect::<Vec<_>>();

    let mut handled = HashSet::<usize>::new();

    while let Some(first) = iter.next() {
        let first = *first as usize;
        let empty_block = *iter.next().unwrap_or(&0u8);

        if handled.insert(id) {
            sum += (2 * pos + first - 1) * (first) / 2 * id;
        }
        pos += first;
        id += 1;

        let mut space_left = empty_block;

        while space_left > 0 {
            if let Some(i) = ids
                .iter()
                .filter(|x| x.0 >= id)
                .position(|(_, size)| *size <= space_left)
            {
                let (last_id, size) = ids.remove(i);
                let size = size as usize;
                handled.insert(last_id);

                sum += (2 * pos + size - 1) * (size) / 2 * last_id;

                pos += size as usize;
                space_left -= size as u8;
            } else {
                pos += space_left as usize;
                break;
            }
        }
    }

    sum
}
