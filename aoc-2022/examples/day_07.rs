use std::collections::HashMap;

type InputType = Structure;

pub fn main() {
    let numbers = input();

    println!("Exercise 1: {}", exercise_1(numbers.clone()));
    println!("Exercise 2: {}", exercise_2(numbers)); //70_000_000
}

fn input() -> InputType {
    let mut lines = include_str!("../input/day_07.txt").lines().peekable();

    let mut folder = Structure::default();

    let mut current = &mut folder;
    let mut stack = Vec::new();

    while let Some(line) = lines.next() {
        if line.starts_with("$ cd") {
            let dir = &line[5..];
            match dir {
                "/" => {
                    stack.clear();
                    current = &mut folder;
                }
                ".." => {
                    stack.pop().unwrap();
                    current = stack
                        .iter()
                        .fold(&mut folder, |acc, x| acc.dirs.get_mut(*x).unwrap());
                }
                dir => {
                    stack.push(dir);
                    current = current.dirs.get_mut(dir).unwrap();
                }
            }
        }

        if line == "$ ls" {
            while let Some(x) = lines.peek() {
                if !x.starts_with("$") {
                    let x = lines.next().unwrap();
                    if x.starts_with("dir") {
                        let dir_name = String::from(&x[4..]);
                        current.dirs.entry(dir_name).or_insert(Structure::default());
                    } else {
                        let mut item = x.split(" ");
                        let file_size = item.next().unwrap().parse().unwrap();
                        current.files.push(file_size);
                    }
                } else {
                    break;
                }
            }
        }
    }

    folder
}

fn exercise_1(input: InputType) -> usize {
    let (_, a) = dir_size(&input);

    a
}

fn dir_size(input: &Structure) -> (usize, usize) {
    let sum_files = input.files.iter().sum::<usize>();
    let (sum_dirs, below) = input
        .dirs
        .iter()
        .map(|x| dir_size(x.1))
        .fold((0, 0), |acc, (a, b)| (acc.0 + a, acc.1 + b));
    let sum = sum_files + sum_dirs;
    let mut below = below;

    if sum <= 100_000 {
        below += sum;
    }

    (sum, below)
}

fn exercise_2(input: InputType) -> usize {
    let (a, b) = dir_size_2(&input);
    println!("space needed: {}", a - (70_000_000 - 30_000_000));
    b
}

fn dir_size_2(input: &Structure) -> (usize, usize) {
    let sum_files = input.files.iter().sum::<usize>();
    let (sum_dirs, below) = input
        .dirs
        .iter()
        .map(|x| dir_size_2(x.1))
        .fold((0, usize::MAX), |acc, (a, b)| (acc.0 + a, acc.1.min(b)));
    let sum = sum_files + sum_dirs;
    let mut below = below;

    if sum >= 5349983 {
        below = below.min(sum);
    }

    (sum, below)
}

#[derive(Debug, Clone, Default)]
struct Structure {
    files: Vec<usize>,
    dirs: HashMap<String, Structure>,
}
