use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

use aoc_2021::{
    grid::{Grid, StaticGrid},
    vector::Vector2,
};

type Input = StaticGrid<u8>;

pub fn main() {
    let input = parse_input(include_str!("../input/test.txt"));
    println!("Ex1: {}", exercise_1(&input));
    println!("Ex2: {}", exercise_2(&input));
}

fn parse_input(input: &str) -> Input {
    let l = input.lines().map(parse_line).collect();

    StaticGrid::from_vec(l)
}

fn parse_line(line: &str) -> Vec<u8> {
    line.chars().map(|x| x as u8 - '0' as u8).collect()
}

fn exercise_1(input: &StaticGrid<u8>) -> usize {
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), Vector2::new([0, 0])));

    let target = Vector2::new([input.width as isize - 1, input.height as isize - 1]);

    let neighbours = [
        Vector2::new([0, -1]),
        Vector2::new([0, 1]),
        Vector2::new([1, 0]),
        Vector2::new([-1, 0]),
    ];

    let mut visited = HashSet::new();

    while let Some((risk, last)) = heap.pop() {
        if !visited.insert(last) {
            continue;
        }

        if last == target {
            return risk.0;
        }

        for neighbour in &neighbours {
            let neighbour = *neighbour + last;
            if let Some(item) = input.get_vec(&neighbour) {
                let new_risk = risk.0 + *item as usize;
                heap.push((Reverse(new_risk), neighbour))
            }
        }
    }

    unreachable!()
}

fn exercise_2(input: &Input) -> usize {
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0usize), Vector2::new([0, 0])));

    let target = Vector2::new([5 * input.width as isize - 1, 5 * input.height as isize - 1]);

    let neighbours = [
        Vector2::new([0, -1]),
        Vector2::new([0, 1]),
        Vector2::new([1, 0]),
        Vector2::new([-1, 0]),
    ];

    let mut visited = HashSet::new();

    while let Some((current_risk, last)) = heap.pop() {
        if last == target {
            return current_risk.0;
        }

        if !visited.insert(last) {
            continue;
        }

        for neighbour in &neighbours {
            let neighbour = *neighbour + last;
            let dx = neighbour[0] / input.width as isize;
            let dy = neighbour[1] / input.height as isize;

            let rx = neighbour[0] % input.width as isize;
            let ry = neighbour[1] % input.height as isize;

            if neighbour[0] >= (input.width * 5) as isize
                || neighbour[1] >= (input.height * 5) as isize
                || neighbour[0] < 0
                || neighbour[1] < 0
            {
                continue;
            }

            if let Some(item_risk) = input.get_vec(&Vector2::new([rx, ry])) {
                let mut item_risk = *item_risk as usize + (dx + dy) as usize;

                if item_risk > 9 {
                    let d = item_risk / 9;
                    item_risk -= 9 * d;
                }

                let new_risk = current_risk.0 + item_risk;

                heap.push((Reverse(new_risk), neighbour))
            }
        }
    }

    unreachable!()
}
