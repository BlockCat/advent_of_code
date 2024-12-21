#![feature(iter_map_windows)]

use aoc_2024::{direction::Direction, stopwatch, vector::Vector2};
use std::collections::{HashMap, VecDeque};

type Input = Vec<[Button; 4]>;

const ACTIONS: [RobotInstruction; 4] = [
    RobotInstruction::Move(Direction::North),
    RobotInstruction::Move(Direction::East),
    RobotInstruction::Move(Direction::South),
    RobotInstruction::Move(Direction::West),
];

pub fn main() {
    let numbers = input(include_str!("../input/day_21.txt"));
    // let numbers = input(include_str!("../input/test.txt")); // ,

    let time = stopwatch(|| {
        println!("Exercise 1: {}", exercise_1(2, &numbers));
        println!("Exercise 2: {}", exercise_1(25, &numbers));
    });

    println!("time: {:?}", time);
}

fn input(input: &str) -> Input {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> [Button; 4] {
    let mut ch = line.chars().map(|x| x as u8 - b'0').map(Button::Number);

    [
        ch.next().unwrap(),
        ch.next().unwrap(),
        ch.next().unwrap(),
        Button::Final,
    ]
}

fn exercise_1(robots: u8, input: &Input) -> usize {
    let mut cache = HashMap::new();

    input
        .iter()
        .map(|x| complexity(x, calculate_shortest_path_len(robots + 1, x, &mut cache)))
        .sum()
}

fn calculate_shortest_path_len(
    robot: u8,
    numbers: &[Button; 4],
    cache: &mut HashMap<(Vector2, u8, RobotInstruction), usize>,
) -> usize {
    [Vector2::new([2, 3])]
        .into_iter()
        .chain(numbers.iter().map(|x| x.to_pos()))
        .map_windows(|[source, target]| {
            fun_name(robot, *source, *target, [3, 4], Vector2::new([0, 3]), cache)
        })
        .sum()
}

fn fun_name(
    robot: u8,
    source: Vector2,
    target: Vector2,
    bounds: [usize; 2],
    gap: Vector2,
    cache: &mut HashMap<(aoc_2024::vector::VectorN<2>, u8, RobotInstruction), usize>,
) -> usize {
    find_shortest_path(robot, source, target, bounds, gap, cache)
        .into_iter()
        .min()
        .unwrap()
}

// to go to and execute instruction on level
fn find_instruction(
    robot: u8,
    pos: Vector2,
    instruction: RobotInstruction,
    cache: &mut HashMap<(Vector2, u8, RobotInstruction), usize>,
) -> usize {
    if robot == 0 {
        1
    } else if let Some(cached) = cache.get(&(pos, robot, instruction)) {
        *cached
    } else {
        let source = pos;
        let target = instruction.to_pos();

        let sum = fun_name(robot, source, target, [3, 2], Vector2::zero(), cache);

        cache.insert((pos, robot, instruction), sum);

        sum
    }
}

fn find_shortest_path(
    robot: u8,
    start_pos: Vector2,
    target: Vector2,
    bounds: [usize; 2],
    gap: Vector2,
    cache: &mut HashMap<(Vector2, u8, RobotInstruction), usize>,
) -> Vec<usize> {
    let mut queue = VecDeque::new();
    queue.push_back((start_pos, 0, 0, RobotInstruction::Press));

    let mut all_shortests = Vec::new();

    let mut shortest = usize::MAX;

    while let Some((pos, score, len, path)) = queue.pop_front() {
        if pos == target {
            if len > shortest {
                return all_shortests;
            }

            shortest = shortest.min(len);
            let score =
                score + find_instruction(robot - 1, path.to_pos(), RobotInstruction::Press, cache);

            all_shortests.push(score);

            continue;
        }
        for action in ACTIONS {
            match action {
                RobotInstruction::Move(direction) => {
                    let np = pos + direction;
                    if np == gap {
                        continue;
                    }
                    if !np.bounded(bounds) {
                        continue;
                    }
                    let score = score + find_instruction(robot - 1, path.to_pos(), action, cache);

                    queue.push_back((np, score, len + 1, action));
                }
                _ => unreachable!(),
            }
        }
    }

    all_shortests
}

fn complexity(line: &[Button], instructions: usize) -> usize {
    line.iter()
        .take(3)
        .map(|i| {
            if let Button::Number(b) = i {
                *b as usize
            } else {
                unreachable!()
            }
        })
        .fold(0, |acc, x| acc * 10 + x)
        * instructions
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum RobotInstruction {
    Move(Direction),
    Press,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Button {
    Number(u8),
    Final,
}

impl Button {
    fn to_pos(&self) -> Vector2 {
        match self {
            Button::Number(0) => Vector2::new([1, 3]),

            Button::Number(1) => Vector2::new([0, 2]),
            Button::Number(2) => Vector2::new([1, 2]),
            Button::Number(3) => Vector2::new([2, 2]),

            Button::Number(4) => Vector2::new([0, 1]),
            Button::Number(5) => Vector2::new([1, 1]),
            Button::Number(6) => Vector2::new([2, 1]),

            Button::Number(7) => Vector2::new([0, 0]),
            Button::Number(8) => Vector2::new([1, 0]),
            Button::Number(9) => Vector2::new([2, 0]),
            Button::Final => Vector2::new([2, 3]),
            _ => unreachable!(),
        }
    }
}

impl RobotInstruction {
    fn to_pos(&self) -> Vector2 {
        match self {
            RobotInstruction::Move(Direction::North) => Vector2::new([1, 0]),
            RobotInstruction::Press => Vector2::new([2, 0]),
            RobotInstruction::Move(Direction::West) => Vector2::new([0, 1]),
            RobotInstruction::Move(Direction::South) => Vector2::new([1, 1]),
            RobotInstruction::Move(Direction::East) => Vector2::new([2, 1]),
        }
    }
}
