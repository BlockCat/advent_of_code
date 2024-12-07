use std::{
    collections::{HashMap, HashSet, VecDeque},
    hash::Hash,
};

use aoc_2024::{
    direction::Direction,
    grid::{DynamicGrid, Grid, StaticGrid},
    stopwatch,
    vector::Vector2,
};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

type Input = StaticGrid<char>;

// wrong: 2151

pub fn main() {
    let input = include_str!("../input/day_06.txt");
    let l = stopwatch(|| {
        let input = parse(input);
        let a1 = exercise_1(&input);
        println!("Ex1: {}", a1);
        let a2 = exercise_2(&input);
        println!("Ex2: {}", a2);
    });

    println!("Time: {:?}", l);
}

fn parse(input: &str) -> Input {
    let l = input.lines().map(parse_line).collect::<Vec<_>>();
    StaticGrid::from_vec(l)
}

fn parse_line(input: &str) -> Vec<char> {
    input.chars().collect()
}

fn exercise_1(input: &Input) -> usize {
    let guard = GuardIter::new(input, |a, b| a.get_vec(&b) == Some(&'#'));
    guard.map(|x| x.0).collect::<HashSet<_>>().len() + 1
}

fn exercise_2(input: &Input) -> usize {
    let cur_pos = input.iter().find(|a| a.1 == &'^').unwrap().0;
    let cur_dir = Direction::North;

    let guard = GuardIter::new(input, |a, b| a.get_vec(&b) == Some(&'#'));
    let path = guard.map(|x| x.0).collect::<HashSet<_>>();

    path.into_par_iter()
        .filter(|&x| does_block(input, cur_pos, cur_dir, x))
        .count()
}

fn does_block(input: &Input, cur_pos: Vector2, cur_dir: Direction, extra_block: Vector2) -> bool {
    let guard = GuardIter::raw_new(input, cur_pos, cur_dir, |a, b| {
        a.get_vec(&b) == Some(&'#') || b == extra_block
    });

    guard.last().unwrap().2 == State::RepeatPosition
}

#[derive(Clone, Debug, PartialEq)]
enum State {
    Position,
    Left,
    RepeatPosition,
}

struct GuardIter<'a, F>
where
    F: Fn(&Input, Vector2) -> bool,
{
    input: &'a Input,
    state: State,
    cur_pos: Vector2,
    cur_dir: Direction,
    visited: HashSet<(Vector2, Direction)>,
    is_block: F,
}

impl<'a, F> GuardIter<'a, F>
where
    F: Fn(&Input, Vector2) -> bool,
{
    pub fn new(input: &'a Input, is_block: F) -> Self {
        let pos = input.iter().find(|a| a.1 == &'^').unwrap().0;
        Self::raw_new(input, pos, Direction::North, is_block)
    }

    pub fn raw_new(input: &'a Input, cur_pos: Vector2, cur_dir: Direction, is_block: F) -> Self {
        let mut visited = HashSet::new();
        visited.insert((cur_pos, Direction::North));
        Self {
            input,
            state: State::Position,
            cur_pos: cur_pos,
            cur_dir: cur_dir,
            visited: visited,
            is_block,
        }
    }
}

impl<'a, F> Iterator for GuardIter<'a, F>
where
    F: Fn(&Input, Vector2) -> bool,
{
    type Item = (Vector2, Direction, State);

    fn next(&mut self) -> Option<Self::Item> {
        match self.state {
            State::Position => {
                let next_pos = self.cur_pos + self.cur_dir;
                if next_pos[0] < 0
                    || next_pos[1] < 0
                    || next_pos[0] >= self.input.width as isize
                    || next_pos[1] >= self.input.height as isize
                {
                    self.state = State::Left;
                    return None;
                }
                let (next_pos, next_dir) = if (self.is_block)(self.input, next_pos) {
                    (self.cur_pos, self.cur_dir.right())
                } else {
                    (next_pos, self.cur_dir)
                };

                self.cur_pos = next_pos;
                self.cur_dir = next_dir;
                self.state = if self.visited.insert((next_pos, next_dir)) {
                    State::Position
                } else {
                    State::RepeatPosition
                };

                Some((next_pos, next_dir, self.state.clone()))
            }
            State::Left => None,
            State::RepeatPosition => None,
        }
    }
}
