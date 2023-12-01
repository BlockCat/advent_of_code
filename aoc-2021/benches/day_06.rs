#![feature(test)]
extern crate test;
use self::test::Bencher;

include!("../examples/day_06.rs");

#[bench]
fn d06_input(b: &mut Bencher) {
    b.iter(|| parse_input(include_str!("../input/day06.txt")));
}

#[bench]
fn d06_ex1_normal(b: &mut Bencher) {
    let input = parse_input(include_str!("../input/day06.txt"));
    b.iter(|| exercise_n::<80>(&input));
}

#[bench]
fn d06_ex2_normal(b: &mut Bencher) {
    let input = parse_input(include_str!("../input/day06.txt"));
    b.iter(|| exercise_n::<256>(&input));
}

#[bench]
fn d06_ex3_normal(b: &mut Bencher) {
    let input = parse_input(include_str!("../input/day06.txt"));
    b.iter(|| exercise_n::<100_000_000>(&input));
}

// Version 1
// test d06_ex1_normal ... bench:      44,315 ns/iter (+/- 385)
// test d06_ex2_normal ... bench:     133,492 ns/iter (+/- 1,350)
// test d06_input      ... bench:       5,539 ns/iter (+/- 794)

// Version 2
// test d06_ex1_normal ... bench:         341 ns/iter (+/- 9)
// test d06_ex2_normal ... bench:         371 ns/iter (+/- 6)
// test d06_input      ... bench:       5,276 ns/iter (+/- 108)