#![feature(portable_simd)]
use std::{
    collections::{HashMap, HashSet},
    simd::Simd,
};

type InputType = Vec<Monkey>;

pub fn main() {
    let numbers = input();
    println!("Exercise 1: {}", exercise_1(numbers));

    let numbers = input();
    println!("Exercise 2: {}", exercise_2(numbers));
}

fn input() -> InputType {
    let mut lines = include_str!("../input/day_11.txt").lines();
    let mut monkeys = Vec::new();
    while let Some(_) = lines.next() {
        let mut lines = lines.by_ref().take_while(|x| !x.is_empty());
        let start_items: Vec<usize> = lines.next().unwrap()[18..]
            .split(", ")
            .map(|x| x.parse().unwrap())
            .collect();

        let operation = lines.next().unwrap();
        let divisible: usize = lines.next().unwrap()[21..].parse().unwrap();

        assert!(divisible > 0);

        let true_monkey = lines.next().unwrap()[29..].parse().unwrap();
        let false_monkey = lines.next().unwrap()[30..].parse().unwrap();

        lines.next();

        let operations = {
            let op = &operation[23..24];
            let item = &operation[25..];

            match (item, op) {
                ("old", "+") => MonkeyOperation::AddOld,
                ("old", "*") => MonkeyOperation::MulOld,
                (_, "+") => MonkeyOperation::AddVal(item.parse().unwrap()),
                (_, "*") => MonkeyOperation::MulVal(item.parse().unwrap()),
                _ => unreachable!(),
            }
        };

        monkeys.push(Monkey {
            divisible,
            next_monkey: [true_monkey, false_monkey],
            start_items,
            operations,
        });
    }
    monkeys
}

fn exercise_1(mut input: InputType) -> usize {    
    let mut inspections = (0..20).fold(vec![0; input.len()], |mut acc, _| {
        for monkey in 0..input.len() {
            while let Some(worry) = &input[monkey].start_items.pop() {
                let (next_worry, next_monkey) = inspect(*worry, &input[monkey]);
                input[next_monkey].start_items.push(next_worry as usize);
                acc[monkey] += 1;
            }
        }
        acc
    });

    inspections.sort();
    inspections.into_iter().rev().take(2).product()
}

fn exercise_2(mut input: InputType) -> usize {
    let lessened = input.iter().map(|x| x.divisible).collect::<HashSet<_>>();
    let lessen = lessened.into_iter().product::<usize>();

    let mut inspections = vec![0; input.len()];
    let mut visited = HashMap::with_capacity(1_000);

    let mut counter = 0;
    const MAX: usize = 10_000;

    while counter < MAX {
        monkey_round(&mut input, &mut inspections, lessen as usize);
        counter += 1;

        let hash = input
            .iter()
            .map(|x| x.start_items.clone())
            .collect::<Vec<_>>();

        let grep = visited.insert(hash, (counter, inspections.clone()));

        if let Some((a, old_reps)) = grep {
            let cycle_length = counter - a;
            let repetitions = (MAX - counter) / cycle_length;

            inspections
                .iter_mut()
                .zip(old_reps.iter())
                .for_each(|(rep, old)| *rep += (*rep - *old) * repetitions);

            println!("cycle detected: {} -> {}, [{}]", a, counter, cycle_length);
            counter += repetitions * cycle_length;

            break;
        }
    }

    while counter < MAX {
        counter += 1;
        monkey_round(&mut input, &mut inspections, lessen as usize);
    }

    inspections.sort();
    inspections.into_iter().rev().take(2).product()
}

fn monkey_round(input: &mut InputType, inspections: &mut [usize], lessen: usize) {
    for monkey in 0..input.len() {
        while let Some(worry) = &input[monkey].start_items.pop() {
            inspections[monkey] += 1;
            let (next_worry, next_monkey) = inspect2(*worry, &input[monkey], lessen);
            input[next_monkey].start_items.push(next_worry as usize);
        }
    }
}

fn inspect(worry: usize, monkey: &Monkey) -> (usize, usize) {
    let next_worry = (monkey.operations.execute(worry)) / 3usize;
    let next_monkey = monkey.next_monkey[1 - (next_worry % monkey.divisible == 0) as usize];
    (next_worry, next_monkey)
}

fn inspect2(worry: usize, monkey: &Monkey, lessen: usize) -> (usize, usize) {
    let next_worry = (monkey.operations.execute(worry)) % lessen;
    let next_monkey = monkey.next_monkey[1 - (next_worry % monkey.divisible == 0) as usize];

    (next_worry, next_monkey)
}

struct Monkey {
    start_items: Vec<usize>,
    operations: MonkeyOperation,
    divisible: usize,
    next_monkey: [usize; 2],
}

#[derive(Debug, Clone, Copy)]
enum MonkeyOperation {
    AddOld,
    MulOld,
    AddVal(usize),
    MulVal(usize),
}

impl MonkeyOperation {
    fn execute(self, worry: usize) -> usize {
        match self {
            MonkeyOperation::AddOld => worry + worry,
            MonkeyOperation::MulOld => worry.pow(2),
            MonkeyOperation::AddVal(i) => worry + i,
            MonkeyOperation::MulVal(i) => worry * i,
        }
    }
}
