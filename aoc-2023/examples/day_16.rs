use std::collections::{HashSet, VecDeque};

use aoc_2023::{
    direction::Direction,
    grid::{Grid, StaticGrid},
    vector::Vector2,
};
use rayon::iter::{ParallelBridge, ParallelIterator};

type InputType = StaticGrid<char>;

pub fn main() {
    let input = parse(include_str!("../input/day_16.txt"));

    println!("Exercise 1: {}", exercise_1(input.clone()));
    println!("Exercise 2: {}", exercise_2(input));
}

#[test]
fn test() {
    let input = parse(include_str!("../input/test.txt"));

    println!("Exercise 1: {}", exercise_1(input.clone()));
    println!("Exercise 2: {}", exercise_2(input));
}

fn parse<'a>(input: &'a str) -> InputType {
    StaticGrid::from_vec(
        input
            .lines()
            .map(|s| s.chars().map(|s| s).collect())
            .collect(),
    )
}

fn exercise_1(input: InputType) -> usize {
    start_light(&input, Vector2::new([0, 0]), Direction::East)
}

fn start_light(input: &InputType, pos: Vector2, dir: Direction) -> usize {
    let mut queue: VecDeque<(Vector2, Direction)> = VecDeque::new();
    let mut visited: HashSet<(Vector2, Direction)> = HashSet::new();
    let mut energized = HashSet::new();

    queue.push_back((pos, dir));

    while let Some((pos, dir)) = queue.pop_back() {
        if !visited.insert((pos, dir)) {
            continue;
        }
        let tile = input.get_vec(&pos);
        if tile.is_some() {
            energized.insert(pos);
        }

        match (tile, dir) {
            (Some('.'), _) => queue.push_back((pos + dir, dir)),
            (Some('-'), Direction::East) | (Some('-'), Direction::West) => {
                queue.push_back((pos + dir, dir))
            }
            (Some('|'), Direction::North) | (Some('|'), Direction::South) => {
                queue.push_back((pos + dir, dir))
            }
            (Some('-'), Direction::North) | (Some('-'), Direction::South) => {
                queue.push_back((pos + Direction::East, Direction::East));
                queue.push_back((pos + Direction::West, Direction::West));
            }
            (Some('|'), Direction::West) | (Some('|'), Direction::East) => {
                queue.push_back((pos + Direction::North, Direction::North));
                queue.push_back((pos + Direction::South, Direction::South));
            }
            (Some('/'), Direction::East) | (Some('\\'), Direction::West) => {
                queue.push_back((pos + Direction::North, Direction::North))
            }
            (Some('/'), Direction::West) | (Some('\\'), Direction::East) => {
                queue.push_back((pos + Direction::South, Direction::South))
            }
            (Some('/'), Direction::North) | (Some('\\'), Direction::South) => {
                queue.push_back((pos + Direction::East, Direction::East))
            }
            (Some('/'), Direction::South) | (Some('\\'), Direction::North) => {
                queue.push_back((pos + Direction::West, Direction::West))
            }
            (None, _) => {}

            _ => unreachable!(),
        }
    }

    // input.pretty_print(|s, c| if energized.contains(&c) { 'X' } else { *s });

    energized.len()
}

fn exercise_2(input: InputType) -> usize {
    let start_positions = (0..input.height as isize)
        .map(|s| (Vector2::new([0, s]), Direction::East))
        .chain(
            (0..input.height as isize)
                .map(|s| (Vector2::new([input.width as isize - 1, s]), Direction::West)),
        )
        .chain((0..input.width as isize).map(|s| (Vector2::new([s, 0]), Direction::South)))
        .chain((0..input.width as isize).map(|s| {
            (
                Vector2::new([s, input.height as isize - 1]),
                Direction::North,
            )
        }))
        .par_bridge()
        .map(|(start_pos, start_dir)| start_light(&input, start_pos, start_dir))
        .max();

    start_positions.unwrap()
}
