use std::collections::{HashMap, HashSet, VecDeque};

use aoc_2021::{
    grid::{Grid, StaticGrid},
    vector::Vector2,
};

type Input = StaticGrid<u8>;

pub fn main() {
    let input = parse_input(include_str!("../input/day09.txt"));
    println!("Ex1: {}", exercise_1(&input));
    println!("Ex2: {}", exercise_2(&input));
}

fn parse_input(input: &str) -> Input {
    let lines = input.lines().map(parse_line).collect();

    StaticGrid::from_vec(lines)
}
fn parse_line(line: &str) -> Vec<u8> {
    line.chars().map(|x| (x as u8 - '0' as u8)).collect()
}

fn exercise_1(input: &Input) -> usize {
    let up = Vector2::new([0, 1]);
    let down = Vector2::new([0, -1]);
    let left = Vector2::new([-1, 0]);
    let right = Vector2::new([1, 0]);

    input
        .iter()
        .filter(|(pos, &mid)| -> bool {
            [*pos + up, *pos + down, *pos + left, *pos + right]
                .into_iter()
                .filter_map(|x| input.get_vec(&x))
                .all(|&point| point > mid)
        })
        .map(|x| (*x.1 + 1) as usize)
        .sum()
}

fn exercise_2(input: &Input) -> usize {
    let up = Vector2::new([0, 1]);
    let down = Vector2::new([0, -1]);
    let left = Vector2::new([-1, 0]);
    let right = Vector2::new([1, 0]);

    let lowest_points = lowest_points(input)
        .into_iter()
        .map(|x| x.0)
        .collect::<Vec<_>>();

    let mut basin_registry = lowest_points
        .iter()
        .map(|x| (*x, 1))
        .collect::<HashMap<_, _>>();
    let mut visited = lowest_points.iter().cloned().collect::<HashSet<_>>();
    let mut queue = lowest_points
        .iter()
        .map(|x| (*x, *x))
        .collect::<VecDeque<_>>();

    while let Some((pos, basin)) = queue.pop_front() {        
        for neighbour in [pos + up, pos + down, pos + left, pos + right] {
            let val = *input.get_vec(&neighbour).unwrap_or(&9);

            if val == 9 {
                // no need to eval
                continue;
            } else if !visited.contains(&neighbour) {
                queue.push_back((neighbour, basin));
                visited.insert(neighbour);

                *basin_registry.get_mut(&basin).unwrap() += 1;
            }
        }
    }

    let mut basins = basin_registry.values().collect::<Vec<_>>();
    basins.sort();
    basins.reverse();

    basins[0] * basins[1] * basins[2]
}

fn lowest_points(input: &Input) -> Vec<(Vector2, u8)> {
    let up = Vector2::new([0, 1]);
    let down = Vector2::new([0, -1]);
    let left = Vector2::new([-1, 0]);
    let right = Vector2::new([1, 0]);

    input
        .iter()
        .filter(|(pos, &mid)| -> bool {
            [*pos + up, *pos + down, *pos + left, *pos + right]
                .into_iter()
                .filter_map(|x| input.get_vec(&x))
                .all(|&point| point > mid)
        })
        .map(|x| (x.0, *x.1))
        .collect()
}
