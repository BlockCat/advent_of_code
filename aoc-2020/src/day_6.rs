use hashbrown::HashSet;
use std::iter::FromIterator;

pub fn run() {
    let input = include_str!("input/day6.txt").trim();
    println!("{}", exercise_1(&input));
    println!("{}", exercise_2(&input));
}

fn exercise_1(input: &str) -> u32 {
    let mut it = input.lines();
    let mut sum = 0;
    while let Some(x) = it.next() {
        // let mut aggregate = 0usize;
        sum += it
            .by_ref()
            .take_while(|x| !x.trim().is_empty())
            .flat_map(|x| x.chars())
            .chain(x.chars())
            .fold(0usize, |acc, x| acc | (1 << (x as usize - 'a' as usize)))
            .count_ones();
    }

    sum
}

fn exercise_2(input: &str) -> u32 {
    let mut it = input.lines();
    let mut sum = 0;
    while let Some(x) = it.next() {
        let initial = chars_to_set(x);
        sum += it
            .by_ref()
            .take_while(|x| !x.trim().is_empty())            
            .map(chars_to_set)
            .fold(initial, |acc, x| acc & x)
            .count_ones();
    }

    sum
}

#[inline(always)]
fn chars_to_set(x: &str) -> u32 {
    x.chars()
        .fold(0, |acc, x| acc | (1 << (x as usize - 'a' as usize)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::Bencher;

    #[test]
    fn d6p1_test() {
        let input = include_str!("input/day6.txt");
        assert_eq!(6612, exercise_1(&input));
    }

    #[test]
    fn d6p2_test() {
        let input = include_str!("input/day6.txt");
        assert_eq!(3268, exercise_2(&input));
    }

    #[bench]
    fn d6_bench_ex1(b: &mut Bencher) {
        let input = include_str!("input/day6.txt");
        b.iter(|| exercise_1(&input));
    }

    #[bench]
    fn d6_bench_ex2(b: &mut Bencher) {
        let input = include_str!("input/day6.txt");
        b.iter(|| exercise_2(&input));
    }

    // // #[bench]
    // // fn d5_bench_ex1bigboi(b: &mut Bencher) {
    // //     let input = include_str!("input/day4bigboi.txt");
    // //     b.iter(|| exercise_1(&input));
    // // }

    // #[bench]
    // fn d5_bench_ex2bigboi(b: &mut Bencher) {
    //     let input = include_str!("input/day5bigboi.txt");
    //     b.iter(|| exercise_2(&input, 18, 7));
    // }
}
