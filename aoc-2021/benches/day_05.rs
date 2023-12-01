#![feature(test)]
extern crate test;
use self::test::Bencher;

include!("../examples/day_05.rs");

#[bench]
fn d05_input(b: &mut Bencher) {
    b.iter(|| parse_input(include_str!("../input/day05.txt")));
}

#[bench]
fn d05_ex1_normal(b: &mut Bencher) {
    let input = parse_input(include_str!("../input/day05.txt"));
    b.iter(|| exercise_1(&input));
}

#[bench]
fn d05_ex2_normal(b: &mut Bencher) {
    let input = parse_input(include_str!("../input/day05.txt"));
    b.iter(|| exercise_2(&input));
}

// Version 1
// test d05_ex1_normal ... bench:   8,114,275 ns/iter (+/- 761,541)
// test d05_ex2_normal ... bench:  18,250,660 ns/iter (+/- 1,157,108)
// test d05_input      ... bench:     210,167 ns/iter (+/- 27,644)