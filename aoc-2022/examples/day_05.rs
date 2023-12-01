use std::collections::VecDeque;

#[derive(Debug, Clone)]
struct InputType {
    stack: Vec<VecDeque<char>>,
    instructions: Vec<(usize, usize, usize)>,
}

pub fn main() {
    let numbers = input();
    println!("Exercise 1: {}", exercise_1(numbers.clone()));
    println!("Exercise 2: {}", exercise_2(numbers));
}

fn input() -> InputType {
    let mut lines = include_str!("../input/day_05.txt").lines();

    let part_1 = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .collect::<Vec<_>>();

    let mut stack = vec![VecDeque::new(); 9];

    let mut part_1 = part_1.into_iter().rev().skip(1);

    while let Some(line) = part_1.next() {
        let mut chars = line.chars();
        let mut count = 0usize;
        while let Some(char) = chars.nth(1) {
            if char != ' ' {
                stack[count].push_back(char);
            }
            chars.nth(1);
            count += 1;
        }
    }

    let instructions = lines
        .map(|x| {
            let mut parts = x.split(' ');
            let mut p = || -> usize { parts.nth(1).unwrap().parse().unwrap() };
            let number = p();
            let from = p() - 1usize;
            let to = p() - 1usize;
            (number, from, to)
        })
        .collect::<Vec<_>>();

    InputType {
        stack,
        instructions,
    }
}

fn exercise_1(input: InputType) -> String {
    let mut stack = input.stack.clone();

    for (count, from, to) in input.instructions {
        let len = stack[from].len();
        let temp = stack[from].split_off(len - count);
        stack[to].extend(temp.into_iter().rev());
    }

    stack.iter_mut().map(|x| x.back().unwrap()).collect()
}
fn exercise_2(input: InputType) -> String {
    let mut stack = input.stack.clone();

    for (count, from, to) in input.instructions {
        let len = stack[from].len();
        let temp = stack[from].split_off(len - count);
        stack[to].extend(temp.into_iter());
    }

    stack.iter_mut().map(|x| x.back().unwrap()).collect()
}
