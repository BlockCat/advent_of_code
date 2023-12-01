use crate::test::Bencher;

use packed_simd::*;

type MySimd = i32x8;

#[test]
pub fn run() {
    let input = read_input(include_str!("input/day1.txt"));
    println!("{}", exercise_1(&input));
    println!("{}", exercise_2(&input));
}

pub fn exercise_1(slice: &Vec<i32>) -> i32 {
    let mut iter = slice.chunks_exact(MySimd::lanes());
    let simd = iter
        .by_ref()
        .map(|x| MySimd::from_slice_unaligned(x) / 3 - 2)
        .sum::<MySimd>()
        .wrapping_sum();

    let seq = iter.remainder().into_iter().map(|x| x / 3 - 2).sum::<i32>();
    seq + simd
}

pub fn exercise_2(slice: &Vec<i32>) -> i32 {    
    let mut iter = slice.chunks_exact(MySimd::lanes());
    let simd = iter
        .by_ref()
        .map(|x| calculate_fuel_simd(MySimd::from_slice_unaligned(x)))
        .sum::<MySimd>()
        .wrapping_sum();

    let seq: i32 = iter.remainder().into_iter().map(calculate_fuel).sum();
    seq + simd
}

fn calculate_fuel_simd(mass: MySimd) -> MySimd {
    let zero: MySimd = MySimd::splat(0);
    let nine: MySimd = MySimd::splat(9);
    let three: MySimd = MySimd::splat(3);
    let two: MySimd = MySimd::splat(2);
    let mut fuel = mass;
    let mut sum = zero;    
    
    while fuel.gt(nine).any() {
        fuel = (fuel / three - two).max(zero);
        sum += fuel;
    }
    sum
} 
fn calculate_fuel(mass: &i32) -> i32 {
    let mut fuel = *mass;
    let mut sum = 0;
    while fuel >= 9 {
        fuel = fuel / 3 - 2;
        sum += fuel;
    }
    sum
}

pub fn read_input(input: &str) -> Vec<i32> {
    input.lines().map(|x| x.parse::<i32>().unwrap()).collect()
}

#[test]
fn d1_test() {
    assert_eq!(exercise_1(&vec!(12)), 2);
    assert_eq!(exercise_1(&vec!(14)), 2);
    assert_eq!(exercise_1(&vec!(1969)), 654);
    assert_eq!(exercise_1(&vec!(100756)), 33583);
    assert_eq!(exercise_2(&vec!(12)), 2);
    assert_eq!(exercise_2(&vec!(1969)), 966);
    assert_eq!(exercise_2(&vec!(100756)), 50346);

    let input = read_input(include_str!("input/day1.txt"));
    assert_eq!(3268951, exercise_1(&input));
    assert_eq!(4900568, exercise_2(&input));
}


#[bench]
fn d1_bench_parse(b: &mut Bencher) {    
    b.iter(|| read_input(include_str!("input/day1.txt")));
}
#[bench]
fn d1_bench_ex1(b: &mut Bencher) {
    let input = read_input(include_str!("input/day1.txt"));
    b.iter(|| exercise_1(&input));
}

#[bench]
fn d1_bench_ex2(b: &mut Bencher) {
    let input = read_input(include_str!("input/day1.txt"));
    b.iter(|| exercise_2(&input));
}

/*
mod big_boy {
    use rayon::prelude::*;
    use crate::test::Bencher;
    use ramp::Int;


    #[bench]
    fn d1_bench_ex1_big(b: &mut Bencher) {
        b.iter(|| exercise_1_big());
    }

    #[bench]
    fn d1_bench_ex2_big(b: &mut Bencher) {
        b.iter(|| exercise_2_big());
    }

    pub fn exercise_1_big() -> Int {
        include_str!("input/day1_big.txt")
            .lines()
            .par_bridge()
            .map(|x| x.parse::<Int>().unwrap())
            .map(|x| x / 3i32 - 2i32)
            .reduce(|| Int::from(0), |acc, x| acc + x)
    }

    pub fn exercise_2_big() -> Int {
        include_str!("input/day1_big.txt")
            .lines()
            .par_bridge()
            .map(|x| x.parse::<Int>().unwrap())
            .map(calculate_fuel_big)
            .reduce(|| Int::from(0), |acc, x| acc + x)
    }

    fn calculate_fuel_big(mass: Int) -> Int {
        let mut fuel = mass;
        let mut sum = Int::from(0);

        while {
            fuel = fuel.clone() / 3i32 - 2i32;
            fuel > 0
        } {
            sum += fuel.clone();
        }
        sum
    }
}
*/
