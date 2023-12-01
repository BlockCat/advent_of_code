use aoc_2021::vector::Vector2;
use std::collections::HashSet;

type Input = (Vec<Vector2>, Vec<Direction>);
pub fn main() {
    let input = parse_input(include_str!("../input/test.txt"));
    println!("Ex1: {}", exercise_1(&input, 1, false));
    println!("Ex2: {}", exercise_1(&input, input.1.len(), true));
}

fn parse_input(input: &str) -> Input {
    let mut lines = input.lines();
    let dots = lines
        .by_ref()
        .take_while(|x| !x.is_empty())
        .map(parse_dot)
        .collect();
    let folds = lines.map(parse_folds).collect();

    (dots, folds)
}

fn parse_dot(line: &str) -> Vector2 {
    let mut split = line.split(',');
    let x = split.next().unwrap().parse().unwrap();
    let y = split.next().unwrap().parse().unwrap();

    Vector2::new([x, y])
}

fn parse_folds(line: &str) -> Direction {
    let shift = line[13..].parse().unwrap();
    match &line[11..12] {
        "x" => Direction::X(shift),
        "y" => Direction::Y(shift),
        c => unreachable!("Not reachable: {}", c),
    }
}

fn exercise_1((dots, folds): &Input, amount: usize, print: bool) -> usize {
    let mut set = dots.iter().cloned().collect::<HashSet<_>>();

    for index in 0..amount {
        let instruction = &folds[index];

        match instruction {
            Direction::Y(shift) => {
                set = set
                    .into_iter()
                    .map(|vec| {
                        if vec[1] >= *shift {
                            translate_y(vec, *shift)
                        } else {
                            vec
                        }
                    })
                    .collect();
            }
            Direction::X(shift) => {
                set = set
                    .into_iter()
                    .map(|vec| {
                        if vec[0] >= *shift {
                            translate_x(vec, *shift)
                        } else {
                            vec
                        }
                    })
                    .collect();
            }
        }
    }

    if print {
        draw_set(&set);
    }
    set.len()
}

fn draw_set(set: &HashSet<Vector2>) {
    let max_x = set.iter().map(|x| x[0]).max().unwrap();
    let max_y = set.iter().map(|x| x[1]).max().unwrap();

    for y in 0..=max_y {
        for x in 0..=max_x {
            if !set.contains(&Vector2::new([x, y])) {
                print!(" ");
            } else {
                print!("#");
            }
        }
        println!();
    }
}

fn translate_y(dot: Vector2, shift: isize) -> Vector2 {
    Vector2::new([dot[0], 2 * shift - dot[1]])
}

fn translate_x(dot: Vector2, shift: isize) -> Vector2 {
    Vector2::new([2 * shift - dot[0], dot[1]])
}

enum Direction {
    Y(isize),
    X(isize),
}
