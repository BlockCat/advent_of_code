use std::{collections::HashMap, hash::Hash};

type InputType = Vec<(Vec<Status>, Vec<usize>)>;

pub fn main() {
    let input = parse(include_str!("../input/day_12.txt"));

    println!("Exercise 1: {}", exercise_1(input.clone()));
    println!(
        "Exercise 2: {}",
        exercise_1(
            input
                .into_iter()
                .map(|(mut a, b)| {
                    a.push(Status::Unknown);
                    a = a.repeat(5);
                    a.remove(a.len() - 1);

                    (a, b.repeat(5))
                })
                .collect::<Vec<_>>()
        )
    );
}

fn parse<'a>(input: &'a str) -> InputType {
    input.lines().map(|line| parse_line(line)).collect()
}

fn parse_line(line: &str) -> (Vec<Status>, Vec<usize>) {
    let (spring, numbers) = line.split_once(" ").unwrap();

    let springs = spring.chars().map(|c| Status::from(c)).collect::<Vec<_>>();

    let numbers = numbers
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    (springs, numbers)
}

fn exercise_1(input: InputType) -> usize {
    input
        .iter()
        .map(|(line, numbers)| possible_solution(line, numbers))
        .sum::<usize>()
}

fn possible_solution(line: &Vec<Status>, numbers: &Vec<usize>) -> usize {
    let mut visited = HashMap::new();

    step(line, numbers, 0, 0, &mut visited)
}

fn step(
    line: &[Status],
    numbers: &[usize],
    line_index: usize,
    number_index: usize,
    visited: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if let Some(n) = visited.get(&(line_index, number_index)) {
        return *n;
    }

    if numbers.is_empty() {
        return !line.iter().any(|s| s == &Status::Broken) as usize;
    }
    let amount = numbers[0];

    let take_not_group = |visited: &mut HashMap<(usize, usize), usize>| {
        step(&line[1..], numbers, line_index + 1, number_index, visited)
    };

    let take_group = |visited: &mut HashMap<(usize, usize), usize>| {
        step(
            &line[(amount + 1).min(line.len())..],
            &numbers[1..],
            line_index + amount + 1,
            number_index + 1,
            visited,
        )
    };

    let s = if line.is_empty() {
        visited.insert((line_index, number_index), 0);
        return 0;
    } else if numbers.len() * 2 - 1 > line.len() {
        // Is there space left to fulfil demand
        visited.insert((line_index, number_index), 0);
        return 0;
    } else if amount > line.len() {
        visited.insert((line_index, number_index), 0);
        return 0;
    } else if line[0] == Status::Operational {
        let s = take_not_group(visited);
        visited.insert((line_index, number_index), s);
        return s;
    } else {
        let has_to_put_it_in = line[0] == Status::Broken;
        let can_put_it_in = can_put_it_in(line, amount);
        match (has_to_put_it_in, can_put_it_in) {
            (true, true) => take_group(visited),
            (true, false) => 0,
            (false, true) => take_group(visited) + take_not_group(visited),
            (false, false) => take_not_group(visited),
        }
    };

    visited.insert((line_index, number_index), s);
    s
}

fn can_put_it_in(line: &[Status], amount: usize) -> bool {
    if amount > line.len() {
        return false;
    }

    let next_ok = amount == line.len() || line[amount] != Status::Broken;

    next_ok && line[0..amount].iter().all(|s| s != &Status::Operational)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Status {
    Unknown,
    Broken,
    Operational,
}

impl From<char> for Status {
    fn from(c: char) -> Self {
        match c {
            '?' => Status::Unknown,
            '#' => Status::Broken,
            '.' => Status::Operational,
            _ => panic!("Invalid input"),
        }
    }
}

// 1005
// 6155
// 10230
// 9667
// 5447
