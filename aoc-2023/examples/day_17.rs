use aoc_2023::{
    direction::Direction,
    grid::{Grid, StaticGrid},
    vector::Vector2,
};
use std::collections::{BinaryHeap, HashSet};

type InputType = StaticGrid<u8>;

pub fn main() {
    let input = parse(include_str!("../input/day_17_big.txt"));

    println!("Exercise 1: {}", exercise_2(input.clone(), 1, 3));
    println!("Exercise 2: {}", exercise_2(input, 4, 10));
}

#[test]
fn test() {
    let input = parse(include_str!("../input/test.txt"));

    assert_eq!(102, exercise_2(input.clone(), 1, 3));
    assert_eq!(94, exercise_2(input, 4, 10));
}

fn parse<'a>(input: &'a str) -> InputType {
    let r = input
        .lines()
        .map(|s| s.chars().map(|s| s.to_digit(10).unwrap() as u8).collect())
        .collect();

    StaticGrid::from_vec(r)
}

fn exercise_2(input: InputType, min: usize, max: usize) -> usize {
    let start_pos = Vector2::new([0, 0]);
    let dest_pos = Vector2::new([input.width as isize - 1, input.height as isize - 1]);

    let mut queue = BinaryHeap::new();

    [Direction::East, Direction::South].iter().for_each(|&dir| {
        insert_range(0, start_pos, dir, &input, &mut queue, min, max);
    });

    let mut visited = HashSet::new();

    while let Some(entry) = queue.pop() {
        
        if !visited.insert((entry.pos, entry.dir)) {
            continue;
        } else if entry.pos == dest_pos {
            return entry.cost as usize;
        }
        insert_range(
            entry.cost,
            entry.pos,
            entry.dir.left(),
            &input,
            &mut queue,
            min,
            max,
        );
        insert_range(
            entry.cost,
            entry.pos,
            entry.dir.right(),
            &input,
            &mut queue,
            min,
            max,
        );
    }
    unreachable!()
}

fn insert_range(
    cost: usize,
    pos: Vector2,
    dir: Direction,
    input: &InputType,
    queue: &mut BinaryHeap<Entry>,
    min: usize,
    max: usize,
) {
    let insertions = (0..)
        .scan((cost, pos), |acc, _| {
            if let Some(hl) = input.get_vec(&(acc.1 + dir)) {
                *acc = (acc.0 + *hl as usize, acc.1 + dir);
                Some(*acc)
            } else {
                None
            }
        })
        .take(max)
        .skip(min - 1)
        .map(|(hl, pos)| Entry::new(hl, pos, dir));

    queue.extend(insertions);
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct Entry {
    cost: usize,
    pos: Vector2,
    dir: Direction,
}

impl Entry {
    fn new(cost: usize, pos: Vector2, dir: Direction) -> Self {
        Self { cost, pos, dir }
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.cost.partial_cmp(&self.cost)
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}
