use std::{collections::VecDeque, path::Iter};

type Input = Vec<Vec<BlockEntry>>;

pub fn main() {
    let input = parse_input(include_str!("../input/day10.txt"));
    println!("Ex1: {}", exercise_1(&input));
    println!("Ex2: {}", exercise_2(&input));
}

fn parse_input(input: &str) -> Input {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> Vec<BlockEntry> {
    line.chars().map(BlockEntry::from).collect()
}

fn exercise_1(input: &Input) -> usize {
    input
        .iter()
        .map(|X| check_line(X))
        .filter_map(|x| match x {
            CheckResult::Corrupted(c) => Some(c),
            CheckResult::Incomplete(_) => None,
        })
        .map(|x| match x {
            Char::A => 3,
            Char::B => 57,
            Char::C => 1197,
            Char::D => 25137,
        })
        .sum()
}

fn exercise_2(input: &Input) -> usize {
    let mut res: Vec<usize> = input
        .iter()
        .map(|x| check_line(x))
        .filter_map(|x| match x {
            CheckResult::Corrupted(_) => None,
            CheckResult::Incomplete(d) => Some(d),
        })
        .map(|x| {
            x.into_iter().fold(0, |acc, x| {
                acc * 5
                    + match x {
                        Char::A => 1,
                        Char::B => 2,
                        Char::C => 3,
                        Char::D => 4,
                    }
            })
        })
        .collect();

    res.sort();
    res[res.len() / 2]
}

fn check_line(iterator: &[BlockEntry]) -> CheckResult {
    let mut block_stack = VecDeque::new();
    for chunk in iterator {
        match chunk {
            BlockEntry::In(c) => {
                block_stack.push_front(*c);
            }
            BlockEntry::Out(c) => {
                let open = block_stack.pop_front().expect("Could not get");
                if open != *c {
                    return CheckResult::Corrupted(*c);
                }
            }
        }
    }

    if !block_stack.is_empty() {
        return CheckResult::Incomplete(block_stack);
    }
    unreachable!()
}

enum CheckResult {
    Corrupted(Char),
    Incomplete(VecDeque<Char>),
}

impl From<char> for BlockEntry {
    fn from(c: char) -> Self {
        match c {
            '(' => BlockEntry::In(Char::A),
            '[' => BlockEntry::In(Char::B),
            '{' => BlockEntry::In(Char::C),
            '<' => BlockEntry::In(Char::D),
            ')' => BlockEntry::Out(Char::A),
            ']' => BlockEntry::Out(Char::B),
            '}' => BlockEntry::Out(Char::C),
            '>' => BlockEntry::Out(Char::D),
            _ => unreachable!(),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum BlockEntry {
    In(Char),
    Out(Char),
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Char {
    A,
    B,
    C,
    D,
}
