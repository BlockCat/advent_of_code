#![feature(test)]
extern crate test;
use self::test::Bencher;

include!("../examples/day_04.rs");

#[bench]
fn d04_input(b: &mut Bencher) {
    b.iter(|| parse_input(include_str!("../input/day04.txt")));
}

#[bench]
fn d04_ex1_normal(b: &mut Bencher) {
    let input = parse_input(include_str!("../input/day04.txt"));
    b.iter(|| exercise_1(input.clone()));
}

#[bench]
fn d04_ex2_normal(b: &mut Bencher) {
    let input = parse_input(include_str!("../input/day04.txt"));
    b.iter(|| exercise_2(input.clone()));
}

// Version 1
// test d04_ex1_normal ... bench:     796,260 ns/iter (+/- 101,847)
// test d04_ex2_normal ... bench:   2,573,050 ns/iter (+/- 385,838)
// test d04_input      ... bench:     302,455 ns/iter (+/- 5,978)

// Version 2
// test d04_ex1_normal ... bench:      65,593 ns/iter (+/- 4,572)
// test d04_ex2_normal ... bench:     191,875 ns/iter (+/- 6,657)
// test d04_input      ... bench:     150,784 ns/iter (+/- 22,890)
