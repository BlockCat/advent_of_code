use aoc_2023::{
    grid::{Grid, StaticGrid},
    vector::Vector2,
};
use std::collections::{HashMap, VecDeque};

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
    let mut state: Vec<MyRange> = Vec::new();
    let mut counter = 0;
    let mut queue = get_events(&input);

    let walls = exercise_1(input.clone())
        .into_iter()
        .collect::<HashMap<_, _>>();

    let mut prev_y = walls.iter().map(|s| s.0[0]).min().unwrap();

    let mut start = None;

    while let Some(corner) = queue.pop_front() {
        match corner {
            Corner::TopLeft(x, y) => {
                if y != prev_y {
                    for y in prev_y..y {
                        for r in state.iter() {
                            for x in r.0..r.1 {
                                if !walls.contains_key(&Vector2::new([x, y])) {
                                    counter += 1;
                                }
                            }
                        }
                    }
                }
                prev_y = y;
                start = Some((x, corner));
            }
            Corner::TopRight(x, _) => {
                let (start_x, _) = start.unwrap();
                start = None;
                let left_range = state.iter().position(|s| s.contains(start_x));
                let right_range = state.iter().position(|s| s.contains(x));

                match (left_range, right_range) {
                    (Some(left_range), Some(right_range)) if left_range == right_range => {
                        let old_range = state.remove(left_range);

                        if old_range.0 < start_x {
                            state.push(MyRange::new(old_range.0, start_x));
                        }

                        state.push(MyRange::new(x, old_range.1));
                        state.sort();
                    }
                    (Some(left_range), Some(right_range)) => {
                        let right_range = state.remove(right_range);
                        let left_range = state.remove(left_range);

                        state.push(left_range.combine(right_range));
                        state.sort();
                    }
                    (None, None) => {
                        state.push(MyRange::new(start_x, x));
                        state.sort();
                    }
                    (None, Some(right_range)) => {
                        let right_range = state.remove(right_range);

                        state.push(MyRange::new(start_x, right_range.1));
                        state.sort();
                    }
                    (Some(left_range), None) => {
                        let left_range = state.remove(left_range);

                        state.push(MyRange::new(left_range.0, x));
                        state.sort();
                    }
                }
            }
            Corner::BottomLeft(x, y) => {
                if y != prev_y {
                    for y in prev_y..y {
                        for r in state.iter() {
                            for x in r.0..r.1 {
                                if !walls.contains_key(&Vector2::new([x, y])) {
                                    counter += 1;
                                }
                            }
                        }
                    }
                }

                prev_y = y;
                start = Some((x, corner))
            }
            Corner::BottomRight(x, _) => {
                let (start_x, _) = start.unwrap();
                start = None;
                let left_range = state.iter().position(|s| (s.0..=s.1).contains(&start_x));
                let right_range = state.iter().position(|s| (s.0..=s.1).contains(&x));
                match (left_range, right_range) {
                    (Some(left_range), Some(right_range)) if left_range == right_range => {
                        let old_range = state.remove(left_range);

                        if old_range.0 < start_x {
                            state.push(MyRange::new(old_range.0, start_x));
                        }

                        state.sort();
                    }
                    (Some(left_range), Some(right_range)) => {
                        let old_right_range = state.remove(right_range);
                        let old_left_range = state.remove(left_range);

                        state.push(old_left_range.combine(old_right_range));

                        state.sort();
                    }
                    (None, None) => {
                        unreachable!();
                    }
                    (None, Some(right_range)) => {
                        let right_range = state.remove(right_range);

                        state.push(MyRange::new(start_x, right_range.1));
                        state.sort();
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    counter
}

fn get_events(input: &InputType) -> VecDeque<Corner> {
    let mut walls = exercise_1(input.clone());
    let sc = translate_s(input).0;
    walls.sort_by_key(|s| (s.0[1], s.0[0]));
    walls.dedup_by_key(|s| s.0);

    let mut queue = VecDeque::new();

    let mut iter = walls.into_iter();

    while let Some((next_pos, next_char)) = iter.next() {
        let next_char = if next_char == 'S' { sc } else { next_char };

        match next_char {
            'F' => {
                queue.push_back(Corner::TopLeft(next_pos[0], next_pos[1]));
            }
            '7' => {
                queue.push_back(Corner::TopRight(next_pos[0], next_pos[1]));
            }
            'L' => {
                queue.push_back(Corner::BottomLeft(next_pos[0], next_pos[1]));
            }
            'J' => {
                queue.push_back(Corner::BottomRight(next_pos[0], next_pos[1]));
            }
            _ => {}
        }
    }

    queue
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Corner {
    TopLeft(isize, isize),
    TopRight(isize, isize),
    BottomLeft(isize, isize),
    BottomRight(isize, isize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct MyRange(isize, isize);

impl MyRange {
    fn new(start: isize, end: isize) -> Self {
        assert!(start < end, "{}, {}", start, end);
        Self(start, end)
    }

    fn contains(&self, value: isize) -> bool {
        self.0 <= value && value <= self.1
    }

    fn combine(&self, other: Self) -> Self {
        Self::new(self.0.min(other.0), self.1.max(other.1))
    }
}
