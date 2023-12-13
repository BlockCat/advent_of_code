use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    num,
};

type InputType = Vec<(Vec<Status>, Vec<usize>)>;

pub fn main() {
    let input = parse(include_str!("../input/day_12.txt"));

    println!("Exercise 1: {}", exercise_1(input.clone()));
    // println!("Exercise 2: {}", exercise_2(input));
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

fn exercise_2(input: InputType) -> usize {
    0
}

fn possible_solution(line: &Vec<Status>, numbers: &Vec<usize>) -> usize {
    // DP
    // sol = DP(line[numbers[0]..], numbers[1..]) + DP(line[1..], numbers);

    // let mut visited = HashMap::new();

    step(line, numbers, 0, 0)
}

fn step(line: &[Status], numbers: &[usize], line_index: usize, number_index: usize) -> usize {
    // print!(
    //     "li: {}, ni: {},  {:?}, {:?}: ",
    //     line_index,
    //     number_index,
    //     &line[line_index.min(line.len())..],
    //     &numbers[number_index.min(numbers.len())..]
    // );

    if number_index == numbers.len() {
        // println!("but no more numbers");
        if line_index == line.len() {
            return 1;
        } else if line_index == line.len() + 1 {
            return 1;
        } else if line[line_index..].iter().any(|s| s == &Status::Broken) {
            return 0;
        } else {
            return 1;
        }
    }

    if line_index >= line.len() {
        // println!("but no more space");
        return 0;
    }

    assert!(number_index < numbers.len());

    let numbers_left = numbers.len() - number_index;
    let lines_left = line.len() - line_index;

    let needed = numbers_left * 2 - 1;

    if needed > lines_left {
        // println!("but not enough lines left");
        return 0;
    }

    if line_index >= line.len() {
        // println!("but no more space");
        return 0;
    }
    if line_index > 0 && line[line_index - 1] == Status::Broken {
        // println!("but prev spot broken");
        return step(line, numbers, line_index + 1, number_index);
    }
    if line[line_index] == Status::Operational {
        // println!("but first spot not ok");
        return step(line, numbers, line_index + 1, number_index);
    }

    let amount = numbers[number_index];

    if line_index + amount > line.len() {
        // // println!("but first spot too far: {}, {:?}", amount, line);
        // println!("but first sport too far?");
        return 0;
    }

    let can_put_it_in = can_put_it_in(line, line_index, amount);

    if line[line_index] == Status::Broken {
        if can_put_it_in {
            // println!("has to put in");
            return step(line, numbers, line_index + amount + 1, number_index + 1);
        } else {
            // println!("can't put it in");
            return 0;
        }
    }

    if can_put_it_in {
        // println!("can put it in");

        return step(line, numbers, line_index + amount + 1, number_index + 1)
            + step(line, numbers, line_index + 1, number_index);
    } else {
        // println!("can't put it in");
        let not_placed = step(line, numbers, line_index + 1, number_index);
        return not_placed;
    }
}

fn can_put_it_in(line: &[Status], line_index: usize, amount: usize) -> bool {
    if line_index + amount > line.len() {
        return false;
    }

    let prev_ok = line_index == 0 || line[line_index - 1] != Status::Broken;
    let next_ok = line_index + amount == line.len() || line[line_index + amount] != Status::Broken;

    prev_ok
        && next_ok
        && line[line_index..(line_index + amount)]
            .iter()
            .all(|s| s != &Status::Operational)
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
