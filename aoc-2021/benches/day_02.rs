#![feature(test)]
extern crate test;
use self::test::Bencher;

include!("../examples/day_02.rs");

#[bench]
fn input(b: &mut Bencher) {
    b.iter(|| {
        include_str!("../input/day02.txt")
            .lines()
            .map(|x| to_direction(x))
            .collect::<Vec<_>>()
    });
}

#[bench]
fn d02_ex1_normal(b: &mut Bencher) {
    let numbers = include_str!("../input/day02.txt")
        .lines()
        .map(|x| to_direction(x))
        .collect::<Vec<_>>();
    b.iter(|| {
        numbers
            .iter()
            .cloned()
            .reduce(|acc, x| (acc.0 + x.0, acc.1 + x.1))
            .map(|(horizontal, depth)| horizontal * depth)
            .unwrap()
    });
}

#[bench]
fn d02_ex2_normal(b: &mut Bencher) {
    let numbers = include_str!("../input/day02.txt")
        .lines()
        .map(|x| to_direction(x))
        .collect::<Vec<_>>();
    b.iter(|| {
        numbers
            .iter()
            .fold((0, 0, 0), |(hor, depth, aim), (forward, db)| {
                (hor + forward, depth + aim * forward, aim + db)
            })
    });
}
