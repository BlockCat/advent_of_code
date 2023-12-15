use std::{
    collections::{HashMap, VecDeque},
    str::FromStr,
};

use aoc_2023::{
    direction::Direction,
    grid::{Grid, StaticGrid},
    vector::Vector2,
};

type InputType = StaticGrid<Tile>;

pub fn main() {
    let input: StaticGrid<Tile> = parse(include_str!("../input/day_14.txt"));

    println!("Exercise 1: {}", exercise_1(input.clone()));
    println!("Exercise 2: {}", exercise_2(input));
}

#[test]
fn test() {
    let input: StaticGrid<Tile> = parse(include_str!("../input/test.txt"));

    input.pretty_print(|s| match s {
        Tile::Cube => '#',
        Tile::Empty => '.',
        Tile::Round => 'O',
    });

    let input = tilt_east(input);
    println!();

    input.pretty_print(|s| match s {
        Tile::Cube => '#',
        Tile::Empty => '.',
        Tile::Round => 'O',
    });
}

fn parse<'a>(input: &'a str) -> InputType {
    let grid: Vec<Vec<Tile>> = input.lines().map(|line| parse_line(line)).collect();
    StaticGrid::from_vec(grid)
}

fn parse_line(line: &str) -> Vec<Tile> {
    line.chars().map(|c| Tile::from(c)).collect()
}

fn exercise_1(input: InputType) -> usize {
    calculate_load_north(&tilt_north(input))
}

fn exercise_2(input: InputType) -> usize {
    let mut input = input;
    let mut map = HashMap::new();
    let count = 1000000000;
    for i in 0..count {
        if let Some(old) = map.insert(input.clone(), i) {
            let diff = i - old;
            let remaining = count - i;
            let remaining = remaining % diff;
            for _ in 0..remaining {
                input = cycle(input);
            }
            break;
        } else {
            input = cycle(input);
        }
    }
    calculate_load_north(&input)
}

fn cycle(input: InputType) -> InputType {
    let input = tilt_north(input);
    let input = tilt_west(input);
    let input = tilt_south(input);
    let input = tilt_east(input);
    input
}

fn tilt_north(mut input: InputType) -> InputType {
    let y = 0isize;
    for x in 0..input.width as isize {
        tilt(Vector2::new([x, y]), Direction::North, &mut input);
    }
    input
}

fn tilt_south(mut input: InputType) -> InputType {
    let y = input.height as isize - 1;
    for x in 0..input.width as isize {
        tilt(Vector2::new([x, y]), Direction::South, &mut input);
    }
    input
}

fn tilt_west(mut input: InputType) -> InputType {
    let x = 0isize;
    for y in 0..input.height as isize {
        tilt(Vector2::new([x, y]), Direction::West, &mut input);
    }
    input
}

fn tilt_east(mut input: InputType) -> InputType {
    let x = input.width as isize - 1;
    for y in 0..input.height as isize {
        tilt(Vector2::new([x, y]), Direction::East, &mut input);
    }
    input
}

fn tilt(start: Vector2, dir: Direction, input: &mut InputType) {
    let mut last_ok = VecDeque::with_capacity(10);

    let rev_dir = dir.reverse();

    let mut current_pos = start;

    while let Some(tile) = input.get_vec(&(current_pos)) {
        match *tile {
            Tile::Cube => last_ok.clear(),
            Tile::Empty => last_ok.push_back(current_pos),
            Tile::Round => {
                // println!("moving {:?} to {:?}: {:?}", current_pos, tile, last_ok);
                if let Some(last) = last_ok.pop_front() {
                    assert_ne!(current_pos, last);
                    input.set_vec(&current_pos, Tile::Empty);
                    input.set_vec(&last, Tile::Round);
                    last_ok.push_back(current_pos);
                }
            }
        }

        current_pos = current_pos + rev_dir;
    }
}

fn calculate_load_north(input: &InputType) -> usize {
    input
        .iter()
        .filter(|s| s.1 == &Tile::Round)
        .map(|(pos, _)| input.height - pos[1] as usize)
        .sum()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
enum Tile {
    Round,
    Cube,
    #[default]
    Empty,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '#' => Tile::Cube,
            '.' => Tile::Empty,
            'O' => Tile::Round,
            _ => panic!("Invalid tile"),
        }
    }
}
