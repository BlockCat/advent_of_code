use nalgebra::{SMatrix, SVector};

type Input = Vec<usize>;

pub fn main() {
    let input = parse_input(include_str!("../input/day06.txt"));

    println!("Ex1: {}", exercise_n::<80>(&input));
    println!("Ex2: {}", exercise_n::<256>(&input));
    println!("Ex3: {}", exercise_n::<9000>(&input));
}

fn parse_input(input: &str) -> Input {
    input.split(',').map(|x| x.parse().unwrap()).collect()
}

fn exercise_n<const N: usize>(fishes: &Input) -> f64 {
    let mut group = [0f64; 9];

    for fish in fishes {
        group[*fish] += 1.0;
    }

    let group = SVector::<f64, 9>::from(group);

    let matrix = SMatrix::<f64, 9, 9>::from([
        [0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0],
        [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0],
        [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    ])
    .transpose()
    .pow(N - 1)
    .unwrap();

    let group = matrix * group;

    group.sum()
}

// fn exercise_2(lines: &Input) -> usize {}
