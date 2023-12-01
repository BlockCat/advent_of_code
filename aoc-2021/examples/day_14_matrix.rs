use std::collections::HashMap;

use num_bigint::BigUint;
use num_traits::Zero;
use rayon::iter::{ParallelBridge, ParallelIterator};

const SIZE: usize = 100;

type Vector = Vec<BigUint>;
type Matrix = Vec<Vec<BigUint>>;

type Input = (Vector, HashMap<(char, char), usize>, char, Matrix);

pub fn main() {
    let input = parse_input(include_str!("../input/test.txt"));

    let start = std::time::SystemTime::now();
    println!("Ex1: {}", exercise_2(&input, 100_000));
    let end = std::time::SystemTime::now();
    println!("D: {:?}", end.duration_since(start));

    let start = std::time::SystemTime::now();

    println!("Ex2: {}", exercise_2(&input, 1_000_000));

    let end = std::time::SystemTime::now();

    println!("D: {:?}", end.duration_since(start));
}


fn parse_input(input: &str) -> Input {
    let mut lines = input.lines();

    let polymer: Vec<char> = lines.next().unwrap().chars().collect();
    let last_char = polymer.last().unwrap();

    lines.next().unwrap();
    let map = lines
        .map(|line| {
            let mut split = line.split(" -> ");
            let left = split.next().unwrap().to_string();
            let right = split.next().unwrap().to_string();

            let mut left = left.chars();
            let mut right = right.chars();

            (
                (left.next().unwrap(), left.next().unwrap()),
                right.next().unwrap(),
            )
        })
        .collect::<HashMap<_, _>>();

    let pair_index = map
        .iter()
        .enumerate()
        .map(|(index, x)| (*x.0, index))
        .collect::<HashMap<_, _>>();

    println!("size: {}", pair_index.len());

    let mut vector = zero_vec();
    let mut matrix = zero_matrix();

    for (a, b) in map {
        let row = pair_index[&a];
        let col1 = pair_index[&(a.0, b)];
        let col2 = pair_index[&(b, a.1)];

        matrix[row][col1] += BigUint::from(1u32);
        matrix[row][col2] += BigUint::from(1u32);
    }

    for window in polymer.windows(2) {
        let row = pair_index[&(window[0], window[1])];
        vector[row] += BigUint::from(1u32);
    }

    let mut af = pair_index.iter().collect::<Vec<_>>();
    af.sort_by_key(|a| a.1);
    println!("{:?}", af);
    println!("{:?}", vector);

    (vector, pair_index, *last_char, matrix)
}

fn print_mat(matrix: &Vec<Vec<BigUint>>, map: HashMap<usize, (char, char)>) {
    for (i, li) in matrix.iter().enumerate() {
        println!("{:?}: {:?}", map[&i], li);
    }
}

fn exercise_2((polymer, map, last_char, matrix): &Input, steps: usize) -> BigUint {
    let mut matrix = matrix.clone();
    pow(&mut matrix, steps);

    let res = mat_mul_vec(&matrix, polymer);

    let mut a: HashMap<char, BigUint> = map.iter().fold(HashMap::new(), |mut acc, (a, s)| {
        *acc.entry(a.0).or_insert(BigUint::zero()) += &res[*s];
        acc
    });

    *a.entry(*last_char).or_insert(BigUint::zero()) += BigUint::from(1u32);

    let max = a.values().max().unwrap().clone();
    let min = a.values().filter(|a| !a.is_zero()).min().unwrap().clone();

    let pairs = map.iter().map(|(a, b)| (*b, *a)).collect::<HashMap<_, _>>();

    let mut f = (0..SIZE)
        .map(|a| {
            let l = res[a].clone();
            let c = pairs[&a];
            (c, l)
        })
        .collect::<Vec<_>>();
    f.sort_by_key(|a| a.0.clone());

    max - min
}

fn pow(d: &mut Matrix, mut e: usize) -> bool {
    let mut p = d.clone();
    let mut q = identity_mat();

    println!("Start multiplication: {}, {}", e, (e as f64).log2());

    // // Exponentiation by squares.
    while e > 0 {
        println!("cycle multiplication: {}, {}", e, (e as f64).log2());
        if (e % 2) == 1 {
            q = mat_mul_mat(&q, &p);
        }

        e /= 2;
        p = mat_mul_mat(&p, &p);
    }

    println!("End multiplication: {}, {}", e, (e as f64).log2());
    *d = q;
    true
}

fn mat_mul_mat(left: &Matrix, right: &Matrix) -> Matrix {
    let mut new = zero_matrix();
    for i in 0..SIZE {
        for j in 0..SIZE {
            new[i][j] = (0..SIZE)
                .par_bridge()
                .map(|k| &left[i][k] * &right[k][j])
                .sum();
        }
    }
    new
}

fn mat_mul_vec(matrix: &Matrix, vector: &Vector) -> Vector {
    let mut new = zero_vec();
    for row in 0..SIZE {
        for col in 0..SIZE {
            new[row] += &vector[col] * &matrix[col][row];
        }
    }
    new
}

fn zero_matrix() -> Matrix {
    (0..SIZE).map(|_| zero_vec()).collect()
}
fn identity_mat() -> Matrix {
    let mut n = zero_matrix();

    for y in 0..SIZE {
        for x in 0..SIZE {
            n[x][y] = if x == y {
                BigUint::from(1u32)
            } else {
                BigUint::zero()
            };
        }
    }

    n
}

fn zero_vec() -> Vector {
    (0..SIZE).map(|_| BigUint::zero()).collect()
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct BigNum(BigUint);
