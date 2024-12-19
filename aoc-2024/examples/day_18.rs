use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet, VecDeque},
};

use aoc_2024::{
    grid::{Grid, StaticGrid},
    stopwatch,
    vector::Vector2,
};

type Input = (StaticGrid<bool>, Vec<Vector2>);

pub fn main() {
    let numbers = input(include_str!("../input/day_18.txt"), 71);
    // let numbers = input(include_str!("../input/test.txt"), 7);

    let time = stopwatch(|| {
        println!("Exercise 1: {}", exercise_1(&numbers, 1024).unwrap());
        println!("Exercise 2: {}", exercise_2(&numbers));
    });

    println!("time: {:?}", time);
}

fn input(input: &str, size: usize) -> Input {
    let grid = StaticGrid::new(size, size);
    (grid, input.lines().map(parse_line).collect())
}

fn parse_line(line: &str) -> Vector2 {
    let (a, b) = line.split_once(",").unwrap();

    Vector2::new([a.parse().unwrap(), b.parse().unwrap()])
}

fn exercise_1((grid, bytes): &Input, len: usize) -> Option<usize> {
    let mut grid = grid.clone();
    for pos in bytes.iter().take(len) {
        grid.set_vec(pos, true);
    }

    let start = Vector2::zero();
    let end = Vector2::new([grid.width as isize - 1, grid.height as isize - 1]);

    let mut queue = VecDeque::new();
    queue.push_back((0, start));

    let mut visited = HashSet::new();

    while let Some((score, pos)) = queue.pop_front() {
        if pos == end {
            return Some(score);
        }
        if !visited.insert(pos) {
            continue;
        }
        for (n, corrupted) in grid.get_neighbours_4(&pos) {
            if !corrupted {
                queue.push_back((score + 1, n));
            }
        }
    }
    None
}
fn exercise_2(input: &Input) -> String {
    for i in 1024.. {
        let result = exercise_1(input, i);

        if result.is_none() {
            let coord = input.1[i - 1];

            return format!("{},{}", coord[0], coord[1]);
        }
    }
    unimplemented!()
}
