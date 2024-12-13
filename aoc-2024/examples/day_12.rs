use std::collections::{HashMap, HashSet, VecDeque};

use aoc_2024::{
    direction::Direction,
    grid::{Grid, StaticGrid},
    stopwatch,
    vector::Vector2,
};

type Input = StaticGrid<char>;

// ex2: wrong: 880180,
pub fn main() {
    let numbers = input(include_str!("../input/day_12.txt"));
    // let numbers = input(include_str!("../input/test.txt"));

    let time = stopwatch(|| {
        println!("Exercise 1: {}", exercise_1(&numbers));
        println!("Exercise 2: {}", exercise_2(&numbers));
    });

    println!("time: {:?}", time);
}

fn input(input: &str) -> Input {
    StaticGrid::from_vec(input.lines().map(parse_line).collect())
}

fn parse_line(line: &str) -> Vec<char> {
    line.chars().collect()
}

fn exercise_1(input: &Input) -> usize {
    let mut visited = HashSet::new();

    let mut sum = 0;
    for (pos, _) in input.iter() {
        if !visited.contains(&pos) {
            let (fence, area) = explore(input, pos, &mut visited);
            sum += fence * area;
        }
    }

    sum
}
fn exercise_2(input: &Input) -> usize {
    let mut visited = HashSet::new();

    let mut sum = 0;
    for (pos, _) in input.iter() {
        if !visited.contains(&pos) {
            let (_, area, sides) = explore2(input, pos, &mut visited);
            sum += sides * area;
        }
    }

    sum
}

fn explore(input: &Input, pos: Vector2, visited: &mut HashSet<Vector2>) -> (usize, usize) {
    let mut area = 0;
    let mut perimeter = 0;

    let mut queue = VecDeque::new();
    queue.push_back(pos);

    let ch = input.get_vec(&pos).unwrap();

    while let Some(n) = queue.pop_back() {
        if !visited.insert(n) {
            continue;
        }

        area += 1;

        for neighbor in n.neighbours_4() {
            if let Some(nchar) = input.get_vec(&neighbor) {
                if nchar == ch {
                    queue.push_back(neighbor);
                } else {
                    perimeter += 1;
                }
            } else {
                perimeter += 1;
            }
        }
    }

    (perimeter, area)
}

fn explore2(
    input: &Input,
    start_pos: Vector2,
    visited: &mut HashSet<Vector2>,
) -> (usize, usize, usize) {
    let mut parts = HashSet::new();
    let mut perimeter = 0;

    let mut queue = VecDeque::new();
    queue.push_back(start_pos);

    let ch = input.get_vec(&start_pos).unwrap();

    while let Some(n) = queue.pop_back() {
        if !visited.insert(n) {
            continue;
        }
        let pos_char = input.get_vec(&n);

        assert_eq!(pos_char, Some(ch));

        parts.insert(n);

        for neighbor in n.neighbours_4() {
            if let Some(nchar) = input.get_vec(&neighbor) {
                if nchar == ch {
                    queue.push_back(neighbor);
                } else {
                    perimeter += 1;
                }
            } else {
                perimeter += 1;
            }
        }
    }

    let area = parts.len();
    let sides = calculate_sides(parts);
    // pos is a corner

    (perimeter, area, sides)
}

fn calculate_starting_positions(parts: &HashSet<Vector2>) -> Vec<Vector2> {
    let mut possible_hole_start_points = Vec::new();
    for pos in parts {
        if !parts.contains(&(*pos + Direction::North)) {
            possible_hole_start_points.push(*pos);
        }
    }

    possible_hole_start_points
}
fn calculate_sides(parts: HashSet<Vector2>) -> usize {
    let mut visited = HashSet::new();
    calculate_starting_positions(&parts)
        .into_iter()
        .map(|start_pos| {
            let mut sides = 0;
            if visited.insert((start_pos, Direction::East)) {
                let mut pos = start_pos;

                let mut dir = Direction::East;
                loop {
                    let next_pos = pos + dir;
                    let next_pos_left = next_pos + dir.left();
                    let pnp = parts.contains(&next_pos);
                    let pnpl = parts.contains(&next_pos_left);

                    // turn left
                    if pnp && pnpl {
                        sides += 1;
                        dir = dir.left();
                        pos = next_pos_left;
                    } else if pnp {
                        pos = next_pos;
                    } else if !pnp {
                        dir = dir.right();
                        sides += 1;
                    } else {
                        unreachable!();
                    }
                    visited.insert((pos, dir));

                    if pos == start_pos && dir == Direction::East {
                        break;
                    }
                }
            }
            sides
        })
        .sum()
}
