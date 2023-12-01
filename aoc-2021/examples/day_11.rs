use std::collections::{HashMap, HashSet, VecDeque};

use aoc_2021::{
    grid::{Grid, StaticGrid},
    vector::Vector2,
};

type Input = StaticGrid<usize>;

pub fn main() {
    let input = parse_input(include_str!("../input/day11.txt"));
    println!("Ex1: {}", exercise_1(&input, 100));
    println!("Ex2: {}", exercise_2(&input));    
}

fn parse_input(input: &str) -> Input {
    StaticGrid::from_vec(input.lines().map(parse_line).collect())
}

fn parse_line(line: &str) -> Vec<usize> {
    line.chars().map(|x| (x as usize - '0' as usize)).collect()
}

fn exercise_1(input: &Input, steps: usize) -> usize {
    let mut prev = input.clone();
    let mut flashes = 0;
    let mut map = HashSet::new();
    map.insert(prev.clone());

    for _ in 0..steps {
        let (new_flashes, cur) = iteration(prev);
        flashes += new_flashes;
        prev = cur;
    }

    flashes
}

fn exercise_2(input: &Input) -> usize {
    let mut prev = input.clone();
    for counter in 1.. {
        let (flashes, cur) = iteration(prev);
        if flashes == 100 {
            return counter;
        }
        prev = cur;
    }

    unreachable!()
}

fn iteration(prev: StaticGrid<usize>) -> (usize, StaticGrid<usize>) {
    let neighbours = [
        Vector2::new([-1, -1]),
        Vector2::new([-1, 0]),
        Vector2::new([-1, 1]),
        Vector2::new([0, -1]),
        Vector2::new([0, 1]),
        Vector2::new([1, -1]),
        Vector2::new([1, 0]),
        Vector2::new([1, 1]),
    ];

    let mut flashes = 0;
    let mut cur = prev.clone();
    let mut handle = VecDeque::new();
    let mut visited = HashSet::new();

    for y in 0..prev.height {
        for x in 0..prev.width {
            let val = cur.get_mut(x, y).unwrap();
            *val += 1;
            if *val > 9 {
                handle.push_front(Vector2::new([x as isize, y as isize]));
            }
        }
    }
    
    while let Some(pos) = handle.pop_front() {
        if !visited.insert(pos) {
            continue;
        }
        for neighbour in &neighbours {
            let neighbour = pos + *neighbour;
            if let Some(x) = cur.get_mut_vec(&neighbour) {
                *x += 1;
                if *x > 9 && !visited.contains(&neighbour) {
                    handle.push_back(neighbour);
                }
            }
        }

    }
    for y in 0..prev.height {
        for x in 0..prev.width {
            let val = cur.get_mut(x, y).unwrap();

            if *val > 9 {
                *val = 0;
                flashes += 1;
            }
        }
    }
    (flashes, cur)
}
