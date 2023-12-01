use crate::test::Bencher;

use packed_simd::*;

type MySimd = i32x8;

// #[test]
pub fn run() {
    let input = read_input(include_str!("input/day1.txt"));
    println!("{}", exercise_1(&input, 2020).unwrap());
    println!("{}", exercise_2(&input, 2020));
}

pub fn read_input(input: &str) -> Vec<usize> {
    let mut lines = input
        .lines()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<_>>();
    lines.sort();
    lines
}

pub fn exercise_1(slice: &[usize], target: usize) -> Option<usize> {
    let mut low = 0;
    let mut up = slice.len() - 1;

    while up > low {
        if slice[low] + slice[up] == target {
            return Some(slice[low] * slice[up]);
        } else if slice[low] + slice[up] > target {
            up -= 1;
        } else {
            low += 1;
        }
    }

    return None;
}


pub fn exercise_2(slice: &[usize], target: usize) -> usize {
    
    let mut low = 0;
    let mut up = slice.len() - 1;

    loop {
        if slice[low] + slice[up] + slice[low + 1] > target {
            up -= 1;
        } else {
            let l = slice[low];
            let u = slice[up];

            let mut lower = true;
            for mid in (low + 1)..up {
                let m = slice[mid];
                if l + u + m == target {
                    return l * m * u;
                } else if l + u + m > target {
                    up -= 1;
                    lower = false;
                    break;
                }
            }
            if lower {
                low += 1;
            }
        }
    }
}

pub fn exercise_2b(slice: &[usize], target: usize) -> usize {
    let mut up = slice.len() - 1;
    for i in 0..slice.len() {
        if slice[i] + slice[up] > target {
            up -= 1;
        }
        if let Some(n) = exercise_1(&slice[i..up], target - slice[i]) {
            return n * slice[i];
        }
    }
    unreachable!()
}


#[test]
fn d1p1_test() {
    let input = read_input(include_str!("input/day1.txt"));
    assert_eq!(1020099, exercise_1(&input, 2020).unwrap());
}

#[test]
fn d1p2_test() {
    let input = read_input(include_str!("input/day1.txt"));
    assert_eq!(49214880, exercise_2(&input, 2020));
}

#[bench]
fn d1_bench_parse(b: &mut Bencher) {
    b.iter(|| read_input(include_str!("input/day1.txt")));
}
#[bench]
fn d1_bench_ex1(b: &mut Bencher) {
    let input = read_input(include_str!("input/day1.txt"));
    b.iter(|| exercise_1(&input, 2020));
}
#[bench]
fn d1_bench_ex1bigboi(b: &mut Bencher) {
    let input = read_input(include_str!("input/day1bigboi.txt"));
    b.iter(|| exercise_1(&input, 99920044));
}

#[bench]
fn d1_bench_ex2bigboi(b: &mut Bencher) {
    let input = read_input(include_str!("input/day1bigboi.txt"));
    b.iter(|| exercise_2(&input, 99920044));
}


#[bench]
fn d1_bench_parsebigboi(b: &mut Bencher) {
    b.iter(|| read_input(include_str!("input/day1bigboi.txt")));
}

#[bench]
fn d1_bench_ex2(b: &mut Bencher) {
    let input = read_input(include_str!("input/day1.txt"));
    b.iter(|| exercise_2(&input, 2020));
}


#[bench]
fn d1_bench_ex2b(b: &mut Bencher) {
    let input = read_input(include_str!("input/day1.txt"));
    b.iter(|| exercise_2b(&input, 2020));
}
