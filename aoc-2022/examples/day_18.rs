use aoc_2022::vector::Vector3;
use std::collections::{HashSet, VecDeque};

type InputType = Vec<Vector3>;

pub fn main() {
    let numbers = input();

    println!("Exercise 1: {}", exercise_1(numbers.clone()));
    println!("Exercise 2: {}", exercise_2(numbers));
}

fn input() -> InputType {
    include_str!("../input/day_18.txt")
        .lines()
        .map(parse_line)
        .collect()
}

fn parse_line(line: &str) -> Vector3 {
    let l = line
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect::<Vec<_>>();

    Vector3::new([l[0], l[1], l[2]])
}

fn exercise_1(input: InputType) -> usize {
    let mut max_sides = input.len() * 6;

    for x in input.iter() {
        for y in input.iter() {
            if x == y || x > y {
                continue;
            }

            let len = Vector3::manhattan(x, y);

            if len == 1 {
                max_sides -= 2;
            }
        }
    }

    max_sides
}
fn exercise_2(input: InputType) -> usize {
    let lower = input
        .iter()
        .fold((isize::MAX, isize::MAX, isize::MAX), |(a, b, c), x| {
            (a.min(x[0]), b.min(x[1]), c.min(x[2]))
        });
    let upper = input
        .iter()
        .fold((isize::MIN, isize::MIN, isize::MIN), |(a, b, c), x| {
            (a.max(x[0]), b.max(x[1]), c.max(x[2]))
        });

    let mut counter = 0;

    println!("l: {:?}, u: {:?}", lower, upper);

    let cubes: HashSet<Vector3> = HashSet::from_iter(input.into_iter());

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_front(Vector3::zero());

    while let Some(pos) = queue.pop_front() {
        if !visited.insert(pos) {
            continue;
        }
        if pos[0] > upper.0 + 1 || pos[1] > upper.1 + 1 || pos[2] > upper.2 + 1 {
            continue;
        }
        if pos[0] < lower.0 - 1 || pos[1] < lower.1 - 1 || pos[2] < lower.2 - 1 {
            continue;
        }

        let all_dirs = [
            Vector3::new([1, 0, 0]),
            Vector3::new([-1, 0, 0]),
            Vector3::new([0, 1, 0]),
            Vector3::new([0, -1, 0]),
            Vector3::new([0, 0, 1]),
            Vector3::new([0, 0, -1]),
        ];

        for dir in all_dirs {
            let np = pos + dir;
            if cubes.contains(&np) {
                counter += 1;
            } else {
                queue.push_front(np);
            }
        }
    }

    counter
}
