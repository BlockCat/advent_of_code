use aoc_2022::{
    grid::{Grid, StaticGrid},
    vector::Vector2,
};
use std::{collections::HashSet, thread};

type IT = StaticGrid<u8>;

pub fn main() {
    let numbers = input();
    println!("1: {}", exercise_1(numbers.clone()));
    println!("2: {}", exercise_2(numbers));
}

fn input() -> IT {
    let l = include_str!("../input/day_08.txt")
        .lines()
        .map(|line| line.chars().map(|x| x as u8 - b'0').collect())
        .collect();
    StaticGrid::from_vec(l)
}

fn exercise_1(input: IT) -> usize {
    let mut visible = HashSet::new();
    let height = input.height as isize;
    let width = input.width as isize;

    let mut cc = |acc, x, y| {
        let val = *input.get_vec(&Vector2::new([x, y])).unwrap() as i32;
        if val > acc {
            visible.insert((x, y));
            val
        } else {
            acc
        }
    };

    for y in 0..height {
        (0..width).fold(-1i32, |acc, x| cc(acc, x, y));
        (0..width).rev().fold(-1i32, |acc, x| cc(acc, x, y));
    }
    for x in 0..width {
        (0..height).fold(-1i32, |acc, y| cc(acc, x, y));
        (0..height).rev().fold(-1i32, |acc, y| cc(acc, x, y));
    }

    visible.len()
}
fn exercise_2(input: IT) -> usize {
    let prep = thread::scope(|s| {
        [
            s.spawn(|| prep_left(&input)),
            s.spawn(|| prep_right(&input)),
            s.spawn(|| prep_up(&input)),
            s.spawn(|| prep_down(&input)),
        ]
        .map(|x| x.join().unwrap())
    });

    (0..input.grid.len())
        .map(|x| prep[0].grid[x] * prep[1].grid[x] * prep[2].grid[x] * prep[3].grid[x])
        .max()
        .unwrap()
}

fn prep_left(input: &IT) -> StaticGrid<usize> {
    let mut grid = StaticGrid::new(input.width, input.height);

    for y in 0..input.height as isize {
        let mut cat = [0u32; 10];
        for x in 0..input.width as isize {
            let val = *input.get(x, y).unwrap();
            let seen = *cat[val as usize..].iter().max().unwrap();
            grid.set(x, y, x as usize - seen as usize);
            cat[val as usize] = x as u32;
        }
    }
    grid
}

fn prep_right(input: &IT) -> StaticGrid<usize> {
    let mut grid = StaticGrid::new(input.width, input.height);

    for y in 0..input.height as isize {
        let mut cat = [input.width as u32 - 1; 10];
        for x in (0..input.width as isize).rev() {
            let val = *input.get(x, y).unwrap();
            let seen = *(cat[val as usize..].iter().min().unwrap());
            grid.set(x, y, seen as usize - x as usize);
            cat[val as usize] = x as u32;
        }
    }
    grid
}

fn prep_up(input: &IT) -> StaticGrid<usize> {
    let mut grid = StaticGrid::new(input.width, input.height);

    for x in 0..input.width as isize {
        let mut cat = [0u32; 10];
        for y in 0..input.height as isize {
            let val = *input.get(x, y).unwrap();
            let seen = *cat[val as usize..].iter().max().unwrap();
            grid.set(x, y, (y as usize) - (seen as usize));
            cat[val as usize] = y as u32;
        }
    }
    grid
}

fn prep_down(input: &IT) -> StaticGrid<usize> {
    let mut grid = StaticGrid::new(input.width, input.height);

    for x in (0..input.width as isize).rev() {
        let mut cat = [input.height as u32 - 1; 10];
        for y in (0..input.height as isize).rev() {
            let val = *input.get(x, y).unwrap();
            let seen = *(cat[val as usize..].iter().min().unwrap());
            grid.set(x, y, seen as usize - y as usize);
            cat[val as usize] = y as u32;
        }
    }
    grid
}
