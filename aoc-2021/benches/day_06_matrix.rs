#![feature(test)]
extern crate test;
use self::test::Bencher;

include!("../examples/day_06_matrix.rs");

#[bench]
fn d06_input(b: &mut Bencher) {
    b.iter(|| parse_input(include_str!("../input/day06.txt")));
}

#[bench]
fn d06_ex1_matrix(b: &mut Bencher) {
    let input = parse_input(include_str!("../input/day06.txt"));
    b.iter(|| exercise_n::<80>(&input));
}

#[bench]
fn d06_ex2_matrix(b: &mut Bencher) {
    let input = parse_input(include_str!("../input/day06.txt"));
    b.iter(|| exercise_n::<256>(&input));
}

#[bench]
fn d06_ex3_matrix(b: &mut Bencher) {
    let input = parse_input(include_str!("../input/day06.txt"));
    b.iter(|| exercise_n::<100_000_000_000_000>(&input));
}

// Version 1
// test d06_ex1_matrix ... bench:       1,975 ns/iter (+/- 25)
// test d06_ex2_matrix ... bench:       2,504 ns/iter (+/- 374)
// test d06_input      ... bench:       6,191 ns/iter (+/- 495)