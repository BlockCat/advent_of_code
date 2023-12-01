use crate::test::Bencher;

use packed_simd::*;

type MySimd = i32x8;

// #[test]
pub fn run() {
    let input = read_input(include_str!("input/day2.txt"));    
    println!("{}", exercise_1(input));
    let input = read_input(include_str!("input/day2.txt"));
    println!("{}", exercise_2(input));
}

pub fn read_input<'a>(input: &'a str) -> impl Iterator<Item = (u8, u8, char, String)> + 'a {
    input.lines().map(|x| {
        let mut parts = x.split(' ');
        let mut a = parts.next().unwrap().split('-');
        let low = a.next().unwrap().parse().unwrap();
        let high = a.next().unwrap().parse().unwrap();
        let cha = parts
            .next()
            .unwrap()
            .replace(':', "")
            .chars()
            .next()
            .unwrap();
        let string = parts.next().unwrap().to_string();
        (low, high, cha, string)
    })
}

pub fn exercise_1(slice: impl Iterator<Item = (u8, u8, char, String)>) -> usize {
    slice
        .filter(|x| is_valid(x.0, x.1, x.2, x.3.clone()))
        .count()
}

fn is_valid(low: u8, high: u8, ch: char, ww: String) -> bool {
    let c = ww.chars().filter(|&x| x == ch).count() as u8;
    c >= low && c <= high
}

fn is_valid2(low: u8, high: u8, ch: char, ww: String) -> bool {
    // let v = ww.chars().collect::<Vec<_>>();
    // (v[(low - 1) as usize] == ch) ^ (v[(high - 1) as usize] == ch)
    (ww[(low - 1) as usize..].chars().next().unwrap() == ch)
        ^ (ww[(high - 1) as usize..].chars().next().unwrap() == ch)
}

pub fn exercise_2(slice: impl Iterator<Item = (u8, u8, char, String)>) -> usize {
    slice
        .filter(|x| is_valid2(x.0, x.1, x.2, x.3.clone()))
        .count()
}

#[test]
fn d2p1_test() {
    let input = read_input(include_str!("input/day2.txt"));
    assert_eq!(410, exercise_1(input));
}

#[test]
fn d2p2_test() {
    assert_eq!(true, is_valid2(1, 3, 'a', "abcde".to_string()));
    assert_eq!(false, is_valid2(1, 3, 'b', "cdefg".to_string()));
    assert_eq!(false, is_valid2(2, 9, 'c', "ccccccccc".to_string()));
}

#[bench]
fn d2_bench_parse(b: &mut Bencher) {
    b.iter(|| read_input(include_str!("input/day2.txt")).collect::<Vec<_>>());
}
#[bench]
fn d2_bench_ex1_complete(b: &mut Bencher) {
    b.iter(|| {
        let input = read_input(include_str!("input/day2.txt"));
        exercise_1(input)
    });
}

#[bench]
fn d2_bench_ex2_complete(b: &mut Bencher) {
    b.iter(|| {
        let input = read_input(include_str!("input/day2.txt"));
        exercise_2(input)
    });
}

#[bench]
fn d2_bench_ex1(b: &mut Bencher) {
    let input = read_input(include_str!("input/day2.txt")).collect::<Vec<_>>();
    b.iter(|| exercise_1(input.clone().into_iter()));
}

#[bench]
fn d2_bench_ex2(b: &mut Bencher) {
    let input = read_input(include_str!("input/day2.txt")).collect::<Vec<_>>();
    b.iter(|| exercise_2(input.clone().into_iter()));
}

#[bench]
fn d2_bench_parsebigboi(b: &mut Bencher) {
    b.iter(|| read_input(include_str!("input/day2bigboi.txt")).collect::<Vec<_>>());
}

#[bench]
fn d2_bench_ex1bigboi(b: &mut Bencher) {
    let input = read_input(include_str!("input/day2bigboi.txt")).collect::<Vec<_>>();
    b.iter(|| exercise_1(input.clone().into_iter()));
}

#[bench]
fn d2_bench_ex2bigboi(b: &mut Bencher) {
    let input = read_input(include_str!("input/day2bigboi.txt")).collect::<Vec<_>>();
    b.iter(|| exercise_2(input.clone().into_iter()));
}
