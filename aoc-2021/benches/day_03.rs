#![feature(test)]
extern crate test;
use self::test::Bencher;

include!("../examples/day_03.rs");

#[bench]
fn d03_input(b: &mut Bencher) {
    b.iter(|| {
        include_str!("../input/day03.txt")
            .lines()
            .map(decode_binary)
            .collect::<Vec<_>>()
    });
}

#[bench]
fn d03_ex1_normal(b: &mut Bencher) {
    let numbers = include_str!("../input/day03.txt")
        .lines()
        .map(decode_binary)
        .collect::<Vec<_>>();
    b.iter(|| exercise_1(&numbers));
}

#[bench]
fn d03_ex2_normal(b: &mut Bencher) {
    let numbers = include_str!("../input/day03.txt")
        .lines()
        .map(decode_binary)
        .collect::<Vec<_>>();
        b.iter(|| exercise_2(&numbers));
}
