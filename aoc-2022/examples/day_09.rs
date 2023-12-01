use aoc_2022::{direction::Direction, stopwatch, vector::Vector2};
use std::{collections::HashSet, thread};

type InputType = Vec<(Direction, usize)>;

pub fn main() {
    let x = stopwatch(move || {
        let numbers = input();
        thread::scope(|s| {
            s.spawn(|| {
                println!("Exercise 1: {}", exercise::<2>(&numbers));
            });
            s.spawn(|| {
                println!("Exercise 2: {}", exercise::<10>(&numbers));
            });
        });
    });

    println!("R: {:?}", x);
}

fn input() -> InputType {
    include_str!("../input/day_09.txt")
        .lines()
        .map(parse_line)
        .collect()
}

fn parse_line(line: &str) -> (Direction, usize) {
    let dir = match &line[0..1] {
        "D" => Direction::South,
        "L" => Direction::West,
        "U" => Direction::North,
        "R" => Direction::East,
        _ => unreachable!(),
    };

    (dir, line[2..].parse().unwrap())
}

fn exercise<const N: usize>(input: &InputType) -> usize {
    let mut visited = HashSet::new();

    let mut parts = [Vector2::new([0, 0]); N];

    visited.insert(parts[0].clone());

    for (dir, steps) in input.iter() {
        for _ in 0..*steps {
            parts[0] += *dir;
            for i in 1..N {
                let x = step(parts[i], parts[i - 1]);
                if x == parts[i] {
                    break;
                }
                parts[i] = x
            }
            visited.insert(parts[N - 1]);
        }
    }

    visited.len()
}

fn step(after_pos: Vector2, next_pos: Vector2) -> Vector2 {
    let dx = next_pos[0] - after_pos[0];
    let dy = next_pos[1] - after_pos[1];

    match (dx.abs(), dy.abs()) {
        (2, 2) => Vector2::new([after_pos[0] + dx.signum(), after_pos[1] + dy.signum()]),
        (2, _) => Vector2::new([after_pos[0] + dx.signum(), next_pos[1]]),
        (_, 2) => Vector2::new([next_pos[0], after_pos[1] + dy.signum()]),
        _ => after_pos,
    }
}
