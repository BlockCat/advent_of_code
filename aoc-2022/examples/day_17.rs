use std::collections::HashMap;

use aoc_2022::direction::Direction;

type InputType = Vec<Direction>;

const SHAPES: [RockShape; 5] = [
    RockShape::Min,
    RockShape::Plus,
    RockShape::Corner,
    RockShape::Straight,
    RockShape::Square,
];

pub fn main() {
    let numbers = input();

    println!("Exercise 1: {}", exercise_2(numbers.clone(), 2022));
    println!("Exercise 2: {}", exercise_2(numbers, 1000000000000));
}

fn input() -> InputType {
    parse_line(include_str!("../input/day_17.txt"))
}

fn parse_line(line: &str) -> Vec<Direction> {
    line.chars()
        .map(|x| match x {
            '<' => Direction::West,
            '>' => Direction::East,
            a => unreachable!("un: {}", a),
        })
        .collect()
}

fn exercise_1(input: InputType, l: usize) -> usize {
    let mut room: Vec<u16> = Vec::with_capacity(l * 3);
    let shapes = SHAPES.into_iter().cycle().take(l);

    let mut jets = input.into_iter().cycle();

    'shape: for fig in shapes {
        let mut y = {
            let pos = room
                .iter()
                .rev()
                .position(|x| (*x & 0b0111_11110) > 0)
                .unwrap_or(0);
            let pos = room.len() - pos;
            // println!("y: {}", pos + 3);

            let pos = pos + 3;

            while room.len() < pos {
                room.push(0b1000_00001);
            }
            pos
        };

        let stencil: Vec<u16> = fig.to_stencil();

        for _ in room.len()..y + stencil.len() {
            room.push(0b1000_00001);
        }
        let mut x = 2usize;

        loop {
            let dir = jets.next().unwrap();
            let next_x = match dir {
                Direction::East => x + 1,
                Direction::West => x.checked_sub(1).unwrap_or(0),
                _ => unreachable!(),
            };
            x = if overlaps(&room, &stencil, next_x, y as isize) {
                x
            } else {
                next_x
            };

            if overlaps(&room, &stencil, x, y as isize - 1) {
                place_shape(&mut room, &stencil, x, y);
                continue 'shape;
            } else {
                y -= 1;
            }
        }
    }

    room.len()
        - room
            .iter()
            .rev()
            .position(|x| (*x & 0b0111_11110) > 0)
            .unwrap()
}

fn overlaps(room: &Vec<u16>, stencil: &Vec<u16>, x: usize, y: isize) -> bool {
    if y < 0 {
        return true;
    }
    let y = y as usize;

    !room[y..(y + stencil.len())]
        .iter()
        .zip(stencil)
        .all(|(a, b)| (a & (b >> x)) == 0)
}

fn place_shape(room: &mut Vec<u16>, stencil: &Vec<u16>, x: usize, y: usize) {
    room[y..(y + stencil.len())]
        .iter_mut()
        .zip(stencil)
        .for_each(|(a, b)| *a |= b >> x);
}

fn exercise_2(input: InputType, rocks: usize) -> usize {
    let mut room: Vec<u16> = Vec::with_capacity(2022 * 3);
    let mut shapes = SHAPES.into_iter().cycle().enumerate().take(rocks);

    println!("{}", input.len());
    let mut jets = input.iter().enumerate().cycle().peekable();
    let mut add_height = 0;

    let mut visited = HashMap::new();

    'shape: while let Some((time, fig)) = shapes.next() {
        let pos = room
            .windows(2)
            .position(|x| x.iter().fold(0, |acc, x| acc | x) == 0b1111_11111);
        if let Some(x) = pos {
            let current_height = add_height + room.len()
                - room
                    .iter()
                    .rev()
                    .position(|x| (*x & 0b0111_11110) > 0)
                    .unwrap();

            let jet = jets.peek().unwrap();

            if let Some((prev_time, prev_height)) =
                visited.insert((x + 1, jet.0), (time, current_height))
            {
                let cycle_len = time - prev_time;
                let remaining = rocks - time;
                let parts = remaining / cycle_len;
                let skip = parts * cycle_len;
                let diff = current_height - prev_height;
                let nrem = rocks - skip;

                return exercise_1(input, nrem) + diff * parts;
            }

            add_height += x + 1;
            room = room[x + 1..].to_vec();
        }

        let mut y = {
            let pos = room
                .iter()
                .rev()
                .position(|x| (*x & 0b0111_11110) > 0)
                .unwrap_or(0);
            let pos = room.len() - pos;

            let pos = pos + 3;

            while room.len() < pos {
                room.push(0b1000_00001);
            }
            pos
        };

        let mut x = 2usize;

        let stencil: Vec<u16> = fig.to_stencil();

        for _ in room.len()..y + stencil.len() {
            room.push(0b1000_00001);
        }

        loop {
            let (_, dir) = jets.next().unwrap();
            let next_x = match dir {
                Direction::East => x + 1,
                Direction::West => x.checked_sub(1).unwrap_or(0),
                _ => unreachable!(),
            };
            x = if overlaps(&room, &stencil, next_x, y as isize) {
                x
            } else {
                next_x
            };

            if overlaps(&room, &stencil, x, y as isize - 1) {
                place_shape(&mut room, &stencil, x, y);

                continue 'shape;
            } else {
                // println!("{},{}", x, y);
                y -= 1;
            }
        }
    }
    room.reverse();
    add_height + room.len() - room.iter().position(|x| (*x & 0b0111_11110) > 0).unwrap()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RockShape {
    Min,
    Plus,
    Corner,
    Straight,
    Square,
}

impl RockShape {
    fn to_stencil(self) -> Vec<u16> {
        match self {
            RockShape::Min => {
                vec![0b0011_1100 << 2]
            }
            RockShape::Plus => {
                vec![0b010 << 5, 0b111 << 5, 0b010 << 5]
            }
            RockShape::Corner => {
                vec![0b111 << 5, 0b001 << 5, 0b001 << 5]
            }
            RockShape::Straight => {
                vec![0b1 << 7, 0b1 << 7, 0b1 << 7, 0b1 << 7]
            }
            RockShape::Square => {
                vec![0b11 << 6, 0b11 << 6]
            }
        }
    }
}
