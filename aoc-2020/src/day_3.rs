use crate::test::Bencher;

use packed_simd::*;
use utils::Grid;

type MyType = bool;

// #[test]
pub fn run() {
    let input = read_input(include_str!("input/day3.txt"));
    println!("{}", exercise_1(&input, (3, 1)));
    println!("{}", exercise_2(&input));
}

pub fn read_input(input: &str) -> Grid<MyType> {
    let v = input
        .lines()
        .map(|line| line.chars().map(|x| x == '#').collect())
        .collect::<Vec<_>>();

    Grid::from_vec(v)
}

pub fn exercise_1(grid: &Grid<MyType>, slope: (usize, usize)) -> usize {
    let start = (0usize, 0usize);

    (0..grid.height / slope.1)
        .filter(|i| {
            *grid
                .get((start.0 + slope.0 * i) % grid.width, start.1 + slope.1 * i)
                .unwrap()
        })
        .count()
}

pub fn exercise_2(grid: &Grid<MyType>) -> usize {
    let slopes = vec![(1usize, 1usize), (3, 1), (5, 1), (7, 1), (1, 2)];

    slopes
        .into_iter()
        .map(|slope| exercise_1(grid, slope))
        .product()
}

#[test]
fn d2p1_test() {
    let input = read_input(include_str!("input/day3.txt"));
    assert_eq!(272, exercise_1(&input, (3, 1)));
}

#[test]
fn d2p2_test() {
    let input = read_input(include_str!("input/day3.txt"));
    assert_eq!(3898725600, exercise_2(&input));
}

#[bench]
fn d2_bench_parse(b: &mut Bencher) {
    b.iter(|| read_input(include_str!("input/day3.txt")));
}
#[bench]
fn d2_bench_ex1_complete(b: &mut Bencher) {
    b.iter(|| {
        let input = read_input(include_str!("input/day3.txt"));
        exercise_1(&input, (3, 1))
    });
}

#[bench]
fn d2_bench_ex2_complete(b: &mut Bencher) {
    b.iter(|| {
        let input = read_input(include_str!("input/day3.txt"));
        exercise_2(&input)
    });
}

#[bench]
fn d2_bench_ex1(b: &mut Bencher) {
    let input = read_input(include_str!("input/day3.txt"));
    b.iter(|| exercise_1(&input, (3, 1)));
}

#[bench]
fn d2_bench_ex2(b: &mut Bencher) {
    let input = read_input(include_str!("input/day3.txt"));
    b.iter(|| exercise_2(&input));
}

// #[bench]
// fn d2_bench_parsebigboi(b: &mut Bencher) {
//     b.iter(|| read_input(include_str!("input/day3bigboi.txt")).collect::<Vec<_>>());
// }

// #[bench]
// fn d2_bench_ex1bigboi(b: &mut Bencher) {
//     let input = read_input(include_str!("input/day3bigboi.txt")).collect::<Vec<_>>();
//     b.iter(|| exercise_1(input.clone().into_iter()));
// }

// #[bench]
// fn d2_bench_ex2bigboi(b: &mut Bencher) {
//     let input = read_input(include_str!("input/day3bigboi.txt")).collect::<Vec<_>>();
//     b.iter(|| exercise_2(input.clone().into_iter()));
// }
