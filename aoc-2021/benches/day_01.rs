#![feature(test)]
extern crate test;
use self::test::Bencher;


include!("../examples/day_01.rs");

#[bench]
fn input(b: &mut Bencher) {
    b.iter(|| {
        include_str!("../input/day01.txt")
            .lines()
            .map(|x| x.parse::<isize>().unwrap())
            .collect::<Vec<_>>()
    });
}

#[bench]
fn ex1_normal(b: &mut Bencher) {
    let numbers: Vec<isize> = include_str!("../input/day01.txt")
        .lines()
        .map(|x| x.parse::<isize>().unwrap())
        .collect();
    b.iter(|| exercise::<2>(&numbers));
}

#[bench]
fn ex2_normal(b: &mut Bencher) {
    let numbers: Vec<isize> = include_str!("../input/day01.txt")
        .lines()
        .map(|x| x.parse::<isize>().unwrap())
        .collect();
    b.iter(|| exercise::<4>(&numbers));
}