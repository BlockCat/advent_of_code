use std::collections::HashMap;

use aoc_2022::{
    direction::Direction,
    grid::{Grid, StaticGrid},
    vector::Vector2,
};

type InputType = (Vec<Instruction>, StaticGrid<char>);

pub fn main() {
    let numbers = input();

    // println!("Exercise 1: {}", exercise_1(numbers.clone()));
    // 148382 too high
    println!("Exercise 2: {}", exercise_2(numbers));
}

fn input() -> InputType {
    let mut lines = include_str!("../input/day_22.txt").lines();

    let grid = StaticGrid::from_vec(
        lines
            .by_ref()
            .take_while(|x| !x.is_empty())
            .map(parse_line)
            .collect(),
    );

    let instructions = lines.next().unwrap();
    assert!(!instructions.is_empty());

    let (mut a, b) = instructions
        .chars()
        .fold((Vec::new(), 0), |(mut col, num), x| match x {
            'R' => {
                if num > 0 {
                    col.push(Instruction::Forward(num));
                }
                col.push(Instruction::Right);
                (col, 0)
            }
            'L' => {
                if num > 0 {
                    col.push(Instruction::Forward(num));
                }
                col.push(Instruction::Left);
                (col, 0)
            }
            c if ('0'..='9').contains(&c) => (col, num * 10 + (c as usize - b'0' as usize)),
            _ => unreachable!(),
        });

    if b > 0 {
        a.push(Instruction::Forward(b));
    }

    (a, grid)
}

fn parse_line(line: &str) -> Vec<char> {
    line.chars().collect()
}

fn exercise_1((instructions, grid): InputType) -> isize {
    let mut pos = Vector2::new([
        grid.grid.iter().position(|x| *x == '.').unwrap() as isize,
        0,
    ]);
    let mut dir = Direction::East;

    for x in instructions {
        print!("i: {:?}, ", x);
        match x {
            Instruction::Forward(l) => {
                pos = (0..l)
                    .try_fold(pos, |acc, _| tick_pos_1(acc, dir, &grid))
                    .unwrap();
            }
            Instruction::Left => dir = dir.left(),
            Instruction::Right => dir = dir.right(),
        }
    }

    let facing = match dir {
        Direction::East => 0,
        Direction::South => 1,
        Direction::West => 2,
        Direction::North => 3,
    };

    println!("\n{:?}, {:?}", pos, dir);
    4 * (pos[0] + 1) + 1000 * (pos[1] + 1) + facing
}

fn exercise_2((instructions, grid): InputType) -> isize {
    let mut pos = Vector2::new([
        grid.grid.iter().position(|x| *x == '.').unwrap() as isize,
        0,
    ]);
    let mut dir = Direction::East;

    let mapping = real_cube();

    println!("{:?}", mapping);

    for x in instructions {
        // print!("i: {:?}, ", x);
        match x {
            Instruction::Forward(l) => {
                let r = (0..l).fold((pos, dir), |(pos, dir), _| {
                    tick_pos_2(pos, dir, &grid, &mapping)
                });
                pos = r.0;
                dir = r.1;
            }
            Instruction::Left => dir = dir.left(),
            Instruction::Right => dir = dir.right(),
        }
    }

    let facing = match dir {
        Direction::East => 0,
        Direction::South => 1,
        Direction::West => 2,
        Direction::North => 3,
    };

    println!("\n{:?}, {:?}", pos, dir);
    4 * (pos[0] + 1) + 1000 * (pos[1] + 1) + facing
}

fn real_cube() -> HashMap<(Vector2, Direction), (Vector2, Direction)> {
    //   ab
    //  .12
    //  .3.
    //  45.
    // a6..
    //

    // (1, 0) -> (0,3)
    let mut a = layer_connections(50, (1, 0, Direction::North), (0, 3, Direction::West), false);

    // (1, 0) -> (0,2)
    a.extend(layer_connections(
        50,
        (1, 0, Direction::West),
        (0, 2, Direction::West),
        true,
    ));
    // (2, 0) -> 0,3
    a.extend(layer_connections(
        50,
        (2, 0, Direction::North),
        (0, 3, Direction::South),
        false,
    ));
    // 2,0 -> 1,2
    a.extend(layer_connections(
        50,
        (2, 0, Direction::East),
        (1, 2, Direction::East),
        true,
    ));
    // 2,0 -> 1,1
    a.extend(layer_connections(
        50,
        (2, 0, Direction::South),
        (1, 1, Direction::East),
        false,
    ));
    // (1, 1) -> 2,0
    a.extend(layer_connections(
        50,
        (1, 1, Direction::East),
        (2, 0, Direction::South),
        false,
    ));
    // 1,1 -> 0,2
    a.extend(layer_connections(
        50,
        (1, 1, Direction::West),
        (0, 2, Direction::North),
        false,
    ));

    // (0, 2) -> 1,1
    // (0, 2) -> 1,0

    // (1, 2) ->

    a.extend(layer_connections(
        50,
        (1, 2, Direction::East),
        (2, 0, Direction::East),
        true,
    ));
    a.extend(layer_connections(
        50,
        (1, 2, Direction::South),
        (0, 3, Direction::East),
        false,
    ));

    a
}

fn layer_connections(
    size: isize,
    s: (isize, isize, Direction),
    t: (isize, isize, Direction),
    rev: bool,
) -> HashMap<(Vector2, Direction), (Vector2, Direction)> {
    let (sx, sy, sd) = s;
    let (tx, ty, td) = t;
    let mut a = layer_connection(size, sx, sy, sd, tx, ty, td.reverse(), rev);
    let b = layer_connection(size, tx, ty, td, sx, sy, sd.reverse(), rev);

    a.extend(b);
    a
}
fn layer_connection(
    size: isize,
    sx: isize,
    sy: isize,
    sd: Direction,
    tx: isize,
    ty: isize,
    td: Direction,
    rev: bool,
) -> HashMap<(Vector2, Direction), (Vector2, Direction)> {
    let source_vecs = dir_vecs(size, sx, sy, sd, 1, false);
    let mut dest_vecs = dir_vecs(size, tx, ty, td.reverse(), 0, true);
    if rev {
        dest_vecs.reverse();
    }
    source_vecs.into_iter().zip(dest_vecs.into_iter()).collect()
}

fn dir_vecs(
    size: isize,
    sx: isize,
    sy: isize,
    sd: Direction,
    add: isize,
    rev: bool,
) -> Vec<(Vector2, Direction)> {
    let sworld_x_start = sx * size;
    let sworld_x_end = (sx + 1) * size;
    let sworld_y_start = sy * size;
    let sworld_y_end = (sy + 1) * size;
    let dir = if rev { sd.reverse() } else { sd };
    let source_vecs = match sd {
        Direction::North => (sworld_x_start..sworld_x_end)
            .map(|x| (Vector2::new([x, sworld_y_start - add]), dir))
            .collect::<Vec<_>>(),
        Direction::East => (sworld_y_start..sworld_y_end)
            .map(|y| (Vector2::new([sworld_x_end - 1 + add, y]), dir))
            .collect(),
        Direction::South => (sworld_x_start..sworld_x_end)
            .map(|x| (Vector2::new([x, sworld_y_end - 1 + add]), dir))
            .collect::<Vec<_>>(),
        Direction::West => (sworld_y_start..sworld_y_end)
            .map(|y| (Vector2::new([sworld_x_start - add, y]), dir))
            .collect(),
    };

    source_vecs
}

fn tick_pos_2(
    pos: Vector2,
    dir: Direction,
    grid: &StaticGrid<char>,
    mapping: &HashMap<(Vector2, Direction), (Vector2, Direction)>,
) -> (Vector2, Direction) {
    let candidate = pos + dir;

    let (candidate, cadir) = if let Some(a) = mapping.get(&(candidate, dir)).cloned() {
        println!("wrapped: {:?},{:?} -> {:?}", pos, dir, a);
        a
    } else {
        (candidate, dir)
    };

    match grid.get_vec(&candidate) {
        Some('#') => (pos, dir),
        Some('.') => (candidate, cadir),
        c => unreachable!("{:?}, {:?}, {:?}", c, pos, dir),
    }
}

fn test_cube() {}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Forward(usize),
    Left,
    Right,
}

fn tick_pos_1(pos: Vector2, dir: Direction, grid: &StaticGrid<char>) -> Option<Vector2> {
    let candidate = pos + dir;

    let candidate = match grid.get_vec(&candidate) {
        Some(' ') | None => match dir {
            Direction::North => {
                let y = (1..)
                    .map(|y| pos[1] + y)
                    .take_while(|y| {
                        let c = grid.get(pos[0], *y);

                        c != Some(&' ') && c != None
                    })
                    .last()
                    .unwrap();
                println!("N: {:?} -> {}", pos, y);
                Vector2::new([pos[0], y])
            }
            Direction::East => {
                let x = (1..)
                    .map(|x| pos[0] - x)
                    .take_while(|x| {
                        let c = grid.get(*x, pos[1]);

                        c != Some(&' ') && c != None
                    })
                    .last()
                    .unwrap();
                println!("E: {:?} -> {}", pos, x);
                Vector2::new([x, pos[1]])
            }
            Direction::South => {
                let y = (1..)
                    .map(|y| pos[1] - y)
                    .take_while(|y| {
                        let c = grid.get(pos[0], *y);
                        c != Some(&' ') && c != None
                    })
                    .last()
                    .unwrap();
                Vector2::new([pos[0], y])
            }
            Direction::West => {
                let x = (0..)
                    .map(|x| pos[0] + x)
                    .take_while(|x| {
                        let c = grid.get(*x, pos[1]);

                        c != Some(&' ') && c != None
                    })
                    .last()
                    .unwrap();
                Vector2::new([x, pos[1]])
            }
        },
        _ => candidate,
    };

    match grid.get_vec(&candidate) {
        Some('#') => Some(pos),
        Some('.') => Some(candidate),
        _ => unreachable!(),
    }
}
