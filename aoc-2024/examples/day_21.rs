use aoc_2024::{direction::Direction, stopwatch, vector::Vector2};
use std::collections::{HashMap, VecDeque};

type Input = Vec<[Button; 4]>;

// too high: 21867034958466832
// //        205620604017764

const ACTIONS: [RobotInstruction; 5] = [
    RobotInstruction::Move(Direction::North),
    RobotInstruction::Move(Direction::East),
    RobotInstruction::Move(Direction::South),
    RobotInstruction::Move(Direction::West),
    RobotInstruction::Press,
];

// one pad
// two pad
// three pad
pub fn main() {
    let numbers = input(include_str!("../input/day_21.txt"));
    // let numbers = input(include_str!("../input/test.txt")); // ,

    let time = stopwatch(|| {
        println!("Exercise 1: {}", exercise_1::<2>(&numbers));
        println!("Exercise 2: {}", exercise_1::<25>(&numbers));
    });

    println!("time: {:?}", time);
}

fn input(input: &str) -> Input {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> [Button; 4] {
    let mut ch = line.chars();

    let a = ch.next().unwrap() as u8 - b'0';
    let b = ch.next().unwrap() as u8 - b'0';
    let c = ch.next().unwrap() as u8 - b'0';

    [
        Button::Number(a),
        Button::Number(b),
        Button::Number(c),
        Button::Final,
    ]
}

fn exercise_1<const S: usize>(input: &Input) -> usize {
    let mut cache = HashMap::new();

    input
        .iter()
        .map(|x| complexity(x, calculate_shortest_path_len(S as u8 + 1, x, &mut cache)))
        .sum()
}

fn calculate_possible_first_levels(pos: Vector2, target: Vector2) -> Vec<Vec<RobotInstruction>> {
    find_shortest_path(pos, target, [3, 4], Vector2::new([0, 3]))
        .into_iter()
        .map(|mut x| {
            x.push(RobotInstruction::Press);
            x
        })
        .collect::<Vec<_>>()
}

fn calculate_shortest_path_len(
    level: u8,
    numbers: &[Button; 4],
    cache: &mut HashMap<(Vector2, u8, RobotInstruction), usize>,
) -> usize {
    let initial_paths = calculate_possible_first_levels(Vector2::new([2, 3]), numbers[0].to_pos());

    let combi = numbers[1..]
        .iter()
        .fold(
            (initial_paths, numbers[0].to_pos()),
            |(existing_paths, pos), x| {
                let shortest_paths = calculate_possible_first_levels(pos, x.to_pos());

                let next_paths = existing_paths
                    .into_iter()
                    .flat_map(|p| {
                        shortest_paths.iter().cloned().map(move |x| {
                            let mut path = p.clone();
                            path.append(&mut x.clone());
                            path
                        })
                    })
                    .collect();

                (next_paths, x.to_pos())
            },
        )
        .0;

    let mut mmm = usize::MAX;

    for c in &combi {
        mmm = mmm.min(
            c.into_iter()
                .fold((Vector2::new([2, 0]), 0), |acc, x| {
                    (
                        x.to_pos(),
                        acc.1 + find_instruction(level - 1, acc.0, *x, cache),
                    )
                })
                .1,
        );
        // println!();
    }

    // for c in &combi {
    //     println!("level: {}", level);
    //     print_path(c);
    //     println!();
    // }

    mmm
}

// to go to and execute instruction on level
fn find_instruction(
    level: u8,
    pos: Vector2,
    instruction: RobotInstruction,
    cache: &mut HashMap<(Vector2, u8, RobotInstruction), usize>,
) -> usize {
    if level == 0 {
        1
    } else if let Some(cached) = cache.get(&(pos, level, instruction)) {
        *cached
    } else {
        let source = pos;
        let target = instruction.to_pos();

        let pf1 = find_shortest_path(source, target, [3, 2], Vector2::zero());

        let sum = pf1
            .into_iter()
            .map(|mut path| {
                path.push(RobotInstruction::Press);
                path.into_iter()
                    .fold((Vector2::new([2, 0]), 0), |acc, x| {
                        (
                            x.to_pos(),
                            acc.1 + find_instruction(level - 1, acc.0, x, cache),
                        )
                    })
                    .1
            })
            .min()
            .unwrap();

        cache.insert((pos, level, instruction), sum);

        sum
    }
}

fn find_shortest_path(
    pos: Vector2,
    target: Vector2,
    bounds: [usize; 2],
    gap: Vector2,
) -> Vec<Vec<RobotInstruction>> {
    let mut queue = VecDeque::new();
    queue.push_back((pos, vec![]));

    let mut all_shortests = Vec::new();

    let mut shortest = usize::MAX;

    while let Some((pos, path)) = queue.pop_front() {
        if pos == target {
            if path.len() > shortest {
                return all_shortests;
            }

            shortest = shortest.min(path.len());
            all_shortests.push(path);

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
                    let mut path = path.clone();
                    path.push(action);
                    queue.push_back((np, path));
                }
                RobotInstruction::Press => {}
            }
        }
    }

    all_shortests
}

fn complexity(line: &[Button], instructions: usize) -> usize {
    let mut com = match line[0] {
        Button::Number(a) => a as usize,
        Button::Final => unreachable!(),
    };
    com *= 10;
    com += match line[1] {
        Button::Number(a) => a as usize,
        Button::Final => unreachable!(),
    };
    com *= 10;
    com += match line[2] {
        Button::Number(a) => a as usize,
        Button::Final => unreachable!(),
    };
    instructions * com
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
            _ => unreachable!(),
        }
    }
}
