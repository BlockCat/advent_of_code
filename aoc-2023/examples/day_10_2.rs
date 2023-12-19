use aoc_2023::{
    grid::{Grid, StaticGrid},
    vector::Vector2,
};
use std::collections::HashMap;

type InputType = StaticGrid<char>;

pub fn main() {
    let input = parse(include_str!("../input/day_10.txt"));

    println!("Exercise 1: {}", exercise_1(input.clone()).len() / 2);
    println!("Exercise 2: {}", exercise_2(input));
}

fn parse<'a>(input: &'a str) -> InputType {
    StaticGrid::from_vec(
        input
            .lines()
            .map(|line| parse_line(line))
            .collect::<Vec<_>>(),
    )
}

fn parse_line(line: &str) -> Vec<char> {
    line.chars().collect()
}

fn exercise_1(input: InputType) -> Vec<(Vector2, char)> {
    let (starting_point, start_char) = input.iter().find(|s| s.1 == &'S').unwrap();

    let mut path = vec![(starting_point.clone(), *start_char)];

    path.push({
        let n = translate_s(&input).1;

        (n, *input.get_vec(&n).unwrap())
    });

    while path.len() == 1 || path.last().unwrap().1 != 'S' {
        let dir = path[path.len() - 1].0 - path[path.len() - 2].0;
        let (current_pos, current_char) = path.last().unwrap();

        match current_char {
            '-' => {
                let next_vec = if dir == Vector2::new([1, 0]) {
                    current_pos.clone() + Vector2::new([1, 0])
                } else if dir == Vector2::new([-1, 0]) {
                    current_pos.clone() + Vector2::new([-1, 0])
                } else {
                    unreachable!()
                };
                path.push((next_vec, *input.get_vec(&next_vec).unwrap()));
            }
            '|' => {
                let next_vec = if dir == Vector2::new([0, 1]) {
                    current_pos.clone() + Vector2::new([0, 1])
                } else if dir == Vector2::new([0, -1]) {
                    current_pos.clone() + Vector2::new([0, -1])
                } else {
                    unreachable!()
                };
                path.push((next_vec, *input.get_vec(&next_vec).unwrap()));
            }
            'L' => {
                let next_vec = if dir == Vector2::new([0, 1]) {
                    current_pos.clone() + Vector2::new([1, 0])
                } else if dir == Vector2::new([-1, 0]) {
                    current_pos.clone() + Vector2::new([0, -1])
                } else {
                    unreachable!()
                };
                path.push((next_vec, *input.get_vec(&next_vec).unwrap()));
            }
            'J' => {
                let next_vec = if dir == Vector2::new([0, 1]) {
                    current_pos.clone() + Vector2::new([-1, 0])
                } else if dir == Vector2::new([1, 0]) {
                    current_pos.clone() + Vector2::new([0, -1])
                } else {
                    unreachable!()
                };
                path.push((next_vec, *input.get_vec(&next_vec).unwrap()));
            }
            '7' => {
                let next_vec = if dir == Vector2::new([1, 0]) {
                    current_pos.clone() + Vector2::new([0, 1])
                } else if dir == Vector2::new([0, -1]) {
                    current_pos.clone() + Vector2::new([-1, 0])
                } else {
                    unreachable!()
                };
                path.push((next_vec, *input.get_vec(&next_vec).unwrap()));
            }
            'F' => {
                let next_vec = if dir == Vector2::new([0, -1]) {
                    current_pos.clone() + Vector2::new([1, 0])
                } else if dir == Vector2::new([-1, 0]) {
                    current_pos.clone() + Vector2::new([0, 1])
                } else {
                    unreachable!()
                };
                path.push((next_vec, *input.get_vec(&next_vec).unwrap()));
            }
            _ => unreachable!(),
        }
    }
    path
}

fn exercise_2(input: InputType) -> usize {
    let mut walls = exercise_1(input.clone());
    walls.iter_mut().for_each(|a| {
        if a.1 == 'S' {
            a.1 = translate_s(&input).0;
        }
    });

    let wall_map = walls.iter().cloned().collect::<HashMap<_, _>>();

    let left = wall_map.iter().map(|(v, _)| v[0]).min().unwrap();
    let right = wall_map.iter().map(|(v, _)| v[0]).max().unwrap();
    let top = wall_map.iter().map(|(v, _)| v[1]).min().unwrap();
    let bottom = wall_map.iter().map(|(v, _)| v[1]).max().unwrap();

    let mut counter = 0;
    for y in top..bottom {
        let mut is_in = false;
        for x in left..right {
            let wall_char = wall_map.get(&Vector2::new([x, y]));
            match wall_char {
                Some('|') => {
                    is_in = !is_in;
                }
                Some('F') => {
                    is_in = !is_in;
                }
                Some('7') => {
                    is_in = !is_in;
                }
                None if is_in => {
                    counter += 1;
                }
                _ => {}
            }
        }
    }
    counter
}

fn translate_s(input: &InputType) -> (char, Vector2) {
    let (starting_point, _) = input.iter().find(|s| s.1 == &'S').unwrap();

    let left = input
        .get_vec(&(starting_point + Vector2::new([-1, 0])))
        .map(|&left| left == '-' || left == 'L' || left == 'F')
        .unwrap();

    let right = input
        .get_vec(&(starting_point + Vector2::new([1, 0])))
        .map(|&right| right == '-' || right == 'J' || right == '7')
        .unwrap_or(false);

    let up = input
        .get_vec(&(starting_point + Vector2::new([0, -1])))
        .map(|&up| up == '|' || up == '7' || up == 'F')
        .unwrap_or(false);

    let down = input
        .get_vec(&(starting_point + Vector2::new([0, 1])))
        .map(|&down| down == '|' || down == 'L' || down == 'J')
        .unwrap();

    match (left, right, up, down) {
        (true, true, _, _) => ('-', starting_point + Vector2::new([1, 0])),
        (_, _, true, true) => ('|', starting_point + Vector2::new([0, 1])),
        (true, _, true, _) => ('J', starting_point + Vector2::new([0, 1])),
        (_, true, true, _) => ('L', starting_point + Vector2::new([1, 0])),
        (true, _, _, true) => ('7', starting_point + Vector2::new([0, 1])),
        (_, true, _, true) => ('F', starting_point + Vector2::new([1, 0])),
        _ => unreachable!(),
    }
}
